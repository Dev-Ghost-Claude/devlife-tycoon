//! "Made with Claude" splash: Clawd jumps twice above the credit, then the screen fades out.
//! Any key or click skips ahead to the fade.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    sprite::Anchor,
};

use crate::{ACCENT, BACKGROUND, INK, MUTED, Screen};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Splash), spawn_splash).add_systems(
            Update,
            (tick, skip, animate_clawd, fade, finish)
                .chain()
                .run_if(in_state(Screen::Splash)),
        );
    }
}

// One jump cycle, frame by frame, with the same timings as art/clawd_pixel.lua:
// idle, crouch, launch, rise, apex, fall, land, settle.
const FRAME_SECS: [f32; 8] = [0.30, 0.12, 0.10, 0.16, 0.20, 0.16, 0.12, 0.14];
const CYCLE: f32 = 1.30;
const TAKEOFF: f32 = 0.42; // "launch" starts
const TOUCHDOWN: f32 = 1.04; // "fall" ends
const JUMP_HEIGHT: f32 = 90.0;

// Timeline in seconds: fade in, two jumps, a short rest, fade out.
const FADE_IN: f32 = 0.5;
const FADE_OUT_AT: f32 = 2.0 * CYCLE + 0.3;
const FADE_OUT: f32 = 0.5;

// Layout: Clawd stands on the ground line, the credit sits below it.
const GROUND_Y: f32 = 20.0;
const TITLE_Y: f32 = -60.0;
const SUBTITLE_Y: f32 = -108.0;
const PIXEL_SCALE: f32 = 10.0;

#[derive(Resource, Default)]
struct SplashClock {
    t: f32,
    fade_out_at: f32,
}

#[derive(Component)]
struct Clawd;

#[derive(Component)]
struct ClawdShadow;

#[derive(Component)]
struct FadeOverlay;

fn spawn_splash(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(SplashClock { t: 0.0, fade_out_at: FADE_OUT_AT });

    // Each frame of the sheet holds its own squash and stretch; nearest sampling keeps pixels sharp.
    let image = assets
        .load_builder()
        .with_settings(|s: &mut ImageLoaderSettings| s.sampler = ImageSampler::nearest())
        .load("clawd/pixel_sheet.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(24), 8, 1, None, None));
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(24.0 * PIXEL_SCALE)),
            ..Sprite::from_atlas_image(image, TextureAtlas { layout, index: 0 })
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(0.0, GROUND_Y, 2.0),
        Clawd,
        DespawnOnExit(Screen::Splash),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Ellipse::new(72.0, 9.0))),
        MeshMaterial2d(materials.add(Color::srgba_u8(0x6B, 0x4A, 0x3A, 46))),
        Transform::from_xyz(0.0, GROUND_Y, 1.0),
        ClawdShadow,
        DespawnOnExit(Screen::Splash),
    ));

    let semibold = assets.load("fonts/SourceSans3-Semibold.ttf");
    let bold = assets.load("fonts/SourceSans3-Bold.ttf");
    commands.spawn((
        Text2d::new("Made with "),
        TextFont { font: semibold.clone().into(), font_size: FontSize::Px(48.0), ..default() },
        TextColor(INK),
        Transform::from_xyz(0.0, TITLE_Y, 2.0),
        DespawnOnExit(Screen::Splash),
        children![(
            TextSpan::new("Claude"),
            TextFont { font: bold.into(), font_size: FontSize::Px(48.0), ..default() },
            TextColor(ACCENT),
        )],
    ));
    commands.spawn((
        Text2d::new("Fable and Opus"),
        TextFont { font: semibold.into(), font_size: FontSize::Px(24.0), ..default() },
        TextColor(MUTED),
        Transform::from_xyz(0.0, SUBTITLE_Y, 2.0),
        DespawnOnExit(Screen::Splash),
    ));

    // Covers everything in the background colour; its alpha does both fades.
    commands.spawn((
        Sprite::from_color(BACKGROUND, Vec2::splat(10_000.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
        FadeOverlay,
        DespawnOnExit(Screen::Splash),
    ));
}

fn tick(time: Res<Time>, mut clock: ResMut<SplashClock>) {
    // Clamp so a slow first frame (shaders, asset loads) does not eat the fade-in.
    clock.t += time.delta_secs().min(1.0 / 30.0);
}

fn skip(keys: Res<ButtonInput<KeyCode>>, mouse: Res<ButtonInput<MouseButton>>, mut clock: ResMut<SplashClock>) {
    if keys.get_just_pressed().next().is_some() || mouse.get_just_pressed().next().is_some() {
        clock.fade_out_at = clock.fade_out_at.min(clock.t);
    }
}

fn animate_clawd(
    clock: Res<SplashClock>,
    clawd: Single<(&mut Sprite, &mut Transform), With<Clawd>>,
    shadow: Single<&mut Transform, (With<ClawdShadow>, Without<Clawd>)>,
) {
    let phase = clock.t % CYCLE;
    let mut frame = FRAME_SECS.len() - 1;
    let mut start = 0.0;
    for (i, secs) in FRAME_SECS.iter().enumerate() {
        if phase < start + secs {
            frame = i;
            break;
        }
        start += secs;
    }

    let height = if (TAKEOFF..TOUCHDOWN).contains(&phase) {
        let u = (phase - TAKEOFF) / (TOUCHDOWN - TAKEOFF);
        4.0 * u * (1.0 - u) * JUMP_HEIGHT
    } else {
        0.0
    };

    let (mut sprite, mut transform) = clawd.into_inner();
    transform.translation.y = GROUND_Y + height;
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = frame;
    }

    // The shadow shrinks as Clawd rises.
    let lift = height / JUMP_HEIGHT;
    shadow.into_inner().scale = Vec3::new(1.0 - 0.45 * lift, 1.0 - 0.3 * lift, 1.0);
}

fn fade(clock: Res<SplashClock>, overlay: Single<&mut Sprite, With<FadeOverlay>>) {
    let fading_in = 1.0 - clock.t / FADE_IN;
    let fading_out = (clock.t - clock.fade_out_at) / FADE_OUT;
    let alpha = fading_in.max(fading_out).clamp(0.0, 1.0);
    overlay.into_inner().color.set_alpha(alpha);
}

fn finish(clock: Res<SplashClock>, mut next: ResMut<NextState<Screen>>) {
    if clock.t >= clock.fade_out_at + FADE_OUT {
        next.set(Screen::Game);
    }
}
