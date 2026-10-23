mod animal;
mod bedroom;
mod camera;
mod capture;
mod clock;
mod fine_animal;
mod gallery;
mod gameplay;
mod house;
mod hud;
mod memguard;
mod outline;
mod splash;
mod street;
mod street_props;
mod voxel;
mod world;

use std::time::Duration;

use bevy::{
    prelude::*,
    ui::IsDefaultUiCamera,
    winit::{UpdateMode, WinitSettings},
};

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
    #[cfg(target_os = "macos")]
    relaunch_with_lean_malloc();
    memguard::start();
    let args: Vec<String> = std::env::args().skip(1).collect();
    // Capture runs save the world image directly, so their window stays hidden instead of
    // popping up over whatever the user is doing.
    let capturing = args.iter().any(|a| a == "--capture");
    let proof = args.iter().any(|a| a == "--proof");
    let gallery = args.iter().any(|a| a == "--gallery");

    let mut app = App::new();
    if proof {
        app.insert_resource(gameplay::ProofMode);
    }
    if gallery {
        app.insert_resource(gallery::GalleryMode);
    }
    if capturing {
        // Capture runs keep their window hidden, so nothing paces them: without this they
        // render as fast as the GPU allows. About 30 frames a second is plenty for shots.
        let paced = UpdateMode::reactive_low_power(Duration::from_millis(33));
        app.insert_resource(WinitSettings { focused_mode: paced, unfocused_mode: paced });
    }
    if args.iter().any(|a| a == "--no-outline") {
        app.insert_resource(camera::NoOutline);
    }
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "DevLife Tycoon".into(),
                resolution: (1280u32, 720u32).into(),
                visible: !capturing,
                focused: !capturing,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND))
        // `--proof` skips the splash and starts in the bedroom as the fine-detail fox.
        .insert_state(if proof || gallery { Screen::Game } else { Screen::Splash })
        .add_plugins((
            splash::SplashPlugin,
            camera::CameraPlugin,
            outline::OutlinePlugin,
            world::WorldPlugin,
            animal::AnimalPlugin,
            gameplay::GameplayPlugin,
            clock::ClockPlugin,
            hud::HudPlugin,
            capture::CapturePlugin::from_args(&args),
            gallery::GalleryPlugin,
            memguard::MemGuardPlugin,
        ))
        .add_systems(
            OnEnter(Screen::Game),
            fine_animal::gallery_lineup.after(world::build_areas).run_if(resource_exists::<gallery::GalleryMode>),
        )
        .add_systems(Update, fine_animal::hold_gallery_faces.run_if(resource_exists::<gallery::GalleryMode>))
        .add_systems(Startup, |mut commands: Commands| {
            // Shows the splash, the scaled-up game canvas, and the interface.
            commands.spawn((Camera2d, IsDefaultUiCamera, Msaa::Off));
        })
        .run();
}

/// macOS's allocator keeps the large blocks the game frees (the voxel grids the scene is
/// built from, meshes on their way to the GPU: 260 MB) and goes on counting them against
/// the game; asking it to release them does nothing. With `MallocLargeCache=0` it hands
/// them straight back. The setting is read when a process starts, so the game restarts
/// itself once with it, before anything else happens.
#[cfg(target_os = "macos")]
fn relaunch_with_lean_malloc() {
    use std::os::unix::process::CommandExt;
    if std::env::var_os("MallocLargeCache").is_some() {
        return;
    }
    let Ok(exe) = std::env::current_exe() else { return };
    // `exec` only returns if the restart failed; then carry on as we are.
    let error = std::process::Command::new(exe).args(std::env::args_os().skip(1)).env("MallocLargeCache", "0").exec();
    eprintln!("couldn't restart with MallocLargeCache=0 ({error}); freed memory will stay counted");
}
