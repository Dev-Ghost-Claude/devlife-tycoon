//! `--capture DIR`: play a fixed script, save screenshots along the way, then quit. For
//! checking the game without watching it, e.g. `cargo run -- --capture target/shots`.

use std::path::PathBuf;

use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};

use crate::{clock::GameClock, gameplay::Scripted, pixel_camera::CanvasImage};

enum Step {
    Shot(&'static str),
    /// Click a tile in the current area.
    Click(i32, i32),
    /// Click a pixel of the canvas, as in the `-canvas.png` shots: tests the real picking.
    ClickPixel(f32, f32),
    /// Jump the clock to this hour of the same day.
    Hour(f32),
    /// Quarter turns of the street view.
    Turn(i32),
}

/// Seconds after start-up, and what to do. The splash ends at about 3.4 s.
const SCRIPT: &[(f32, Step)] = &[
    (0.95, Step::Shot("splash")),
    (4.2, Step::Shot("street-morning")),
    // Click the house wall: walks to its door and says it's not your house.
    (4.3, Step::ClickPixel(150.0, 40.0)),
    (8.0, Step::Shot("house-door")),
    (8.1, Step::Click(3, 5)),
    (13.6, Step::Shot("shop-entering")),
    (15.0, Step::Shot("shop-inside")),
    // The shopkeeper, a shelf, then the door in the west wall.
    (15.1, Step::ClickPixel(361.0, 211.0)),
    (21.0, Step::Shot("shopkeeper")),
    (21.1, Step::ClickPixel(336.0, 134.0)),
    (26.0, Step::Shot("shelf")),
    (26.1, Step::ClickPixel(218.0, 138.0)),
    (31.0, Step::Shot("street-again")),
    (31.1, Step::Hour(21.5)),
    (32.5, Step::Shot("street-night")),
    (32.6, Step::Turn(-1)),
    (34.5, Step::Shot("street-turned")),
];

pub struct CapturePlugin {
    dir: Option<PathBuf>,
}

impl CapturePlugin {
    pub fn from_args(args: &[String]) -> Self {
        let dir = args.iter().position(|a| a == "--capture").and_then(|i| args.get(i + 1)).map(PathBuf::from);
        Self { dir }
    }
}

#[derive(Resource)]
struct Capture {
    dir: PathBuf,
    t: f32,
    next: usize,
}

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Some(dir) = &self.dir {
            std::fs::create_dir_all(dir).expect("create capture folder");
            app.insert_resource(Capture { dir: dir.clone(), t: 0.0, next: 0 }).add_systems(Update, run_script);
        }
    }
}

fn run_script(
    time: Res<Time>,
    mut capture: ResMut<Capture>,
    mut commands: Commands,
    mut scripted: Option<ResMut<Scripted>>,
    mut clock: Option<ResMut<GameClock>>,
    canvas: Option<Res<CanvasImage>>,
    mut exit: MessageWriter<AppExit>,
) {
    // Same clamp as the splash clock, so the script lines up with the splash timeline.
    capture.t += time.delta_secs().min(1.0 / 30.0);
    while let Some((at, step)) = SCRIPT.get(capture.next) {
        if capture.t < *at {
            return;
        }
        match step {
            Step::Shot(name) => {
                let path = capture.dir.join(format!("{:05.2}-{name}.png", at));
                commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
                // The window shot can come out black when macOS stops drawing the window;
                // the canvas (world only, no interface) is always there.
                if let Some(canvas) = &canvas {
                    let path = capture.dir.join(format!("{:05.2}-{name}-canvas.png", at));
                    commands.spawn(Screenshot::image(canvas.0.clone())).observe(save_to_disk(path));
                }
            }
            Step::Click(x, z) => {
                if let Some(s) = scripted.as_mut() {
                    s.click = Some(IVec2::new(*x, *z));
                }
            }
            Step::ClickPixel(x, y) => {
                if let Some(s) = scripted.as_mut() {
                    s.click_canvas = Some(Vec2::new(*x, *y));
                }
            }
            Step::Hour(h) => {
                if let Some(c) = clock.as_mut() {
                    let day_start = (c.minutes / 1440.0).floor() * 1440.0;
                    c.minutes = day_start + h * 60.0;
                }
            }
            Step::Turn(q) => {
                if let Some(s) = scripted.as_mut() {
                    s.turn += q;
                }
            }
        }
        capture.next += 1;
    }
    if capture.t >= SCRIPT[SCRIPT.len() - 1].0 + 0.5 {
        exit.write(AppExit::Success);
    }
}
