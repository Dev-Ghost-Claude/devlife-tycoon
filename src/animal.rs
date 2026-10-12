//! Voxel animals: a species builds its head, ears and tail; everyone shares the body plan.
//! Parts are separate entities so legs and arms can swing from their joints.

use bevy::prelude::*;

use crate::voxel::{VOXEL, VoxelGrid};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Species {
    Fox,
    Rabbit,
    Raccoon,
    Bear,
}

impl Species {
    pub const ALL: [Species; 4] = [Species::Fox, Species::Rabbit, Species::Raccoon, Species::Bear];

    pub fn name(self) -> &'static str {
        match self {
            Species::Fox => "fox",
            Species::Rabbit => "rabbit",
            Species::Raccoon => "raccoon",
            Species::Bear => "bear",
        }
    }

    fn fur(self) -> Fur {
        let c = |r, g, b| Color::srgb_u8(r, g, b);
        match self {
            Species::Fox => Fur { main: c(0xE0, 0x7A, 0x3F), light: c(0xF4, 0xE3, 0xC9), dark: c(0x3A, 0x2A, 0x22), accent: c(0x4A, 0x32, 0x28) },
            Species::Rabbit => Fur { main: c(0xC8, 0xB4, 0x9A), light: c(0xF0, 0xEA, 0xE0), dark: c(0x3A, 0x2E, 0x2A), accent: c(0xE8, 0x9A, 0xA0) },
            Species::Raccoon => Fur { main: c(0x8E, 0x8C, 0x92), light: c(0xDA, 0xD6, 0xD2), dark: c(0x2E, 0x2C, 0x32), accent: c(0x45, 0x43, 0x4A) },
            Species::Bear => Fur { main: c(0x7A, 0x52, 0x36), light: c(0xC4, 0x9A, 0x6C), dark: c(0x2A, 0x1C, 0x14), accent: c(0x5E, 0x3E, 0x28) },
        }
    }
}

struct Fur {
    main: Color,
    light: Color,
    dark: Color,
    accent: Color,
}

/// Clothes, picked per character.
pub struct Outfit {
    pub shirt: Color,
    pub pants: Color,
    pub shoes: Color,
}

pub const SHIRTS: [Color; 6] = [
    Color::srgb_u8(0x3E, 0x6E, 0xA8),
    Color::srgb_u8(0x5A, 0x8F, 0x5A),
    Color::srgb_u8(0xC8, 0x4B, 0x4B),
    Color::srgb_u8(0xE6, 0xC0, 0x4E),
    Color::srgb_u8(0x7A, 0x5C, 0xA6),
    Color::srgb_u8(0xEE, 0xEE, 0xE8),
];
pub const PANTS: [Color; 4] = [
    Color::srgb_u8(0x2E, 0x3E, 0x5C),
    Color::srgb_u8(0x4A, 0x40, 0x38),
    Color::srgb_u8(0x30, 0x30, 0x34),
    Color::srgb_u8(0x6A, 0x72, 0x7C),
];

#[derive(Component)]
pub struct Animal;

/// A leg or arm that swings from its joint. `owner` is the animal's root, which holds the gait.
#[derive(Component, Clone, Copy)]
pub struct Limb {
    owner: Entity,
    /// +1 swings forward when the phase is positive, -1 the opposite way.
    sign: f32,
}

/// The torso and everything above it; bobs while walking.
#[derive(Component)]
pub struct UpperBody {
    owner: Entity,
}

/// Walking state, shared by the player and anyone else who walks.
#[derive(Component, Default)]
pub struct Gait {
    /// Advances while moving; drives the swing.
    pub phase: f32,
    /// 0 standing still, 1 full stride. Eases in and out.
    pub stride: f32,
    pub moving: bool,
}

pub struct AnimalPlugin;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_gait);
    }
}

/// Spawns an animal standing at `at`, facing `facing`. Returns the root entity.
pub fn spawn_animal(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    species: Species,
    outfit: &Outfit,
    at: Vec3,
    facing: Vec3,
) -> Entity {
    let fur = species.fur();
    let mut part = |grid: VoxelGrid, origin: Vec3| (Mesh3d(meshes.add(grid.mesh(origin))), MeshMaterial3d(material.clone()));
    let v = VOXEL;

    // Body plan, in voxels: legs 3 tall, torso 4 tall, head 4 tall; 6 wide with the arms.
    // About 1.4 m tall and 0.75 m wide: a door is 2.25 m.
    // Legs: 2 x 3 x 2, pivot at the hip. Bottom row is shoes.
    let mut leg = VoxelGrid::new(2, 3, 2);
    leg.fill(IVec3::new(0, 1, 0), IVec3::new(1, 2, 1), outfit.pants);
    leg.fill(IVec3::new(0, 0, 0), IVec3::new(1, 0, 1), outfit.shoes);
    // Torso: 4 x 4 x 3, a belt line of pants at the bottom.
    let mut torso = VoxelGrid::new(4, 4, 3);
    torso.fill(IVec3::ZERO, IVec3::new(3, 3, 2), outfit.shirt);
    torso.fill(IVec3::ZERO, IVec3::new(3, 0, 2), outfit.pants);
    // Arms: 1 x 3 x 2, pivot at the shoulder; sleeve on top, paw at the bottom.
    let mut arm = VoxelGrid::new(1, 3, 2);
    arm.fill(IVec3::ZERO, IVec3::new(0, 2, 1), outfit.shirt);
    arm.fill(IVec3::ZERO, IVec3::new(0, 0, 1), fur.main);

    let (head, head_origin) = build_head(species, &fur);
    let tail = build_tail(species, &fur);
    let tail_width = tail.size.x as f32;

    let root = commands
        .spawn((
            Transform::from_translation(at).looking_to(facing, Vec3::Y),
            Visibility::default(),
            Animal,
            Gait::default(),
        ))
        .id();

    let limb = |sign| Limb { owner: root, sign };
    let left_leg = commands.spawn((part(leg.clone(), Vec3::new(1.0, 3.0, 1.0)), Transform::from_xyz(-1.0 * v, HIP * v, 0.0), limb(1.0))).id();
    let right_leg = commands.spawn((part(leg, Vec3::new(1.0, 3.0, 1.0)), Transform::from_xyz(1.0 * v, HIP * v, 0.0), limb(-1.0))).id();
    let upper = commands.spawn((Transform::from_xyz(0.0, HIP * v, 0.0), Visibility::default(), UpperBody { owner: root })).id();
    let torso = commands.spawn((part(torso, Vec3::new(2.0, 0.0, 1.5)), Transform::default())).id();
    let left_arm = commands.spawn((part(arm.clone(), Vec3::new(0.5, 3.0, 1.0)), Transform::from_xyz(-2.5 * v, 4.0 * v, 0.0), limb(-1.0))).id();
    let right_arm = commands.spawn((part(arm, Vec3::new(0.5, 3.0, 1.0)), Transform::from_xyz(2.5 * v, 4.0 * v, 0.0), limb(1.0))).id();
    let head = commands.spawn((part(head, head_origin), Transform::from_xyz(0.0, 4.0 * v, 0.0))).id();
    let tail = commands.spawn((part(tail, Vec3::new(tail_width / 2.0, 0.0, 0.0)), Transform::from_xyz(0.0, 1.0 * v, 1.5 * v))).id();

    commands.entity(upper).add_children(&[torso, left_arm, right_arm, head, tail]);
    commands.entity(root).add_children(&[left_leg, right_leg, upper]);
    root
}

/// Hip height in voxels: where the legs hang from and the upper body sits.
const HIP: f32 = 3.0;

/// Head grid plus the grid point that sits on the neck. The head is 5 wide so the nose has
/// a centre column; the snout pokes out at the front (z = 0 and 1).
fn build_head(species: Species, fur: &Fur) -> (VoxelGrid, Vec3) {
    const SNOUT: i32 = 2;
    let mut g = VoxelGrid::new(5, 9, 4 + SNOUT as u32);
    let front = SNOUT;
    g.fill(IVec3::new(0, 0, front), IVec3::new(4, 3, front + 3), fur.main);
    let eyes = |g: &mut VoxelGrid, c: Color| {
        g.set(1, 2, front, Some(c));
        g.set(3, 2, front, Some(c));
    };
    match species {
        Species::Fox => {
            // Pointed two-voxel snout with a dark nose, cream cheeks, dark ear tips.
            g.fill(IVec3::new(2, 0, 0), IVec3::new(2, 1, front - 1), fur.light);
            g.set(2, 1, 0, Some(fur.dark));
            g.fill(IVec3::new(0, 0, front), IVec3::new(4, 0, front), fur.light);
            for x in [0, 4] {
                g.set(x, 4, front + 1, Some(fur.main));
                g.set(x, 5, front + 1, Some(fur.accent));
            }
            eyes(&mut g, fur.dark);
        }
        Species::Rabbit => {
            // Small muzzle, pink nose, long ears with pink insides.
            g.set(2, 0, front - 1, Some(fur.light));
            g.set(2, 1, front - 1, Some(fur.accent));
            for x in [1, 3] {
                g.fill(IVec3::new(x, 4, front + 2), IVec3::new(x, 8, front + 2), fur.main);
                g.fill(IVec3::new(x, 5, front + 1), IVec3::new(x, 7, front + 1), fur.accent);
            }
            eyes(&mut g, fur.dark);
        }
        Species::Raccoon => {
            // Dark mask band across the eyes, light muzzle, small light ears.
            g.fill(IVec3::new(0, 2, front), IVec3::new(4, 2, front), fur.accent);
            g.set(2, 0, front - 1, Some(fur.light));
            g.set(2, 1, front - 1, Some(fur.dark));
            for x in [0, 4] {
                g.set(x, 4, front + 1, Some(fur.light));
            }
            eyes(&mut g, fur.light);
        }
        Species::Bear => {
            // Broad muzzle, round ears with dark insides.
            g.fill(IVec3::new(1, 0, front - 1), IVec3::new(3, 1, front - 1), fur.light);
            g.set(2, 1, front - 1, Some(fur.dark));
            for x in [0, 4] {
                g.fill(IVec3::new(x, 4, front + 1), IVec3::new(x, 4, front + 2), fur.main);
                g.set(x, 4, front + 1, Some(fur.accent));
            }
            eyes(&mut g, fur.dark);
        }
    }
    (g, Vec3::new(2.5, 0.0, front as f32 + 2.0))
}

/// Tail grid, attached at its front-bottom edge to the back of the torso.
fn build_tail(species: Species, fur: &Fur) -> VoxelGrid {
    match species {
        Species::Fox => {
            let mut g = VoxelGrid::new(2, 2, 4);
            g.fill(IVec3::ZERO, IVec3::new(1, 1, 3), fur.main);
            g.fill(IVec3::new(0, 0, 3), IVec3::new(1, 1, 3), fur.light);
            g.clear(IVec3::new(0, 1, 0), IVec3::new(1, 1, 0));
            g
        }
        Species::Rabbit => {
            let mut g = VoxelGrid::new(2, 2, 1);
            g.fill(IVec3::ZERO, IVec3::new(1, 1, 0), fur.light);
            g
        }
        Species::Raccoon => {
            let mut g = VoxelGrid::new(1, 1, 4);
            g.fill_with(IVec3::ZERO, IVec3::new(0, 0, 3), |p| Some(if p.z % 2 == 0 { fur.main } else { fur.accent }));
            g
        }
        Species::Bear => {
            let mut g = VoxelGrid::new(1, 1, 1);
            g.set(0, 0, 0, Some(fur.main));
            g
        }
    }
}

fn animate_gait(
    time: Res<Time>,
    mut gaits: Query<&mut Gait>,
    mut limbs: Query<(&Limb, &mut Transform), Without<UpperBody>>,
    mut uppers: Query<(&UpperBody, &mut Transform), Without<Limb>>,
) {
    let dt = time.delta_secs();
    for mut gait in &mut gaits {
        let target = if gait.moving { 1.0 } else { 0.0 };
        gait.stride += (target - gait.stride) * (1.0 - (-12.0 * dt).exp());
        if gait.moving {
            gait.phase += dt * 9.0;
        }
    }
    for (limb, mut transform) in &mut limbs {
        if let Ok(gait) = gaits.get(limb.owner) {
            transform.rotation = Quat::from_rotation_x(limb.sign * gait.phase.sin() * 0.7 * gait.stride);
        }
    }
    for (upper, mut transform) in &mut uppers {
        if let Ok(gait) = gaits.get(upper.owner) {
            transform.translation.y = HIP * VOXEL + gait.phase.sin().abs() * VOXEL * 0.6 * gait.stride;
        }
    }
}
