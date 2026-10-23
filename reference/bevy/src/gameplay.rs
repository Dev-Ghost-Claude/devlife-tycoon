//! The player: click to walk, doors between areas, things to interact with, and the camera
//! that follows along. In the street the view can turn; in a room it stays put and pulls
//! back to frame the whole room.

use std::collections::{BinaryHeap, HashMap, VecDeque};

use bevy::{
    picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings},
    prelude::*,
    window::{CursorIcon, SystemCursorIcon},
};

use crate::{
    Screen,
    animal::{Gait, Outfit, PANTS, SHIRTS, Species},
    fine_animal::{Expression, Face, spawn_animal},
    camera::{CameraRig, CursorPicker, ground_point},
    voxel::hash,
    world::{Action, Area, Areas, Clickable, VoxelMaterial},
};

/// Street zoom levels, chosen with the mouse wheel: a street voxel (1/8 m) is 3, 2 or 1
/// canvas pixels.
const STREET_ZOOMS: [f32; 3] = [1.0, 1.5, 3.0];
/// The street starts at the middle level.
const STREET_ZOOM_START: usize = 1;
/// On entering a room the view starts this much closer, then pulls back.
const ROOM_PULL_BACK: f32 = 0.7;
const WALK_SPEED: f32 = 2.4;
const FADE_SECS: f32 = 0.25;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Game), spawn_people.after(crate::world::build_areas))
            .add_systems(
                Update,
                (hover_cursor, click_to_walk, walk, turn_view, follow_camera, run_transition, resting_face).chain().run_if(in_state(Screen::Game)),
            );
    }
}

#[derive(Component)]
pub struct Player;

/// Where the player is now.
#[derive(Resource)]
pub struct Here(pub Area);

/// One line of text shown at the bottom of the screen for a few seconds.
#[derive(Resource, Default)]
pub struct Toast {
    pub text: String,
    pub age: f32,
}

impl Toast {
    pub fn say(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.age = 0.0;
        // Also in the log, so scripted runs can check what was said.
        info!("toast: {}", self.text);
    }
}

/// Input from the `--capture` script instead of the mouse and keyboard.
#[derive(Resource, Default)]
pub struct Scripted {
    /// A tile in the current area to click.
    pub click: Option<IVec2>,
    /// A canvas pixel to click, going through the same picking as the mouse.
    pub click_canvas: Option<Vec2>,
    /// Quarter turns of the street view (negative turns left).
    pub turn: i32,
}

/// Holds the camera at a zoom, whatever the area would choose (for capture shots).
#[derive(Resource)]
pub struct ZoomOverride(pub f32);

/// `--proof`: start in the bedroom as the fine-detail fox, skipping the splash.
#[derive(Resource)]
pub struct ProofMode;

/// Who the player turned out to be this run.
#[derive(Resource)]
pub struct Identity {
    pub species: Species,
}

#[derive(Component, Default)]
struct Walker {
    path: VecDeque<Vec3>,
    /// The tile whose action runs on arrival.
    then: Option<IVec2>,
}

/// Turning the street view: the target angle; the rig eases toward it.
#[derive(Resource)]
struct ViewTurn {
    target: f32,
    /// Index into STREET_ZOOMS.
    zoom: usize,
    /// The street yaw to come back to after leaving a room.
    street: f32,
}

#[derive(Component)]
struct Fade;

/// A door trip in progress: fade out, move, fade in.
#[derive(Resource)]
struct Transition {
    to: Area,
    at: IVec2,
    facing: Vec3,
    t: f32,
    moved: bool,
}

fn seed() -> u32 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.subsec_nanos() ^ d.as_secs() as u32).unwrap_or(7)
}

fn choose<T: Copy>(items: &[T], seed: u32, salt: i32) -> T {
    items[(hash(IVec3::new(salt, 0, 0), seed) % items.len() as u32) as usize]
}

pub fn spawn_people(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<VoxelMaterial>,
    areas: Res<Areas>,
    proof: Option<Res<ProofMode>>,
) {
    let seed = seed();
    let species = choose(&Species::ALL, seed, 1);
    let outfit = Outfit { shirt: choose(&SHIRTS, seed, 2), pants: choose(&PANTS, seed, 3), shoes: Color::srgb_u8(0x2A, 0x26, 0x24) };
    let street = &areas.0[&Area::Street];
    let (player, species, at) = if proof.is_some() {
        let room = &areas.0[&Area::Bedroom];
        let at = room.tile_centre(room.arrival);
        // A sky-blue tee, dark jeans and white sneakers: 1978, a colour the room doesn't
        // use, and nothing cream next to the cream chin.
        let outfit = Outfit { shirt: Color::srgb_u8(0x5F, 0x8F, 0xB8), pants: Color::srgb_u8(0x2E, 0x3F, 0x5A), shoes: Color::srgb_u8(0xEE, 0xEA, 0xE0) };
        (spawn_animal(&mut commands, &mut meshes, &material.0, Species::Fox, &outfit, at, Vec3::Z), Species::Fox, at)
    } else {
        let at = street.tile_centre(street.arrival);
        (spawn_animal(&mut commands, &mut meshes, &material.0, species, &outfit, at, Vec3::Z), species, at)
    };
    commands.entity(player).insert((Player, Walker::default(), DespawnOnExit(Screen::Game)));
    commands.insert_resource(Identity { species });
    commands.insert_resource(Here(if proof.is_some() { Area::Bedroom } else { Area::Street }));
    commands.insert_resource(Toast::default());
    commands.init_resource::<Scripted>();
    commands.insert_resource(ViewTurn { target: 45.0_f32.to_radians(), zoom: STREET_ZOOM_START, street: 45.0_f32.to_radians() });
    if proof.is_some() {
        let room = &areas.0[&Area::Bedroom];
        commands.insert_resource(CameraRig::new(room.centre() + Vec3::new(0.0, 0.8, 0.0), 45.0_f32.to_radians(), room.zoom));
    } else {
        commands.insert_resource(CameraRig::new(at, 45.0_f32.to_radians(), STREET_ZOOMS[STREET_ZOOM_START]));
    }

    // The shopkeeper: someone of a different species, behind the counter, facing the room.
    let shop = &areas.0[&Area::ComputerShop];
    let keeper_species = Species::ALL.into_iter().find(|s| *s != species).unwrap_or(Species::Bear);
    let keeper_outfit = Outfit { shirt: Color::srgb_u8(0x2E, 0x7E, 0x7A), pants: PANTS[1], shoes: Color::srgb_u8(0x2A, 0x26, 0x24) };
    if let Some((&tile, _)) = shop.actions.iter().find(|(_, a)| matches!(a, Action::Say(t, _) if t.starts_with('"'))) {
        let keeper = spawn_animal(&mut commands, &mut meshes, &material.0, keeper_species, &keeper_outfit, shop.tile_centre(tile), Vec3::Z);
        commands.entity(keeper).insert((Clickable { tile, area: Area::ComputerShop }, DespawnOnExit(Screen::Game)));
    }

    commands.spawn((
        Sprite::from_color(Color::BLACK.with_alpha(0.0), Vec2::splat(10_000.0)),
        Transform::from_xyz(0.0, 0.0, 50.0),
        Fade,
        DespawnOnExit(Screen::Game),
    ));
}

/// Finds the clickable thing under a ray: the nearest shape that is, or belongs to, a
/// [`Clickable`]. Other shapes (trees, lamps, floors) do not block the click.
fn pick_clickable(ray: Ray3d, ray_cast: &mut MeshRayCast, clickables: &Query<&Clickable>, parents: &Query<&ChildOf>, area: Area) -> Option<IVec2> {
    let owner = |mut e: Entity| loop {
        if let Ok(c) = clickables.get(e) {
            return (c.area == area).then_some(c.tile);
        }
        e = parents.get(e).ok()?.parent();
    };
    let filter = |e: Entity| owner(e).is_some();
    let hits = ray_cast.cast_ray(ray, &MeshRayCastSettings::default().with_filter(&filter));
    hits.first().and_then(|(e, _)| owner(*e))
}

/// A pointing hand over anything that can be used.
fn hover_cursor(
    mut commands: Commands,
    picker: CursorPicker,
    mut ray_cast: MeshRayCast,
    clickables: Query<&Clickable>,
    parents: Query<&ChildOf>,
    here: Res<Here>,
    window: Single<(Entity, Option<&CursorIcon>), With<Window>>,
) {
    let over = picker.ray().and_then(|ray| pick_clickable(ray, &mut ray_cast, &clickables, &parents, here.0)).is_some();
    let want = CursorIcon::System(if over { SystemCursorIcon::Pointer } else { SystemCursorIcon::Default });
    let (entity, current) = *window;
    if current != Some(&want) {
        commands.entity(entity).insert(want);
    }
}

fn click_to_walk(
    mouse: Res<ButtonInput<MouseButton>>,
    picker: CursorPicker,
    mut ray_cast: MeshRayCast,
    clickables: Query<&Clickable>,
    parents: Query<&ChildOf>,
    areas: Res<Areas>,
    here: Res<Here>,
    transition: Option<Res<Transition>>,
    mut scripted: ResMut<Scripted>,
    player: Single<(&Transform, &mut Walker), With<Player>>,
) {
    if transition.is_some() {
        return;
    }
    let area = &areas.0[&here.0];
    let mouse_ray = if let Some(px) = scripted.click_canvas.take() {
        picker.ray_at_canvas(px)
    } else if mouse.just_pressed(MouseButton::Left) {
        picker.ray()
    } else {
        None
    };
    let target = if let Some(tile) = scripted.click.take() {
        tile
    } else if let Some(ray) = mouse_ray {
        // A clicked object wins; otherwise walk to the spot on the ground.
        match pick_clickable(ray, &mut ray_cast, &clickables, &parents, here.0) {
            Some(tile) => tile,
            None => match ground_point(ray, area.origin.y) {
                Some(point) => area.tile_at(point),
                None => return,
            },
        }
    } else {
        return;
    };
    let (transform, mut walker) = player.into_inner();
    let start = area.tile_at(transform.translation);

    // Clicking a blocked thing walks to the nearest free neighbour and uses it from there.
    let (goal, then) = if area.walkable(target) {
        (target, area.actions.contains_key(&target).then_some(target))
    } else if area.actions.contains_key(&target) {
        let Some(n) = NEIGHBOURS.iter().map(|d| target + *d).filter(|t| area.walkable(*t)).min_by_key(|t| (*t - start).length_squared()) else {
            return;
        };
        (n, Some(target))
    } else {
        return;
    };

    let Some(tiles) = find_path(start, goal, |t| area.walkable(t)) else { return };
    debug!("click: tile {target} in {:?}, walking to {goal}", here.0);
    walker.path = tiles.into_iter().map(|t| area.tile_centre(t)).collect();
    walker.then = then;
}

fn walk(
    time: Res<Time>,
    areas: Res<Areas>,
    here: Res<Here>,
    mut toast: ResMut<Toast>,
    mut commands: Commands,
    player: Single<(&mut Transform, &mut Walker, &mut Gait, Option<&mut Face>), With<Player>>,
) {
    let (mut transform, mut walker, mut gait, face) = player.into_inner();
    let dt = time.delta_secs();
    let mut step = WALK_SPEED * dt;
    let mut heading = None;
    while let Some(&next) = walker.path.front() {
        let to = next - transform.translation;
        let distance = to.length();
        if distance <= step {
            transform.translation = next;
            step -= distance;
            walker.path.pop_front();
            if distance > 0.0 {
                heading = Some(to);
            }
        } else {
            transform.translation += to / distance * step;
            heading = Some(to);
            break;
        }
    }
    gait.moving = !walker.path.is_empty();
    if let Some(h) = heading.filter(|h| h.length_squared() > 1e-6) {
        let want = Transform::default().looking_to(h.normalize(), Vec3::Y).rotation;
        transform.rotation = transform.rotation.slerp(want, 1.0 - (-14.0 * dt).exp());
    }

    if walker.path.is_empty()
        && let Some(target) = walker.then.take()
    {
        let area = &areas.0[&here.0];
        // Face the thing being used.
        let look = area.tile_centre(target) - transform.translation;
        if look.length_squared() > 1e-4 {
            transform.look_to(look.normalize(), Vec3::Y);
        }
        match area.actions.get(&target) {
            Some(Action::Say(text, expression)) => {
                toast.say(*text);
                if let Some(mut face) = face {
                    face.react(*expression, 3.5);
                }
            }
            Some(Action::Enter(room)) => {
                let inside = &areas.0[room];
                commands.insert_resource(Transition { to: *room, at: inside.arrival, facing: Vec3::X, t: 0.0, moved: false });
            }
            Some(Action::Exit) => {
                let street = &areas.0[&Area::Street];
                if let Some(door) = street.doors.get(&here.0) {
                    commands.insert_resource(Transition { to: Area::Street, at: *door + IVec2::Y, facing: Vec3::Z, t: 0.0, moved: false });
                }
            }
            None => {}
        }
    }
}

fn turn_view(
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<bevy::input::mouse::MouseWheel>,
    here: Res<Here>,
    mut scripted: ResMut<Scripted>,
    mut turn: ResMut<ViewTurn>,
) {
    let quarters = std::mem::take(&mut scripted.turn);
    let scroll: f32 = wheel.read().map(|w| w.y).sum();
    if here.0 != Area::Street {
        return;
    }
    // Wheel up moves in, wheel down pulls back, one level per notch.
    if scroll > 0.0 {
        turn.zoom = turn.zoom.saturating_sub(1);
    } else if scroll < 0.0 {
        turn.zoom = (turn.zoom + 1).min(STREET_ZOOMS.len() - 1);
    }
    let quarter = std::f32::consts::FRAC_PI_2;
    if keys.just_pressed(KeyCode::KeyQ) {
        turn.target -= quarter;
    }
    if keys.just_pressed(KeyCode::KeyE) {
        turn.target += quarter;
    }
    turn.target += quarters as f32 * quarter;
    turn.street = turn.target;
}

fn follow_camera(
    time: Res<Time>,
    here: Res<Here>,
    areas: Res<Areas>,
    turn: Res<ViewTurn>,
    mut rig: ResMut<CameraRig>,
    player: Single<&Transform, With<Player>>,
    gallery: Option<Res<crate::gallery::GalleryMode>>,
    zoom_override: Option<Res<ZoomOverride>>,
) {
    let ease = |rate: f32| 1.0 - (-rate * time.delta_secs()).exp();
    let (focus, zoom) = if gallery.is_some() {
        (crate::gallery::gallery_centre(), 1.0)
    } else if here.0 == Area::Street {
        (player.translation, STREET_ZOOMS[turn.zoom])
    } else {
        // Frame the room: aim a little above the floor centre so the walls fit too.
        (areas.0[&here.0].centre() + Vec3::new(0.0, 0.8, 0.0), areas.0[&here.0].zoom)
    };
    rig.focus = rig.focus.lerp(focus, ease(8.0));
    rig.yaw += (turn.target - rig.yaw) * ease(8.0);
    // The rig eases toward this zoom itself.
    rig.zoom = zoom_override.map_or(zoom, |z| z.0);
}

fn run_transition(
    time: Res<Time>,
    mut commands: Commands,
    transition: Option<ResMut<Transition>>,
    areas: Res<Areas>,
    mut here: ResMut<Here>,
    mut turn: ResMut<ViewTurn>,
    mut rig: ResMut<CameraRig>,
    fade: Single<&mut Sprite, With<Fade>>,
    player: Single<(&mut Transform, &mut Walker), With<Player>>,
) {
    let Some(mut tr) = transition else { return };
    tr.t += time.delta_secs();
    let mut sprite = fade.into_inner();
    if !tr.moved && tr.t >= FADE_SECS {
        // Fully dark: move the player and snap the camera to the new place.
        let area = &areas.0[&tr.to];
        let (mut transform, mut walker) = player.into_inner();
        transform.translation = area.tile_centre(tr.at);
        transform.look_to(tr.facing, Vec3::Y);
        walker.path.clear();
        walker.then = None;
        here.0 = tr.to;
        if tr.to == Area::Street {
            turn.target = turn.street;
            *rig = CameraRig::new(transform.translation, turn.street, STREET_ZOOMS[turn.zoom]);
        } else {
            // Rooms keep one angle and are framed whole. Start close in, then pull back.
            turn.target = 45.0_f32.to_radians();
            *rig = CameraRig::new(area.centre() + Vec3::new(0.0, 0.8, 0.0), turn.target, area.zoom);
            rig.apparent = area.zoom * ROOM_PULL_BACK;
        }
        tr.moved = true;
    }
    let alpha = if tr.moved { 1.0 - (tr.t - FADE_SECS) / FADE_SECS } else { tr.t / FADE_SECS };
    sprite.color.set_alpha(alpha.clamp(0.0, 1.0));
    if tr.moved && tr.t >= 2.0 * FADE_SECS {
        commands.remove_resource::<Transition>();
    }
}

const NEIGHBOURS: [IVec2; 8] = [
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
    IVec2::new(-1, -1),
];

/// A* over tiles with diagonal steps that never cut a blocked corner. Returns the tiles to
/// walk through after `start`, ending at `goal`.
pub fn find_path(start: IVec2, goal: IVec2, walkable: impl Fn(IVec2) -> bool) -> Option<Vec<IVec2>> {
    #[derive(PartialEq, Eq)]
    struct Open(u32, IVec2);
    impl Ord for Open {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.0.cmp(&self.0).then_with(|| (self.1.x, self.1.y).cmp(&(other.1.x, other.1.y)))
        }
    }
    impl PartialOrd for Open {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    // Costs in tenths: 10 straight, 14 diagonal.
    let estimate = |t: IVec2| {
        let d = (t - goal).abs();
        (10 * d.x.max(d.y) + 4 * d.x.min(d.y)) as u32
    };
    let mut open = BinaryHeap::from([Open(estimate(start), start)]);
    let mut cost = HashMap::from([(start, 0u32)]);
    let mut came_from = HashMap::new();
    while let Some(Open(_, at)) = open.pop() {
        if at == goal {
            let mut path = vec![at];
            let mut t = at;
            while let Some(&prev) = came_from.get(&t) {
                if prev == start {
                    break;
                }
                path.push(prev);
                t = prev;
            }
            path.reverse();
            return Some(path);
        }
        for d in NEIGHBOURS {
            let next = at + d;
            let diagonal = d.x != 0 && d.y != 0;
            if !walkable(next) || diagonal && (!walkable(at + IVec2::new(d.x, 0)) || !walkable(at + IVec2::new(0, d.y))) {
                continue;
            }
            let c = cost[&at] + if diagonal { 14 } else { 10 };
            if cost.get(&next).is_none_or(|&old| c < old) {
                cost.insert(next, c);
                came_from.insert(next, at);
                open.push(Open(c + estimate(next), next));
            }
        }
    }
    None
}

/// The player's resting face follows their state. There is no mood or stamina yet, so for
/// now it follows the clock: sleepy late at night.
fn resting_face(clock: Res<crate::clock::GameClock>, player: Single<&mut Face, With<Player>>) {
    let hour = clock.hour();
    let resting = if !(6.0..22.0).contains(&hour) { Expression::Tired } else { Expression::Neutral };
    let mut face = player.into_inner();
    if face.resting != resting {
        face.resting = resting;
    }
}
