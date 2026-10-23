//! Animal kinds, clothes and the walk cycle. The models themselves are built in
//! `fine_animal.rs`.

use bevy::prelude::*;


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

    pub(crate) fn fur(self) -> Fur {
        let c = |r, g, b| Color::srgb_u8(r, g, b);
        match self {
            Species::Fox => Fur {
                main: c(0xE0, 0x7A, 0x3F),
                light: c(0xF4, 0xE3, 0xC9),
                dark: c(0x3A, 0x2A, 0x22),
                accent: c(0x4A, 0x32, 0x28),
                nose: c(0x2A, 0x1E, 0x18),
                paw: c(0x3A, 0x2A, 0x22),
            },
            Species::Rabbit => Fur {
                main: c(0xC8, 0xB4, 0x9A),
                light: c(0xF0, 0xEA, 0xE0),
                dark: c(0x3A, 0x2E, 0x2A),
                accent: c(0xE8, 0x9A, 0xA0),
                nose: c(0xD8, 0x70, 0x80),
                paw: c(0xF0, 0xEA, 0xE0),
            },
            Species::Raccoon => Fur {
                main: c(0x8E, 0x8C, 0x92),
                light: c(0xDA, 0xD6, 0xD2),
                dark: c(0x2E, 0x2C, 0x32),
                accent: c(0x3E, 0x3C, 0x44),
                nose: c(0x1A, 0x18, 0x1C),
                paw: c(0x2E, 0x2C, 0x32),
            },
            Species::Bear => Fur {
                main: c(0x7A, 0x52, 0x36),
                light: c(0xC4, 0x9A, 0x6C),
                dark: c(0x2A, 0x1C, 0x14),
                accent: c(0x5E, 0x3E, 0x28),
                nose: c(0x2A, 0x1C, 0x14),
                paw: c(0x5E, 0x3E, 0x28),
            },
        }
    }
}

/// A species' colours: `main` fur, `light` chin and belly, `dark` ear tips and ear
/// insides, `accent` a species mark (the raccoon's mask, the rabbit's ear insides), the
/// nose, and the paws.
pub(crate) struct Fur {
    pub main: Color,
    pub light: Color,
    pub dark: Color,
    pub accent: Color,
    pub nose: Color,
    pub paw: Color,
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
    pub owner: Entity,
    /// +1 swings forward when the phase is positive, -1 the opposite way.
    pub sign: f32,
}

/// The torso and everything above it; bobs while walking.
#[derive(Component)]
pub struct UpperBody {
    pub owner: Entity,
    /// Resting height above the animal's root, in metres.
    pub base: f32,
    /// How far it bobs at full stride, in metres.
    pub bob: f32,
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
        app.add_systems(Update, (animate_gait, crate::fine_animal::animate_faces));
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
            transform.translation.y = upper.base + gait.phase.sin().abs() * upper.bob * gait.stride;
        }
    }
}
