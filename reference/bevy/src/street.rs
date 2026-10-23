//! The main street, 1978 (`assets/maps/street.txt`): pavements, parking lanes and two
//! traffic lanes a kerb lower, a zebra crossing, your house with its front garden, the
//! computer shop and the flats, street furniture and parked cars.
//!
//! The ground and building shells are coarse voxels (8 to a 1 m tile); street furniture,
//! cars and small parts are fine voxels (24 to the metre) from `street_props.rs`.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    Screen,
    fine_animal::Expression,
    house::{HOME, HouseStyle, NEIGHBOURS, UPSTAIRS, house},
    street_props::{Prop, catalogue},
    voxel::{FINE, MeshCache, VOXEL, VoxelGrid, hash, speckle},
    world::{Action, Area, AreaMap, Building, Clickable, HiddenIn, Lamp, OnlyIn, building, c, place, place_shared, rows, tree},
};

const MAP: &str = include_str!("../assets/maps/street.txt");
/// Grass and street around the map, outside the walkable area, so the town never ends at
/// the screen edge.
const BORDER: i32 = 18;
/// How far the road sits below the pavement: one coarse voxel (12.5 cm).
const KERB: f32 = VOXEL;

/// What a tile is underneath whatever stands on it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Ground {
    Grass,
    Lawn,
    FlowerBed,
    Path,
    Pavement,
    Parking,
    Lane,
    Crossing,
}

impl Ground {
    fn of(ch: char) -> Ground {
        match ch {
            '-' | 'L' | 'Z' | 'P' | 'K' | 'S' | 'B' | 'Y' | 'N' | 'U' | 'D' | '@' => Ground::Pavement,
            'p' | 'M' | 'm' => Ground::Parking,
            '=' => Ground::Lane,
            'z' => Ground::Crossing,
            'g' | 'w' => Ground::Lawn,
            'f' => Ground::FlowerBed,
            'o' => Ground::Path,
            _ => Ground::Grass,
        }
    }

    fn is_road(self) -> bool {
        matches!(self, Ground::Parking | Ground::Lane | Ground::Crossing)
    }
}

pub fn build(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    glow: &Handle<StandardMaterial>,
    window_light: &Handle<StandardMaterial>,
) -> AreaMap {
    let map = rows(MAP);
    let size = IVec2::new(map[0].len() as i32, map.len() as i32);
    let mut cache = MeshCache::default();
    let tile = |x: i32, z: i32| map.get(z as usize).and_then(|r| r.get(x as usize)).copied().unwrap_or('.');
    // Outside the map, street rows run on east and west; everything else is grass.
    let ground_at = |x: i32, z: i32| -> Ground {
        // Back gardens behind the houses (ours and the next street's), with a flower bed
        // along the bottom of ours.
        if (-11..=-1).contains(&z) && (-17..=56).contains(&x) {
            return if z == -1 && (15..=20).contains(&x) { Ground::FlowerBed } else { Ground::Lawn };
        }
        if !(0..size.y).contains(&z) {
            return Ground::Grass;
        }
        if (0..size.x).contains(&x) {
            return Ground::of(tile(x, z));
        }
        match Ground::of(tile(x.clamp(0, size.x - 1), z)) {
            g @ (Ground::Pavement | Ground::Parking | Ground::Lane) => g,
            Ground::Crossing => Ground::Lane,
            _ => Ground::Grass,
        }
    };

    place(commands, meshes, material, ground(size, &ground_at).mesh(Vec3::new(BORDER as f32 * 8.0, 2.0, BORDER as f32 * 8.0)), Vec3::ZERO, 0.0);
    for z in -BORDER..size.y + BORDER {
        for x in -BORDER..size.x + BORDER {
            if ground_at(x, z) == Ground::FlowerBed {
                place(commands, meshes, material, flower_bed(hash(IVec3::new(x, 0, z), 23)).mesh_sized(Vec3::ZERO, FINE), Vec3::new(x as f32, 0.0, z as f32), 0.0);
            }
        }
    }
    // Trees out in the border grass.
    for i in 0..48u32 {
        let h = hash(IVec3::new(i as i32, 0, 0), 99);
        let x = (h % (size.x as u32 + 2 * BORDER as u32)) as i32 - BORDER;
        let z = ((h / 97) % (size.y as u32 + 2 * BORDER as u32)) as i32 - BORDER;
        let inside = (0..size.x).contains(&x) && (0..size.y).contains(&z);
        if !inside && ground_at(x, z) == Ground::Grass {
            place(commands, meshes, material, tree(h), Vec3::new(x as f32 + 0.5, 0.0, z as f32 + 0.5), 0.0);
        }
    }

    crate::world::memlog("street: ground and trees");
    let props: HashMap<&'static str, Prop> = catalogue().into_iter().map(|p| (p.name, p)).collect();
    crate::world::memlog("street: catalogue built");
    let find = |key: &str| props.iter().filter(|(name, _)| name.contains(key)).map(|(_, p)| p).min_by_key(|p| p.name);
    let cars: Vec<&Prop> = {
        let mut cars: Vec<&Prop> = props.values().filter(|p| ["hatchback", "saloon", "van", "car"].iter().any(|k| p.name.contains(k))).collect();
        cars.sort_by_key(|p| p.name);
        cars
    };

    let mut walkable = vec![false; (size.x * size.y) as usize];
    let mut heights = vec![0.0; (size.x * size.y) as usize];
    let mut actions = HashMap::new();
    let mut doors = HashMap::new();
    let mut arrival = IVec2::ZERO;
    let mut footprints: HashMap<char, (IVec2, IVec2)> = HashMap::new();
    let mut car_index = 0;

    for z in 0..size.y {
        for x in 0..size.x {
            let t = IVec2::new(x, z);
            let ch = tile(x, z);
            let i = (z * size.x + x) as usize;
            let ground = Ground::of(ch);
            heights[i] = if ground.is_road() { -KERB } else { 0.0 };
            let centre = Vec3::new(x as f32 + 0.5, heights[i], z as f32 + 0.5);
            walkable[i] = matches!(ch, '.' | '-' | 'p' | '=' | 'z' | 'g' | 'o' | 'D' | '@');
            // Things on the south pavement face north, toward the road.
            let facing_road = if z > size.y / 2 { std::f32::consts::PI } else { 0.0 };
            let key = match ch {
                'L' => Some("lamp"),
                'Z' => Some("beacon"),
                'P' => Some("post"),
                'K' => Some("phone"),
                'S' => Some("bus"),
                'B' => Some("bench"),
                'Y' => Some("hydrant"),
                'N' => Some("newspaper"),
                'U' => Some("bin"),
                _ => None,
            };
            if let Some(key) = key {
                match find(key) {
                    Some(prop) => spawn_prop(commands, meshes, &mut cache, material, glow, prop, centre, facing_road),
                    None => info!("street: no model yet for {key:?}"),
                }
            }
            match ch {
                '@' => arrival = t,
                'a' | 'b' | 'c' => {
                    let e = footprints.entry(ch).or_insert((t, t));
                    e.0 = e.0.min(t);
                    e.1 = e.1.max(t);
                }
                'D' => match tile(x, z - 1) {
                    'a' => {
                        actions.insert(t, Action::Enter(Area::ComputerShop));
                        doors.insert(Area::ComputerShop, t);
                    }
                    'b' => {
                        // Your family's house: the door leads up to your bedroom.
                        actions.insert(t, Action::Enter(Area::Bedroom));
                        doors.insert(Area::Bedroom, t);
                    }
                    _ => {
                        actions.insert(t, Action::Say("The flats' door needs a key you don't have.", Expression::Puzzled));
                    }
                },
                'L' => {
                    // Sodium orange, hung out over the road.
                    let toward_road = if z > size.y / 2 { -0.6 } else { 0.6 };
                    commands.spawn((
                        PointLight { intensity: 0.0, range: 11.0, color: c(0xF2, 0xA8, 0x50), shadow_maps_enabled: false, ..default() },
                        Transform::from_translation(centre + Vec3::new(0.0, 3.0, toward_road)),
                        Lamp { strength: 160_000.0 },
                        DespawnOnExit(Screen::Game),
                    ));
                }
                'M' if !cars.is_empty() => {
                    // Cars park with the traffic: westbound on the north side, eastbound on
                    // the south side (traffic drives on the right).
                    let car = cars[car_index % cars.len()];
                    car_index += 1;
                    let along = Vec3::new(x as f32 + 2.0, heights[i], z as f32 + 1.0);
                    let yaw = if z > size.y / 2 { 0.0 } else { std::f32::consts::PI };
                    spawn_prop(commands, meshes, &mut cache, material, glow, car, along, yaw + CAR_YAW);
                }
                'T' => {
                    place(commands, meshes, material, tree(x as u32 * 31 + z as u32), centre, 0.0);
                }
                _ => {}
            }
        }
    }

    crate::world::memlog("street: tiles and props");
    for (&ch, &(min, max)) in &footprints {
        let tiles = max - min + IVec2::ONE;
        let corner = Vec3::new(min.x as f32, 0.0, min.y as f32);
        let door_x = (min.x..=max.x).find(|&x| tile(x, max.y + 1) == 'D').map(|x| x - min.x);
        let shell = if ch == 'b' {
            spawn_house(commands, meshes, &mut cache, material, window_light, corner, tiles, door_x.unwrap_or(tiles.x / 2), HOME, false, Some(IVec2::new(min.x + door_x.unwrap_or(0), max.y + 1)))
        } else {
            let style = if ch == 'a' { Building::ComputerShop } else { Building::Flats };
            place(commands, meshes, material, building(style, tiles.as_uvec2(), door_x).mesh(Vec3::ZERO), corner, 0.0)
        };
        // Clicking anywhere on a building walks to its door.
        if let Some(dx) = door_x {
            commands.entity(shell).insert(Clickable { tile: IVec2::new(min.x + dx, max.y + 1), area: Area::Street });
        }
    }

    crate::world::memlog("street: buildings");
    // Neighbours: more houses along the street past both ends of the map, and a row on the
    // next street north, backing onto the gardens (turned away, so we see their backs).
    let street_line = footprints.values().map(|(min, _)| min.y).min().unwrap_or(0);
    for (i, x) in [-16, -8, 42, 50].into_iter().enumerate() {
        let corner = Vec3::new(x as f32, 0.0, street_line as f32);
        spawn_house(commands, meshes, &mut cache, material, window_light, corner, IVec2::splat(6), 2 + (i as i32 % 2), NEIGHBOURS[i % 3], false, None);
    }
    for (i, x) in [-6, 2, 10, 18, 26, 34, 42].into_iter().enumerate() {
        let corner = Vec3::new(x as f32, 0.0, street_line as f32 - 17.0);
        spawn_house(commands, meshes, &mut cache, material, window_light, corner, IVec2::splat(6), 3, NEIGHBOURS[(i + 1) % 3], true, None);
    }

    crate::world::memlog("street: neighbours");
    // The front garden: a low brick wall along the pavement with stone coping, pillars at
    // the gate, hedges down both sides.
    let garden: Vec<IVec2> = (0..size.y).flat_map(|z| (0..size.x).map(move |x| IVec2::new(x, z))).filter(|t| matches!(tile(t.x, t.y), 'g' | 'w' | 'o')).collect();
    if let (Some(min), Some(max)) = (garden.iter().copied().reduce(IVec2::min), garden.iter().copied().reduce(IVec2::max)) {
        let (w, d) = ((max.x - min.x + 1) * 8, (max.y - min.y + 1) * 8);
        let mut g = VoxelGrid::new(w as u32, 7, d as u32);
        let brick = [c(0x9A, 0x4E, 0x3A), c(0x8A, 0x44, 0x34)];
        let hedge = [c(0x4E, 0x7E, 0x40), c(0x46, 0x74, 0x3A), c(0x58, 0x88, 0x48)];
        let gate = |x: i32| tile(min.x + x / 8, max.y) == 'o';
        for x in 0..w {
            if gate(x) {
                continue;
            }
            g.fill_with(IVec3::new(x, 0, d - 3), IVec3::new(x, 3, d - 1), |p| Some(if p.y == 3 { c(0xD8, 0xCC, 0xB4) } else { speckle(p, 4, &brick) }));
        }
        // Gate pillars either side of the opening, a little taller, with caps.
        for x in 0..w {
            if !gate(x) && (gate(x - 1) || gate(x + 1)) {
                let x0 = if gate(x + 1) { x - 1 } else { x };
                g.fill(IVec3::new(x0, 0, d - 3), IVec3::new(x0 + 1, 5, d - 1), brick[0]);
                g.fill(IVec3::new(x0, 6, d - 3), IVec3::new(x0 + 1, 6, d - 1), c(0xD8, 0xCC, 0xB4));
            }
        }
        for x in [0, w - 3] {
            g.fill_with(IVec3::new(x, 0, 0), IVec3::new(x + 2, 5, d - 4), |p| (hash(p, 8) % 9 != 0 || p.y < 4).then(|| speckle(p, 8, &hedge)));
        }
        place(commands, meshes, material, g.mesh(Vec3::ZERO), Vec3::new(min.x as f32, 0.0, min.y as f32), 0.0);
    }

    AreaMap { origin: Vec3::ZERO, size, walkable, actions, arrival, doors, zoom: 1.0, heights }
}

/// Spawns a street model standing centred on `centre`, turned by `yaw`.
#[allow(clippy::too_many_arguments)]
fn spawn_prop(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    cache: &mut MeshCache,
    material: &Handle<StandardMaterial>,
    glow: &Handle<StandardMaterial>,
    prop: &Prop,
    centre: Vec3,
    yaw: f32,
) {
    let s = prop.grid.size.as_vec3();
    let origin = Vec3::new(s.x / 2.0, 0.0, s.z / 2.0);
    let transform = Transform::from_translation(centre).with_rotation(Quat::from_rotation_y(yaw));
    commands.spawn((Mesh3d(cache.sized(meshes, &prop.grid, origin, FINE)), MeshMaterial3d(material.clone()), transform, DespawnOnExit(Screen::Game)));
    if let Some(lit) = &prop.glow {
        commands.spawn((Mesh3d(cache.sized(meshes, lit, origin, FINE)), MeshMaterial3d(glow.clone()), transform, DespawnOnExit(Screen::Game)));
    }
}

/// Turn applied to every car model so its front points along +X (west to east) before
/// the parking direction is applied. Set to match how the car models are built.
const CAR_YAW: f32 = 0.0;

/// The ground: 2 coarse voxels thick, pavements and grass on top, the road one voxel lower
/// so the kerb casts a shadow. Paving slabs, a pale kerb stone, a dark gutter, lane lines,
/// a dashed centre line, the zebra crossing, a manhole, drain grates.
fn ground(size: IVec2, ground_at: &dyn Fn(i32, i32) -> Ground) -> VoxelGrid {
    let border = BORDER * 8;
    let (gw, gd) = (size.x * 8 + 2 * border, size.y * 8 + 2 * border);
    let grass = [c(0x7C, 0xA8, 0x5A), c(0x74, 0xA0, 0x54), c(0x84, 0xB0, 0x60)];
    let lawn = [c(0x86, 0xB6, 0x60), c(0x7E, 0xAE, 0x5A)];
    let pavement = [c(0xC4, 0xBE, 0xB2), c(0xBC, 0xB6, 0xAA)];
    let asphalt = [c(0x5A, 0x5C, 0x62), c(0x56, 0x58, 0x5E)];
    let paint = c(0xEE, 0xE6, 0xC8);
    // The centre line runs between the two traffic lanes.
    let lanes: Vec<i32> = (0..size.y).filter(|&z| ground_at(-1, z) == Ground::Lane).collect();
    let centre_z = (lanes.first().copied().unwrap_or(0) + lanes.last().copied().unwrap_or(0) + 1) / 2;
    let mut g = VoxelGrid::new(gw as u32, 2, gd as u32);
    g.fill_with(IVec3::ZERO, IVec3::new(gw - 1, 1, gd - 1), |p| {
        let q = IVec3::new(p.x - border, p.y, p.z - border);
        let (tx, tz) = (q.x.div_euclid(8), q.z.div_euclid(8));
        let (vx, vz) = (q.x.rem_euclid(8), q.z.rem_euclid(8));
        let here = ground_at(tx, tz);
        if here.is_road() && p.y == 1 {
            return None;
        }
        // Neighbours, for kerbs and gutters.
        let north = ground_at(tx, tz - 1);
        let south = ground_at(tx, tz + 1);
        Some(match here {
            Ground::Pavement => {
                let kerb = (south.is_road() && vz == 7) || (north.is_road() && vz == 0);
                if kerb {
                    c(0xD8, 0xD2, 0xC6)
                } else if vx % 4 == 0 || vz % 4 == 0 {
                    c(0xAE, 0xA8, 0x9C)
                } else {
                    speckle(q, 1, &pavement)
                }
            }
            Ground::Parking | Ground::Lane | Ground::Crossing => {
                let gutter = (north == Ground::Pavement && vz == 0) || (south == Ground::Pavement && vz == 7);
                // Lane edge: a thin line between parking and traffic.
                let edge = (here == Ground::Lane && north == Ground::Parking && vz == 0) || (here == Ground::Lane && south == Ground::Parking && vz == 7);
                // The centre line: dashes 12 voxels on, 12 off, on the boundary between the
                // two traffic lanes.
                let centre = ((tz == centre_z && vz == 0) || (tz == centre_z - 1 && vz == 7)) && q.x.rem_euclid(24) < 12;
                // Zebra stripes: 4 voxels painted, 4 bare, across the road.
                let zebra = here == Ground::Crossing && q.z.rem_euclid(8) < 4 && !gutter;
                // A manhole and drain grates, by position.
                let manhole = here == Ground::Lane && hash(IVec3::new(tx, 0, tz), 31) % 41 == 0 && (2..6).contains(&vx) && (2..6).contains(&vz);
                let drain = gutter && (vx == 3 || vx == 4) && hash(IVec3::new(tx, 1, tz), 32) % 9 == 0;
                if zebra {
                    paint
                } else if gutter || drain {
                    if drain { c(0x2E, 0x30, 0x34) } else { c(0x4A, 0x4C, 0x52) }
                } else if (edge || centre) && here != Ground::Crossing {
                    paint
                } else if manhole {
                    c(0x6E, 0x70, 0x74)
                } else {
                    speckle(q, 2, &asphalt)
                }
            }
            Ground::Lawn => {
                // Mown stripes.
                lawn[q.x.div_euclid(4).rem_euclid(2) as usize]
            }
            // Dark soil under the bed plants.
            Ground::FlowerBed => {
                if hash(q, 12) % 3 == 0 { c(0x4A, 0x34, 0x28) } else { c(0x5A, 0x40, 0x30) }
            }
            Ground::Path => {
                if vx % 3 == 0 || vz % 3 == 0 { c(0xB0, 0xA4, 0x8C) } else { c(0xC8, 0xBC, 0xA2) }
            }
            Ground::Grass => speckle(q, 3, &grass),
        })
    });
    g
}

/// The north-west corner of your house's footprint on the street, in world metres.
pub fn house_corner() -> Vec3 {
    let map = rows(MAP);
    let corner = map.iter().enumerate().flat_map(|(z, row)| row.iter().enumerate().filter(|(_, ch)| **ch == 'b').map(move |(x, _)| IVec2::new(x as i32, z as i32))).reduce(IVec2::min).unwrap_or_default();
    Vec3::new(corner.x as f32, 0.0, corner.y as f32)
}

/// Splits a grid into the voxels below `y` and those at or above it (same size and layout).
fn split_at(grid: &VoxelGrid, y: i32) -> (VoxelGrid, VoxelGrid) {
    let s = grid.size;
    let (mut low, mut high) = (VoxelGrid::new(s.x, s.y, s.z), VoxelGrid::new(s.x, s.y, s.z));
    let s = s.as_ivec3();
    for z in 0..s.z {
        for yy in 0..s.y {
            for x in 0..s.x {
                if let Some(colour) = grid.get(x, yy, z) {
                    if yy < y { low.set(x, yy, z, Some(colour)) } else { high.set(x, yy, z, Some(colour)) }
                }
            }
        }
    }
    (low, high)
}

/// Spawns a house on a footprint of `tiles` with its north-west corner at `corner`. `turned`
/// faces it north (away from the street) instead of south. Your own house (`click` set)
/// splits into a ground floor and an upper floor plus roof, which hide while you are in your
/// bedroom. Returns the ground-floor shell.
#[allow(clippy::too_many_arguments)]
fn spawn_house(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    cache: &mut MeshCache,
    material: &Handle<StandardMaterial>,
    window_light: &Handle<StandardMaterial>,
    corner: Vec3,
    tiles: IVec2,
    door_tile: i32,
    style: HouseStyle,
    turned: bool,
    click: Option<IVec2>,
) -> Entity {
    // Your house has your bedroom upstairs, filling the footprint: its walls stand one
    // voxel outside it.
    let grow = if click.is_some() { 1 } else { 0 };
    let house = house(tiles, door_tile, style, grow);
    let centre = corner + Vec3::new(tiles.x as f32, 0.0, tiles.y as f32) / 2.0;
    let yaw = if turned { std::f32::consts::PI } else { 0.0 };
    // Meshes are built around the footprint's centre so a turned house turns in place.
    let grid_centre = (centre - corner - house.shell_at) * 8.0;
    let rotate = |offset: Vec3| Quat::from_rotation_y(yaw) * offset;
    let shells = if click.is_some() {
        let (low, high) = split_at(&house.shell, UPSTAIRS);
        let (glass_low, glass_high) = split_at(&house.glass, UPSTAIRS);
        vec![(low, glass_low, false), (high, glass_high, true)]
    } else {
        vec![(house.shell, house.glass, false)]
    };
    let mut ground_floor = None;
    for (shell, glass, upstairs) in shells {
        // Neighbours with the same style and door share their meshes.
        let s = place_shared(commands, cache.sized(meshes, &shell, grid_centre, VOXEL), material, centre, yaw);
        let g = place_shared(commands, cache.sized(meshes, &glass, grid_centre, VOXEL), window_light, centre, yaw);
        if upstairs {
            commands.entity(s).insert(HiddenIn(Area::Bedroom));
            commands.entity(g).insert(HiddenIn(Area::Bedroom));
        } else {
            ground_floor = Some(s);
        }
        // Clicking anywhere on your house walks to its door.
        if let Some(tile) = click {
            commands.entity(s).insert(Clickable { tile, area: Area::Street });
        }
    }
    // While you are in your bedroom, the roof shows cut open at the front corner.
    if let Some(cut_open) = &house.cut_open {
        let e = place_shared(commands, cache.sized(meshes, cut_open, grid_centre, VOXEL), material, centre, yaw);
        commands.entity(e).insert(OnlyIn(Area::Bedroom));
    }
    for (part, offset, cut_away) in &house.fine {
        let at = centre + rotate(*offset - (centre - corner));
        let e = place_shared(commands, cache.sized(meshes, part, Vec3::ZERO, FINE), material, at, yaw);
        if *cut_away && click.is_some() {
            commands.entity(e).insert(HiddenIn(Area::Bedroom));
        }
    }
    ground_floor.expect("a house has a ground floor")
}

/// One tile of flower bed in fine voxels: leafy clumps of different heights with small
/// flower heads in soft red, mustard, lavender and white, soil showing between.
fn flower_bed(seed: u32) -> VoxelGrid {
    let leaves = [c(0x4E, 0x7E, 0x40), c(0x5A, 0x8C, 0x48), c(0x46, 0x72, 0x3C)];
    let flowers = [c(0xC0, 0x56, 0x4A), c(0xD8, 0xA8, 0x4A), c(0x9A, 0x7E, 0xB8), c(0xF0, 0xEA, 0xE0)];
    let mut g = VoxelGrid::new(24, 9, 24);
    // Six to nine clumps, each a rounded mound with a few flowers on top.
    let count = 6 + hash(IVec3::ZERO, seed) % 4;
    for i in 0..count {
        let h = hash(IVec3::new(i as i32, 1, 0), seed);
        let centre = Vec3::new(3.0 + (h % 18) as f32, 0.0, 3.0 + (h / 18 % 18) as f32);
        let radius = 2.5 + (h / 324 % 3) as f32;
        let height = 3.0 + (h / 972 % 4) as f32;
        let flower = flowers[(h / 3888 % 4) as usize];
        for z in 0..24 {
            for y in 0..9 {
                for x in 0..24 {
                    let p = IVec3::new(x, y, z);
                    let q = p.as_vec3() + Vec3::splat(0.5);
                    let d = Vec2::new(q.x - centre.x, q.z - centre.z).length() / radius;
                    if d > 1.0 {
                        continue;
                    }
                    let top = height * (1.0 - d * d);
                    if q.y <= top {
                        g.set(x, y, z, Some(leaves[(hash(p, seed) % 3) as usize]));
                    } else if q.y <= top + 1.0 && hash(p, seed + i) % 3 == 0 {
                        g.set(x, y, z, Some(flower));
                    }
                }
            }
        }
    }
    g
}

/// A telephone wire from `from` to `to` (world metres), sagging `sag` metres in the middle:
/// a line of fine voxels, dark against the sky. Returns the grid and where its corner goes.
fn wire(from: Vec3, to: Vec3, sag: f32) -> (VoxelGrid, Vec3) {
    let points: Vec<Vec3> = {
        let steps = ((to - from).length() / FINE * 1.5).ceil().max(2.0) as i32;
        (0..=steps)
            .map(|i| {
                let t = i as f32 / steps as f32;
                from.lerp(to, t) - Vec3::Y * sag * 4.0 * t * (1.0 - t)
            })
            .collect()
    };
    let cells: Vec<IVec3> = points.iter().map(|p| (*p / FINE).floor().as_ivec3()).collect();
    let min = cells.iter().copied().reduce(IVec3::min).unwrap_or_default();
    let max = cells.iter().copied().reduce(IVec3::max).unwrap_or_default();
    let size = (max - min + IVec3::ONE).as_uvec3();
    let mut g = VoxelGrid::new(size.x, size.y, size.z);
    for cell in cells {
        let p = cell - min;
        g.set(p.x, p.y, p.z, Some(c(0x2A, 0x26, 0x24)));
    }
    (g, min.as_vec3() * FINE)
}
