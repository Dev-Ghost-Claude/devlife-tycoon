mod animal;
mod capture;
mod clock;
mod gameplay;
mod hud;
mod pixel_camera;
mod splash;
mod voxel;
mod world;

use bevy::{prelude::*, ui::IsDefaultUiCamera, window::WindowLevel};

/// The screens of the game, in the order the player meets them.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Screen {
    #[default]
    Splash,
    Game,
}

/// Warm cream background shared by every screen.
pub const BACKGROUND: Color = Color::srgb_u8(0xF5, 0xF1, 0xEA);
/// Main text: warm near-black.
pub const INK: Color = Color::srgb_u8(0x2B, 0x24, 0x20);
/// Secondary text: warm grey, 4.9:1 on the background.
pub const MUTED: Color = Color::srgb_u8(0x7A, 0x6E, 0x64);
/// Claude terracotta, darkened for text contrast on the background.
pub const ACCENT: Color = Color::srgb_u8(0xA8, 0x4E, 0x33);

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // Screenshots of a covered window come out black, so the capture script keeps it on top.
    let capturing = args.iter().any(|a| a == "--capture");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "DevLife Tycoon".into(),
                resolution: (1280u32, 720u32).into(),
                window_level: if capturing { WindowLevel::AlwaysOnTop } else { WindowLevel::Normal },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND))
        .init_state::<Screen>()
        .add_plugins((
            splash::SplashPlugin,
            pixel_camera::PixelCameraPlugin,
            world::WorldPlugin,
            animal::AnimalPlugin,
            gameplay::GameplayPlugin,
            clock::ClockPlugin,
            hud::HudPlugin,
            capture::CapturePlugin::from_args(&args),
        ))
        .add_systems(Startup, |mut commands: Commands| {
            // Shows the splash, the scaled-up game canvas, and the interface.
            commands.spawn((Camera2d, IsDefaultUiCamera, Msaa::Off));
        })
        .run();
}
