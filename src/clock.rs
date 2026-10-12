//! Game time, and the sun, sky and lamps that follow it.

use std::f32::consts::PI;

use bevy::{light::CascadeShadowConfigBuilder, prelude::*};

use crate::{
    Screen,
    gameplay::Here,
    pixel_camera::WorldCamera,
    world::{Area, Lamp, LampGlow},
};

/// Real seconds per game minute at normal speed: a day lasts 24 real minutes.
const SECS_PER_MINUTE: f32 = 1.0;
/// How much faster time runs while fast-forward is on.
const FAST: f32 = 30.0;

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Game), spawn_sun)
            .add_systems(Update, (tick, light_the_world).chain().run_if(in_state(Screen::Game)));
    }
}

#[derive(Resource)]
pub struct GameClock {
    /// Minutes since midnight of day 1.
    pub minutes: f32,
    pub fast: bool,
}

impl GameClock {
    pub fn day(&self) -> u32 {
        (self.minutes / 1440.0) as u32 + 1
    }

    /// Hour of the day, 0.0 to 24.0.
    pub fn hour(&self) -> f32 {
        (self.minutes % 1440.0) / 60.0
    }

    /// "08:30"
    pub fn time_text(&self) -> String {
        let m = (self.minutes % 1440.0) as u32;
        format!("{:02}:{:02}", m / 60, m % 60)
    }
}

#[derive(Component)]
struct Sun;

fn spawn_sun(mut commands: Commands) {
    commands.insert_resource(GameClock { minutes: 8.0 * 60.0, fast: false });
    commands.spawn((
        DirectionalLight { shadow_maps_enabled: true, ..default() },
        // One cascade is enough: the camera never sees far.
        CascadeShadowConfigBuilder { num_cascades: 1, maximum_distance: 90.0, first_cascade_far_bound: 90.0, ..default() }.build(),
        Transform::default(),
        Sun,
        DespawnOnExit(Screen::Game),
    ));
}

fn tick(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut clock: ResMut<GameClock>) {
    if keys.just_pressed(KeyCode::KeyT) {
        clock.fast = !clock.fast;
    }
    let speed = if clock.fast { FAST } else { 1.0 };
    clock.minutes += time.delta_secs() * speed / SECS_PER_MINUTE;
}

/// 0 at night, 1 at noon, smooth through dawn and dusk.
fn daylight(hour: f32) -> f32 {
    // Sun is up from 06:00 to 20:00.
    let t = (hour - 6.0) / 14.0;
    if (0.0..=1.0).contains(&t) { (t * PI).sin() } else { 0.0 }
}

fn mix(a: Color, b: Color, t: f32) -> Color {
    let (a, b) = (a.to_linear(), b.to_linear());
    Color::LinearRgba(a.mix(&b, t.clamp(0.0, 1.0)))
}

fn light_the_world(
    clock: Res<GameClock>,
    mut ambient: ResMut<GlobalAmbientLight>,
    sun: Single<(&mut DirectionalLight, &mut Transform), With<Sun>>,
    camera: Single<&mut Camera, With<WorldCamera>>,
    here: Option<Res<Here>>,
    glow: Res<LampGlow>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lamps: Query<&mut PointLight, With<Lamp>>,
) {
    let hour = clock.hour();
    let day = daylight(hour);

    // The sun crosses from east (+X) through south (+Z) to west (-X).
    let t = ((hour - 6.0) / 14.0).clamp(0.0, 1.0);
    let elevation = (55.0_f32.to_radians() * (t * PI).sin()).max(8.0_f32.to_radians());
    let azimuth = t * PI;
    let towards_sun = Vec3::new(azimuth.cos() * elevation.cos(), elevation.sin(), azimuth.sin() * elevation.cos());
    let (mut light, mut transform) = sun.into_inner();
    *transform = Transform::default().looking_to(-towards_sun, Vec3::Y);
    light.illuminance = 9_000.0 * day;
    // Warm low sun, white high sun.
    light.color = mix(Color::srgb_u8(0xFF, 0xB0, 0x70), Color::srgb_u8(0xFF, 0xF6, 0xE8), day * 1.6);

    let night = Color::srgb_u8(0x50, 0x60, 0xA0);
    let noon = Color::srgb_u8(0xD8, 0xE4, 0xF0);
    ambient.color = mix(night, noon, day * 1.4);
    ambient.brightness = 260.0 + 360.0 * day;

    // Dusk colours only show while the sun is low; rooms sit on a dark backdrop instead.
    let sky_night = Color::srgb_u8(0x1E, 0x24, 0x40);
    let sky_dusk = Color::srgb_u8(0xE8, 0xA8, 0x6A);
    let sky_day = Color::srgb_u8(0x9F, 0xC7, 0xD8);
    let sky = if day < 0.15 { mix(sky_night, sky_dusk, day / 0.15) } else { mix(sky_dusk, sky_day, (day - 0.15) / 0.2) };
    let indoors = here.is_some_and(|h| h.0 != Area::Street);
    let backdrop = if indoors { Color::srgb_u8(0x2A, 0x24, 0x22) } else { sky };
    camera.into_inner().clear_color = ClearColorConfig::Custom(backdrop);

    let lamp_on = (0.25 - day).clamp(0.0, 0.25) / 0.25;
    for mut lamp in &mut lamps {
        lamp.intensity = 160_000.0 * lamp_on;
    }
    // Only touch the material when the glow changes; every change re-uploads it.
    let emissive = LinearRgba::rgb(300.0, 230.0, 120.0) * lamp_on;
    if materials.get(&glow.0).is_some_and(|m| m.emissive != emissive)
        && let Some(mut material) = materials.get_mut(&glow.0)
    {
        material.emissive = emissive;
    }
}
