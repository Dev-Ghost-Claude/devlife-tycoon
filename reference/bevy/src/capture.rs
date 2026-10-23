//! `--capture DIR`: play a fixed script, save screenshots along the way, then quit. For
//! checking the game without watching it, e.g. `cargo run -- --capture target/shots`.

use std::{
    path::PathBuf,
    sync::{
        OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
};

use crate::{
    clock::GameClock,
    gameplay::{ProofMode, Scripted},
    camera::CanvasImage,
};

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
    /// Hold the camera at this zoom (larger shows more), whatever the area would choose.
    Zoom(f32),
}

/// Seconds after start-up, and what to do. The splash ends at about 3.4 s; the first street
/// shot waits for the renderer to finish compiling its shaders.
const SCRIPT: &[(f32, Step)] = &[
    (0.95, Step::Shot("splash")),
    (5.0, Step::Shot("street-morning")),
    // Click the flats' wall (real picking): walks to their door, which is locked.
    (5.1, Step::ClickPixel(225.0, 60.0)),
    (10.5, Step::Shot("flats-door")),
    // Walk to the computer shop's door and go in.
    (10.6, Step::Click(29, 9)),
    (22.0, Step::Shot("shop-entering")),
    (23.5, Step::Shot("shop-inside")),
    // The shopkeeper, a shelf, then the door in the west wall.
    (23.6, Step::Click(8, 4)),
    (29.5, Step::Shot("shopkeeper")),
    (29.6, Step::Click(3, 0)),
    (34.5, Step::Shot("shelf")),
    (34.6, Step::Click(0, 4)),
    (39.5, Step::Shot("street-again")),
    (39.6, Step::Hour(21.5)),
    (41.0, Step::Shot("street-night")),
    (41.1, Step::Turn(-1)),
    (43.0, Step::Shot("street-turned")),
    (43.1, Step::Turn(1)),
    // Walk home: the house door leads up to the bedroom.
    (44.5, Step::Click(17, 6)),
    (54.0, Step::Shot("bedroom-from-street")),
];

/// `--proof --capture DIR`: the bedroom by day, at night with the lamp on, and at the desk.
const PROOF_SCRIPT: &[(f32, Step)] = &[
    (2.5, Step::Shot("bedroom-day")),
    (2.6, Step::Click(3, 0)),
    (6.0, Step::Shot("bedroom-at-desk")),
    (6.1, Step::Hour(21.5)),
    (7.5, Step::Shot("bedroom-night")),
    // Pulled back: the bedroom in its house, the street around it.
    (7.6, Step::Zoom(1.9)),
    (9.5, Step::Shot("bedroom-wide-night")),
    (9.6, Step::Hour(10.0)),
    (11.0, Step::Shot("bedroom-wide-day")),
];

/// `--gallery --capture DIR`: the street models from three sides, then at night. The
/// first shot waits until the renderer has finished compiling its shaders (earlier shots
/// can come out with only the outlines drawn).
const GALLERY_SCRIPT: &[(f32, Step)] = &[
    (5.0, Step::Shot("gallery-day")),
    (5.1, Step::Turn(1)),
    (6.6, Step::Shot("gallery-turned")),
    (6.7, Step::Turn(1)),
    (8.2, Step::Shot("gallery-back")),
    (8.3, Step::Turn(-2)),
    (8.4, Step::Hour(21.5)),
    (10.0, Step::Shot("gallery-night")),
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
            let lock = CaptureLock::claim();
            start_watchdog();
            app.insert_resource(Capture { dir: dir.clone(), t: 0.0, next: 0 }).insert_resource(lock).add_systems(Update, run_script);
        }
    }
}

// Safety. On 2026-10-13 a capture run hung the GPU (two runs were going at once, rendering
// flat out with their windows hidden) and froze the whole Mac until a forced restart. So:
// only one capture runs at a time, captures are paced (see `main.rs`), and a watchdog
// kills a run that stops making progress or runs far too long.

/// When the run started, and when the script last made progress (milliseconds after that).
static START: OnceLock<Instant> = OnceLock::new();
static HEARTBEAT: AtomicU64 = AtomicU64::new(0);
/// No progress for this long means the run is stuck (most likely the GPU hung): quit.
const STUCK: Duration = Duration::from_secs(30);
/// No capture script is anywhere near this long.
const LONGEST: Duration = Duration::from_secs(150);
const LOCK_FILE: &str = "target/capture.lock";
/// Screenshots asked for, and written to disk, so far.
static SHOTS_TAKEN: AtomicU64 = AtomicU64::new(0);
static SHOTS_SAVED: AtomicU64 = AtomicU64::new(0);

fn start_watchdog() {
    let start = *START.get_or_init(Instant::now);
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(1));
            let now = start.elapsed();
            let last = Duration::from_millis(HEARTBEAT.load(Ordering::Relaxed));
            if now > LONGEST || now.saturating_sub(last) > STUCK {
                // Without clean-up: with the GPU stuck, clean-up can hang too.
                crate::memguard::quit_now(3, &format!("capture stuck or too long ({:.0} s, last progress at {:.0} s)", now.as_secs_f32(), last.as_secs_f32()));
            }
        }
    });
}

/// Held while a capture runs: `target/capture.lock` with this process's id. A second
/// capture refuses to start while the first is alive.
#[derive(Resource)]
struct CaptureLock(PathBuf);

impl CaptureLock {
    fn claim() -> Self {
        let path = PathBuf::from(LOCK_FILE);
        if let Ok(pid) = std::fs::read_to_string(&path) {
            let alive = std::process::Command::new("kill").args(["-0", pid.trim()]).status().is_ok_and(|s| s.success());
            if alive {
                eprintln!("capture: another capture (process {}) is still running; run one at a time", pid.trim());
                std::process::exit(2);
            }
        }
        let _ = std::fs::write(&path, std::process::id().to_string());
        // Captures end by quitting on the spot, so the lock is deleted then, not on drop.
        crate::memguard::remove_on_quit(path.clone());
        Self(path)
    }
}

impl Drop for CaptureLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn run_script(
    time: Res<Time>,
    mut capture: ResMut<Capture>,
    mut commands: Commands,
    mut scripted: Option<ResMut<Scripted>>,
    mut clock: Option<ResMut<GameClock>>,
    canvas: Option<Res<CanvasImage>>,
    proof: Option<Res<ProofMode>>,
    gallery: Option<Res<crate::gallery::GalleryMode>>,
) {
    if let Some(start) = START.get() {
        HEARTBEAT.store(start.elapsed().as_millis() as u64, Ordering::Relaxed);
    }
    // Same clamp as the splash clock, so the script lines up with the splash timeline.
    capture.t += time.delta_secs().min(1.0 / 30.0);
    let script = if gallery.is_some() {
        GALLERY_SCRIPT
    } else if proof.is_some() {
        PROOF_SCRIPT
    } else {
        SCRIPT
    };
    while let Some((at, step)) = script.get(capture.next) {
        if capture.t < *at {
            return;
        }
        match step {
            Step::Shot(name) => {
                // The window is hidden during captures, so shoot the canvas: the world as
                // rendered, without the interface.
                if let Some(canvas) = &canvas {
                    let path = capture.dir.join(format!("{:05.2}-{name}-canvas.png", at));
                    SHOTS_TAKEN.fetch_add(1, Ordering::Relaxed);
                    let mut save = save_to_disk(path);
                    commands.spawn(Screenshot::image(canvas.0.clone())).observe(move |shot: On<ScreenshotCaptured>| {
                        save(shot);
                        SHOTS_SAVED.fetch_add(1, Ordering::Relaxed);
                    });
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
            Step::Zoom(z) => {
                commands.insert_resource(crate::gameplay::ZoomOverride(*z));
            }
            Step::Turn(q) => {
                if let Some(s) = scripted.as_mut() {
                    s.turn += q;
                }
            }
        }
        capture.next += 1;
    }
    // Quit once every screenshot is read back and saved (or after 5 s, if one never is).
    // Straight out, without clean-up: shutting the renderer down once hung for 30 s.
    let last = script[script.len() - 1].0;
    let shots = SHOTS_TAKEN.load(Ordering::Relaxed);
    let saved = SHOTS_SAVED.load(Ordering::Relaxed);
    if (capture.t >= last + 0.5 && saved >= shots) || capture.t >= last + 5.0 {
        if saved < shots {
            eprintln!("capture: only {saved} of {shots} screenshots were saved");
        }
        crate::memguard::exit_now(if saved >= shots { 0 } else { 4 });
    }
}
