//! Builds areas from their text maps: ground, buildings, props, lights, and the walk grid.
//! An area is a street or a room. Rooms sit far away from the street in world space, so both
//! can exist at once and moving between them is a teleport.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    Screen,
    fine_animal::Expression,
    voxel::{VOXEL, VoxelGrid, parse_model, speckle},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Area {
    Street,
    ComputerShop,
    Bedroom,
}

/// What happens when the player reaches a tile.
#[derive(Clone, Debug)]
pub enum Action {
    /// Go through a door into another area, appearing at that area's door.
    Enter(Area),
    /// Leave a room back to the street door it belongs to.
    Exit,
    /// Show a line of text, and make a face at it.
    Say(&'static str, Expression),
}

/// One walkable grid plus what lives on it.
pub struct AreaMap {
    pub origin: Vec3,
    pub size: IVec2,
    pub(crate) walkable: Vec<bool>,
    /// Tiles that do something when reached. Blocked tiles here are used from a neighbour.
    pub actions: HashMap<IVec2, Action>,
    /// Where the player appears when arriving (street: start; room: inside the door).
    pub arrival: IVec2,
    /// For the street: the door tile of each enterable building.
    pub doors: HashMap<Area, IVec2>,
    /// For rooms: the camera zoom that frames the whole room.
    pub zoom: f32,
    /// Ground height of each tile in metres (the road sits a kerb lower than the pavement).
    pub heights: Vec<f32>,
}

impl AreaMap {
    /// A room: no street doors of its own; framed at `zoom`.
    pub fn room(origin: Vec3, size: IVec2, walkable: Vec<bool>, actions: HashMap<IVec2, Action>, arrival: IVec2, zoom: f32) -> Self {
        let heights = vec![0.0; walkable.len()];
        Self { origin, size, walkable, actions, arrival, doors: HashMap::new(), zoom, heights }
    }

    /// Ground height of a tile, in metres.
    pub fn height(&self, t: IVec2) -> f32 {
        if t.x >= 0 && t.y >= 0 && t.x < self.size.x && t.y < self.size.y { self.heights[(t.y * self.size.x + t.x) as usize] } else { 0.0 }
    }

    pub fn walkable(&self, t: IVec2) -> bool {
        t.x >= 0 && t.y >= 0 && t.x < self.size.x && t.y < self.size.y && self.walkable[(t.y * self.size.x + t.x) as usize]
    }

    /// Centre of a tile on the ground, in world space.
    pub fn tile_centre(&self, t: IVec2) -> Vec3 {
        self.origin + Vec3::new(t.x as f32 + 0.5, self.height(t), t.y as f32 + 0.5)
    }

    pub fn tile_at(&self, world: Vec3) -> IVec2 {
        let local = world - self.origin;
        IVec2::new(local.x.floor() as i32, local.z.floor() as i32)
    }

    /// Room bounds centre, for framing the camera.
    pub fn centre(&self) -> Vec3 {
        self.origin + Vec3::new(self.size.x as f32 / 2.0, 0.0, self.size.y as f32 / 2.0)
    }
}

#[derive(Resource)]
pub struct Areas(pub HashMap<Area, AreaMap>);

/// A shape the player can click to use: it stands for the tile whose action runs.
/// Clicking any child part of an entity with this counts too.
#[derive(Component)]
pub struct Clickable {
    pub tile: IVec2,
    /// The area it belongs to: areas can be in view of each other (the street around the
    /// bedroom), and only the current one's things respond.
    pub area: Area,
}

/// Shown only while the player is in this area (a room's own things, a door's gem).
#[derive(Component)]
pub struct OnlyIn(pub Area);

/// Hidden while the player is in this area (the house's upper floor and roof while you are
/// in your bedroom, which sits inside it as a cutaway).
#[derive(Component)]
pub struct HiddenIn(pub Area);

/// A bobbing gem over a door the player can go through.
#[derive(Component)]
pub struct EntryMarker {
    height: f32,
}

/// Lamps; the clock turns them on at night, up to `strength` lumens.
#[derive(Component)]
pub struct Lamp {
    pub strength: f32,
}

/// Window glass: its tint follows the time of day (sky by day, deep blue at night).
#[derive(Resource)]
pub struct WindowGlass(pub Handle<StandardMaterial>);

/// Building window glass; the clock lights it from inside at night.
#[derive(Resource)]
pub struct WindowLight(pub Handle<StandardMaterial>);

/// The glowing material of lamp heads; the clock sets its brightness.
#[derive(Resource)]
pub struct LampGlow(pub Handle<StandardMaterial>);

/// The shared voxel material: white base so vertex colours show as painted, fully matte.
#[derive(Resource)]
pub struct VoxelMaterial(pub Handle<StandardMaterial>);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Game), build_areas)
            .add_systems(Update, cutaway.run_if(in_state(Screen::Game)))
            .add_systems(Update, memlog_frames.run_if(in_state(Screen::Game)))
            .add_systems(Update, bob_markers.run_if(in_state(Screen::Game)));
    }
}

/// Shows each area's own things only there, and opens the house up while you are inside.
fn cutaway(
    here: Option<Res<crate::gameplay::Here>>,
    mut only: Query<(&OnlyIn, &mut Visibility), Without<HiddenIn>>,
    mut hidden: Query<(&HiddenIn, &mut Visibility), Without<OnlyIn>>,
) {
    let Some(here) = here else { return };
    if !here.is_changed() {
        return;
    }
    for (area, mut visibility) in &mut only {
        *visibility = if area.0 == here.0 { Visibility::Inherited } else { Visibility::Hidden };
    }
    for (area, mut visibility) in &mut hidden {
        *visibility = if area.0 == here.0 { Visibility::Hidden } else { Visibility::Inherited };
    }
}

fn bob_markers(time: Res<Time>, mut markers: Query<(&EntryMarker, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (marker, mut transform) in &mut markers {
        transform.translation.y = marker.height + (t * 3.0).sin() * 0.08;
    }
}

/// An upside-down stepped pyramid in warm gold, drawn unlit so it reads day and night.
fn marker_mesh() -> Mesh {
    let mut g = VoxelGrid::new(5, 3, 5);
    g.set(2, 0, 2, Some(c(0xE0, 0xA0, 0x30)));
    g.fill(IVec3::new(1, 1, 1), IVec3::new(3, 1, 3), c(0xF2, 0xC1, 0x4E));
    g.fill(IVec3::new(0, 2, 0), IVec3::new(4, 2, 4), c(0xF8, 0xDC, 0x7A));
    g.mesh(Vec3::new(2.5, 0.0, 2.5))
}

/// The gem for fine-detail rooms, built from fine voxels so its edges land on whole pixels:
/// a point below, widening to a crown, darker toward the point.
fn fine_marker_mesh() -> Mesh {
    let mut g = VoxelGrid::new(9, 6, 9);
    for (y, half, colour) in [(0, 0, 0xC88A28), (1, 1, 0xE0A030), (2, 2, 0xF2C14E), (3, 3, 0xF2C14E), (4, 4, 0xF8DC7A), (5, 3, 0xFCEBA8)] {
        g.fill(IVec3::new(4 - half, y, 4 - half), IVec3::new(4 + half, y, 4 + half), Color::srgb_u8((colour >> 16) as u8, (colour >> 8) as u8, colour as u8));
    }
    g.mesh_sized(Vec3::new(4.5, 0.0, 4.5), crate::voxel::FINE)
}

fn spawn_marker(commands: &mut Commands, meshes: &mut Assets<Mesh>, material: &Handle<StandardMaterial>, at: Vec3, fine: bool) -> Entity {
    commands.spawn((
        Mesh3d(meshes.add(if fine { fine_marker_mesh() } else { marker_mesh() })),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(at),
        EntryMarker { height: at.y },
        DespawnOnExit(Screen::Game),
    ))
    .id()
}

const SHOP: &str = include_str!("../assets/maps/computer_shop.txt");
const CRT: &str = include_str!("../assets/models/crt_computer.txt");

/// Rooms live this far east of the street.
const ROOM_OFFSET: Vec3 = Vec3::new(1000.0, 0.0, 0.0);

pub(crate) fn rows(text: &str) -> Vec<Vec<char>> {
    text.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).map(|l| l.chars().collect()).collect()
}

pub(crate) fn c(r: u8, g: u8, b: u8) -> Color {
    Color::srgb_u8(r, g, b)
}

/// `DEVLIFE_MEMLOG=1`: log this process's memory footprint (as macOS counts it, GPU memory
/// included) at a named point. For tracking down memory blow-ups; off by default.
pub fn memlog(stage: &str) {
    if std::env::var_os("DEVLIFE_MEMLOG").is_none() {
        return;
    }
    let footprint = crate::memguard::footprint_mb().map_or("?".into(), |mb| format!("{mb} MB"));
    info!("memlog: {stage}: {footprint}");
}

/// `DEVLIFE_MEMLOG=1`: memory over the first frames, while meshes and textures reach the GPU,
/// and on the first frame how much mesh data the world holds.
pub fn memlog_frames(mut frame: Local<u32>, meshes: Res<Assets<Mesh>>) {
    *frame += 1;
    if *frame == 1 && std::env::var_os("DEVLIFE_MEMLOG").is_some() {
        let (mut vertices, mut indices, mut biggest) = (0usize, 0usize, 0usize);
        // Only meshes still holding their data; ones already on the GPU have none here.
        for (_, mesh) in meshes.iter().filter(|(_, m)| m.try_attribute(Mesh::ATTRIBUTE_POSITION).is_ok()) {
            let v = mesh.count_vertices();
            vertices += v;
            indices += mesh.indices().map_or(0, |i| i.len());
            biggest = biggest.max(v);
        }
        info!("memlog: meshes {} with {vertices} vertices ({} MB of vertex data), {indices} indices, biggest {biggest} vertices", meshes.len(), vertices * 40 / 1_000_000);
    }
    if matches!(*frame, 1 | 2 | 5 | 10 | 30 | 60 | 120) {
        memlog(&format!("frame {}", *frame));
    }
}

pub fn build_areas(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    });
    commands.insert_resource(VoxelMaterial(material.clone()));
    // Glowing parts show their own painted colour, brightened past white at night so they
    // glow (an orange sodium lamp, an amber beacon). Unlit: light doesn't shade a lamp.
    let glow = materials.add(StandardMaterial { base_color: Color::WHITE, unlit: true, ..default() });
    commands.insert_resource(LampGlow(glow.clone()));
    let glass = materials.add(StandardMaterial { base_color: Color::WHITE, unlit: true, ..default() });
    commands.insert_resource(WindowGlass(glass.clone()));
    // Building windows: glass by day, warm light from inside at night (the clock sets it).
    let window_light = materials.add(StandardMaterial { base_color: Color::WHITE, perceptual_roughness: 0.3, ..default() });
    commands.insert_resource(WindowLight(window_light.clone()));

    let mut areas = HashMap::new();
    memlog("before street");
    areas.insert(Area::Street, crate::street::build(&mut commands, &mut meshes, &material, &glow, &window_light));
    memlog("after street");
    areas.insert(Area::ComputerShop, build_shop(&mut commands, &mut meshes, &material));
    memlog("after shop");
    // Your bedroom is upstairs in the house on the street, shown as a cutaway when you are in it.
    let bedroom_at = crate::street::house_corner() + crate::bedroom::IN_HOUSE;
    areas.insert(Area::Bedroom, crate::bedroom::build(&mut commands, &mut meshes, &mut materials, &material, &glow, &glass, bedroom_at));
    memlog("after bedroom");

    // Entry markers over every door that leads somewhere.
    // Lit, so the gem's faces shade like everything else, with a glow so it still stands out.
    let marker = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::rgb(0.35, 0.26, 0.08),
        perceptual_roughness: 0.6,
        ..default()
    });
    for (area, map) in &areas {
        for (tile, action) in &map.actions {
            // Fine-detail rooms (zoomed in further) get a smaller gem, closer to the door top.
            let fine = *area == Area::Bedroom;
            let at = match (area, action) {
                // Street doors: above the door, just in front of the south wall.
                (Area::Street, Action::Enter(_)) => map.tile_centre(*tile) + Vec3::new(0.0, 3.4, -0.35),
                // Room exits: in front of the west-wall door.
                (_, Action::Exit) if fine => map.tile_centre(*tile) + Vec3::new(-0.3, 2.35, 0.0),
                (_, Action::Exit) => map.tile_centre(*tile) + Vec3::new(-0.2, 2.7, 0.0),
                _ => continue,
            };
            let gem = spawn_marker(&mut commands, &mut meshes, &marker, at, fine);
            commands.entity(gem).insert(OnlyIn(*area));
        }
    }
    commands.insert_resource(Areas(areas));
}

/// Spawns a voxel mesh at `at`, turned by `yaw` radians, cleaned up when the game ends.
pub(crate) fn place(commands: &mut Commands, meshes: &mut Assets<Mesh>, material: &Handle<StandardMaterial>, mesh: Mesh, at: Vec3, yaw: f32) -> Entity {
    place_shared(commands, meshes.add(mesh), material, at, yaw)
}

/// Like [`place`], with a mesh that may be shown in several places.
pub(crate) fn place_shared(commands: &mut Commands, mesh: Handle<Mesh>, material: &Handle<StandardMaterial>, at: Vec3, yaw: f32) -> Entity {
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(at).with_rotation(Quat::from_rotation_y(yaw)),
            DespawnOnExit(Screen::Game),
        ))
        .id()
}

#[derive(Clone, Copy)]
pub(crate) enum Building {
    ComputerShop,
    Flats,
}

/// A building on a footprint of `tiles`. Windows on every side; the door (if any) on the
/// south face, `door` tiles from the west edge.
pub(crate) fn building(style: Building, tiles: UVec2, door: Option<i32>) -> VoxelGrid {
    let (floors, wall, trim, roof): (i32, [Color; 2], Color, Color) = match style {
        Building::ComputerShop => (2, [c(0xB8, 0x6A, 0x50), c(0xAE, 0x62, 0x4A)], c(0xE8, 0xDC, 0xC4), c(0x5C, 0x56, 0x58)),
        Building::Flats => (4, [c(0xA8, 0xAE, 0xB4), c(0xA0, 0xA6, 0xAC)], c(0xD8, 0xD8, 0xD2), c(0x5A, 0x5E, 0x64)),
    };
    let glass = c(0x4E, 0x6E, 0x86);
    let frame = trim;
    let (w, d) = (tiles.x as i32 * 8, tiles.y as i32 * 8);
    let floor_h = 24; // 3 m
    let h = floors * floor_h + 2;
    let mut g = VoxelGrid::new(w as u32, h as u32 + 1, d as u32);

    // Walls with a speckle, a darker plinth, and a trim band between floors.
    g.fill_with(IVec3::ZERO, IVec3::new(w - 1, h - 1, d - 1), |p| {
        Some(if p.y < 2 {
            c(0x6A, 0x62, 0x5C)
        } else if p.y % floor_h == 0 {
            trim
        } else {
            speckle(p, 7, &wall)
        })
    });
    // Parapet on the roof edge, roof surface one voxel lower inside it.
    g.fill(IVec3::new(0, h, 0), IVec3::new(w - 1, h, d - 1), trim);
    g.clear(IVec3::new(1, h, 1), IVec3::new(w - 2, h, d - 2));
    g.fill(IVec3::new(1, h - 1, 1), IVec3::new(w - 2, h - 1, d - 2), roof);

    // Windows: 1 m wide, 1.25 m tall, every 2 m along each face, on every floor.
    let window = |g: &mut VoxelGrid, along_x: bool, fixed: i32, inward: i32, start: i32, y0: i32| {
        for i in 0..8 {
            for y in 0..10 {
                let edge = i == 0 || i == 7 || y == 0 || y == 9;
                let (x, z) = if along_x { (start + i, fixed) } else { (fixed, start + i) };
                g.set(x, y0 + y, z, Some(if edge { frame } else { glass }));
                // Recess the glass one voxel.
                if !edge {
                    let (x2, z2) = if along_x { (x, fixed + inward) } else { (fixed + inward, z) };
                    g.set(x, y0 + y, z, None);
                    g.set(x2, y0 + y, z2, Some(glass));
                }
            }
        }
    };
    for floor in 0..floors {
        let y0 = floor * floor_h + 8;
        for start in (4..w - 8).step_by(16) {
            let is_door_column = floor == 0 && door.is_some_and(|dx| (start..start + 8).contains(&(dx * 8 + 4)));
            if !is_door_column {
                window(&mut g, true, d - 1, -1, start, y0);
            }
            window(&mut g, true, 0, 1, start, y0);
        }
        for start in (4..d - 8).step_by(16) {
            window(&mut g, false, 0, 1, start, y0);
            window(&mut g, false, w - 1, -1, start, y0);
        }
    }

    // Door on the south face: frame, recessed panel, a handle.
    if let Some(dx) = door {
        let x0 = dx * 8;
        g.fill(IVec3::new(x0, 2, d - 1), IVec3::new(x0 + 7, 19, d - 1), frame);
        g.clear(IVec3::new(x0 + 1, 2, d - 1), IVec3::new(x0 + 6, 18, d - 1));
        g.fill(IVec3::new(x0 + 1, 2, d - 2), IVec3::new(x0 + 6, 18, d - 2), c(0x5A, 0x3E, 0x2E));
        g.set(x0 + 5, 10, d - 2, Some(c(0xE8, 0xC8, 0x6A)));
        if let Building::ComputerShop = style {
            // Shop sign above the door: a teal board with a pixel "chip" icon.
            let sign = c(0x2E, 0x7E, 0x7A);
            g.fill(IVec3::new(x0 - 8, 20, d - 1), IVec3::new(x0 + 15, 23, d - 1), sign);
            for (i, x) in (x0 - 6..x0 + 14).step_by(3).enumerate() {
                g.set(x, 21 + (i as i32 % 2), d - 1, Some(c(0xF2, 0xE8, 0xC8)));
            }
            // Awning sticking out over the door.
            if d < 255 {
                g.fill(IVec3::new(x0 - 2, 19, d - 1), IVec3::new(x0 + 9, 19, d - 1), c(0xD8, 0x5A, 0x48));
            }
        }
    }
    g
}



pub(crate) fn tree(seed: u32) -> Mesh {
    let mut g = VoxelGrid::new(14, 30, 14);
    let leaves = [c(0x4E, 0x8A, 0x44), c(0x46, 0x80, 0x3E), c(0x58, 0x96, 0x4C)];
    g.fill(IVec3::new(6, 0, 6), IVec3::new(7, 12, 7), c(0x6A, 0x4A, 0x32));
    // A lumpy crown: a sphere with a few voxels bitten out by the hash.
    let centre = Vec3::new(6.5, 19.0, 6.5);
    g.fill_with(IVec3::new(0, 10, 0), IVec3::new(13, 29, 13), |p| {
        let d = (p.as_vec3() - centre) * Vec3::new(1.0, 1.25, 1.0);
        let bite = crate::voxel::hash(p, seed) % 7 == 0;
        if d.length() < 7.0 && !bite { Some(speckle(p, seed, &leaves)) } else { None }
    });
    // Trunk stays visible under the crown.
    g.fill(IVec3::new(6, 0, 6), IVec3::new(7, 12, 7), c(0x6A, 0x4A, 0x32));
    g.mesh_standing()
}


fn build_shop(commands: &mut Commands, meshes: &mut Assets<Mesh>, material: &Handle<StandardMaterial>) -> AreaMap {
    let map = rows(SHOP);
    let size = IVec2::new(map[0].len() as i32, map.len() as i32);
    let origin = ROOM_OFFSET;
    let tile = |x: i32, z: i32| map.get(z as usize).and_then(|r| r.get(x as usize)).copied().unwrap_or('.');

    // Floor: chequered lino. Walls: north and west only, 3 m tall, with a skirting board.
    let mut floor = VoxelGrid::new(size.x as u32 * 8, 1, size.y as u32 * 8);
    floor.fill_with(IVec3::ZERO, IVec3::new(size.x * 8 - 1, 0, size.y * 8 - 1), |p| {
        Some(if ((p.x / 4) + (p.z / 4)) % 2 == 0 { c(0xD6, 0xCC, 0xB4) } else { c(0xB4, 0xA8, 0x8E) })
    });
    place(commands, meshes, material, floor.mesh(Vec3::new(0.0, 1.0, 0.0)), origin, 0.0);

    let wall_paint = [c(0xA8, 0xC4, 0xB0), c(0xA2, 0xBE, 0xAA)];
    let (w, d) = (size.x * 8, size.y * 8);
    let mut walls = VoxelGrid::new(w as u32 + 1, 24, d as u32 + 1);
    // North wall along grid z = 0 (behind the first row), west wall along x = 0.
    walls.fill_with(IVec3::new(0, 0, 0), IVec3::new(w, 23, 0), |p| Some(if p.y < 2 { c(0x6E, 0x5A, 0x48) } else { speckle(p, 9, &wall_paint) }));
    walls.fill_with(IVec3::new(0, 0, 0), IVec3::new(0, 23, d), |p| Some(if p.y < 2 { c(0x6E, 0x5A, 0x48) } else { speckle(p, 9, &wall_paint) }));
    // A poster on the north wall: a game advert in two colours.
    walls.fill(IVec3::new(54, 8, 0), IVec3::new(63, 19, 0), c(0x2C, 0x3A, 0x6A));
    walls.fill(IVec3::new(56, 10, 0), IVec3::new(61, 13, 0), c(0xE8, 0x7A, 0x3A));
    walls.fill(IVec3::new(56, 15, 0), IVec3::new(61, 17, 0), c(0xF2, 0xE8, 0xC8));
    // Door in the west wall: the opening shows the street light behind it.
    // The door is its own shape (same grid layout) so it can be clicked on its own.
    let door_z = (0..size.y).find(|&z| tile(0, z) == 'D').unwrap_or(0);
    let mut door = VoxelGrid::new(w as u32 + 1, 24, d as u32 + 1);
    door.fill(IVec3::new(0, 0, door_z * 8 + 1), IVec3::new(0, 18, door_z * 8 + 8), c(0xE8, 0xDC, 0xC4));
    door.fill(IVec3::new(0, 0, door_z * 8 + 2), IVec3::new(0, 17, door_z * 8 + 7), c(0x5A, 0x3E, 0x2E));
    door.set(0, 9, door_z * 8 + 6, Some(c(0xE8, 0xC8, 0x6A)));
    walls.clear(IVec3::new(0, 0, door_z * 8 + 1), IVec3::new(0, 18, door_z * 8 + 8));
    // The grid's x = 0, z = 0 corner sits one voxel outside the floor.
    place(commands, meshes, material, walls.mesh(Vec3::new(1.0, 0.0, 1.0)), origin, 0.0);
    let door = place(commands, meshes, material, door.mesh(Vec3::new(1.0, 0.0, 1.0)), origin, 0.0);
    commands.entity(door).insert(Clickable { tile: IVec2::new(0, door_z), area: Area::ComputerShop });

    let crt = parse_model(CRT);
    let mut walkable = vec![false; (size.x * size.y) as usize];
    let mut actions = HashMap::new();
    let mut arrival = IVec2::ZERO;
    for z in 0..size.y {
        for x in 0..size.x {
            let t = IVec2::new(x, z);
            let ch = tile(x, z);
            let centre = origin + Vec3::new(x as f32 + 0.5, 0.0, z as f32 + 0.5);
            walkable[(z * size.x + x) as usize] = matches!(ch, '.' | 'D' | '@');
            match ch {
                '@' => arrival = t,
                'D' => {
                    actions.insert(t, Action::Exit);
                }
                'S' => {
                    // Shelves stand against the north wall with their open side to the room.
                    let e = place(commands, meshes, material, shelf(x as u32), centre, std::f32::consts::PI);
                    commands.entity(e).insert(Clickable { tile: t, area: Area::ComputerShop });
                    actions.insert(t, Action::Say("Boxed games in zip-lock bags. The price tags are more than you have.", Expression::Melancholy));
                }
                'C' => {
                    let e = place(commands, meshes, material, table(), centre, 0.0);
                    commands.entity(e).insert(Clickable { tile: t, area: Area::ComputerShop });
                    // The computer faces south, toward the room; its front is -Z, so turn it around.
                    let e = place(commands, meshes, material, crt.mesh_standing(), centre + Vec3::new(0.0, 6.0 * VOXEL, 0.0), std::f32::consts::PI);
                    commands.entity(e).insert(Clickable { tile: t, area: Area::ComputerShop });
                    actions.insert(t, Action::Say("A demo machine, blinking READY. The owner watches you like a hawk.", Expression::Surprised));
                }
                'K' => {
                    let e = place(commands, meshes, material, counter(), centre, 0.0);
                    commands.entity(e).insert(Clickable { tile: t, area: Area::ComputerShop });
                    actions.insert(t, Action::Say("The counter. A till, a bell, a stack of computer magazines.", Expression::Neutral));
                }
                'k' => {
                    actions.insert(t, Action::Say("\"Browsing is free. Touching is not.\"", Expression::Puzzled));
                }
                'P' => {
                    place(commands, meshes, material, plant(), centre, 0.0);
                }
                _ => {}
            }
        }
    }
    // The shop's own light, so it stays cosy at night.
    commands.spawn((
        PointLight { intensity: 60_000.0, range: 14.0, color: c(0xFF, 0xE6, 0xC0), shadow_maps_enabled: true, ..default() },
        Transform::from_translation(origin + Vec3::new(size.x as f32 / 2.0, 3.2, size.y as f32 / 2.0)),
        DespawnOnExit(Screen::Game),
    ));

    AreaMap::room(origin, size, walkable, actions, arrival, 1.0)
}

fn shelf(seed: u32) -> Mesh {
    let mut g = VoxelGrid::new(8, 16, 4);
    let wood = c(0x8A, 0x60, 0x40);
    g.fill(IVec3::ZERO, IVec3::new(7, 15, 3), wood);
    g.clear(IVec3::new(1, 1, 0), IVec3::new(6, 14, 2));
    // Boxes on three shelves, colours by position.
    let boxes = [c(0xC8, 0x4B, 0x4B), c(0x3E, 0x6E, 0xA8), c(0xE6, 0xC0, 0x4E), c(0x5A, 0x8F, 0x5A), c(0xEE, 0xEE, 0xE8)];
    for level in 0..3 {
        let y = 1 + level * 5;
        g.fill(IVec3::new(1, y + 4, 0), IVec3::new(6, y + 4, 2), wood);
        for x in 1..7 {
            if crate::voxel::hash(IVec3::new(x, y, seed as i32), seed) % 4 != 0 {
                let top = y + 1 + (crate::voxel::hash(IVec3::new(x, y, 1), seed) % 3) as i32;
                g.fill(IVec3::new(x, y, 1), IVec3::new(x, top, 2), speckle(IVec3::new(x, y, 0), seed, &boxes));
            }
        }
    }
    g.mesh_standing()
}

fn table() -> Mesh {
    let mut g = VoxelGrid::new(8, 6, 6);
    let wood = c(0x9A, 0x74, 0x52);
    g.fill(IVec3::new(0, 5, 0), IVec3::new(7, 5, 5), wood);
    for (x, z) in [(0, 0), (7, 0), (0, 5), (7, 5)] {
        g.fill(IVec3::new(x, 0, z), IVec3::new(x, 4, z), c(0x6A, 0x50, 0x3A));
    }
    g.mesh_standing()
}

fn counter() -> Mesh {
    let mut g = VoxelGrid::new(8, 8, 8);
    g.fill(IVec3::ZERO, IVec3::new(7, 6, 7), c(0x7A, 0x56, 0x3C));
    g.fill(IVec3::new(0, 7, 0), IVec3::new(7, 7, 7), c(0xD6, 0xC4, 0xA0));
    g.mesh_standing()
}

fn plant() -> Mesh {
    let mut g = VoxelGrid::new(6, 12, 6);
    g.fill(IVec3::new(1, 0, 1), IVec3::new(4, 3, 4), c(0xB8, 0x6A, 0x50));
    let leaves = [c(0x4E, 0x8A, 0x44), c(0x5E, 0x9A, 0x50)];
    g.fill_with(IVec3::new(0, 4, 0), IVec3::new(5, 11, 5), |p| {
        let d = (p.as_vec3() - Vec3::new(2.5, 7.5, 2.5)).length();
        (d < 3.2 && crate::voxel::hash(p, 5) % 4 != 0).then(|| speckle(p, 5, &leaves))
    });
    g.mesh_standing()
}
