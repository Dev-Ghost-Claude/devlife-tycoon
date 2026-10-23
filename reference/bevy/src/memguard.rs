//! The memory cap. On 2026-10-13 a runaway capture run grew to 4.4 GB, hung the GPU and
//! froze the whole Mac. A watchdog on its own thread (so it keeps working when the game's
//! loops are stuck) checks the game's memory four times a second:
//!
//! - over the soft limit (1.5 GB): the game goes easy, with smaller shadow maps and slower
//!   frames;
//! - over the hard limit (2.5 GB): quit at once;
//! - 400 MB more within 2 seconds, once loaded and settled: quit at once (a runaway, not
//!   a scene);
//! - the GPU not finishing frames for 6 seconds: quit at once.
//!
//! Quitting skips all clean-up: with the GPU stuck, clean-up can hang too, and the system
//! frees everything anyway. The reason is written to `target/memguard.log` (or the temp
//! folder outside the project). `DEVLIFE_MEM_SOFT_MB`, `DEVLIFE_MEM_HARD_MB` and
//! `DEVLIFE_MEM_RUNAWAY_MB` change the limits, for testing the cap.

use std::{
    io::Write,
    path::PathBuf,
    sync::{
        OnceLock,
        atomic::{AtomicBool, AtomicU32, Ordering::Relaxed},
    },
    time::Duration,
};

use bevy::{
    light::{DirectionalLightShadowMap, PointLightShadowMap},
    prelude::*,
    window::WindowOccluded,
    winit::{UpdateMode, WinitSettings},
};

const SOFT_MB: u64 = 1536;
const HARD_MB: u64 = 2560;
/// Growth this fast after loading is a runaway.
const RUNAWAY_MB: u64 = 400;
const SAMPLE: Duration = Duration::from_millis(250);
/// Samples in the runaway window: 2 seconds.
const WINDOW: usize = 8;
/// The runaway check starts this long after loading: the first second or so in a scene
/// can legitimately add hundreds of MB (seen once at 700 MB, likely the system compiling
/// shaders it had not cached), and a real runaway goes on for minutes.
const SETTLE: Duration = Duration::from_secs(5);
/// `wait_for_gpu` waits 2 seconds a frame; this many timeouts in a row means a stuck GPU.
const GPU_STUCK: u32 = 3;

/// Set by the watchdog when memory passes the soft limit.
static SOFT_LIMIT_HIT: AtomicBool = AtomicBool::new(false);
/// Set once the scene is built and on the GPU: from then on, fast growth is a runaway.
static LOADED: AtomicBool = AtomicBool::new(false);
/// Set when memory legitimately jumps (the window resized): the runaway check starts over.
static REBASE: AtomicBool = AtomicBool::new(false);
/// `wait_for_gpu` timeouts in a row.
static GPU_TIMEOUTS: AtomicU32 = AtomicU32::new(0);
static LOG: OnceLock<PathBuf> = OnceLock::new();
/// A file to delete when quitting on the spot (the capture lock).
static REMOVE_ON_QUIT: OnceLock<PathBuf> = OnceLock::new();

/// This process's memory as macOS counts it (Activity Monitor's "Memory"), GPU memory
/// included, in MB. One cheap system call.
pub fn footprint_mb() -> Option<u64> {
    #[cfg(target_os = "macos")]
    {
        let mut info: libc::rusage_info_v4 = unsafe { std::mem::zeroed() };
        // SAFETY: `info` is a `rusage_info_v4`, the flavour asked for.
        let ok = unsafe { libc::proc_pid_rusage(std::process::id() as i32, libc::RUSAGE_INFO_V4, (&raw mut info).cast()) } == 0;
        ok.then_some(info.ri_phys_footprint >> 20)
    }
    #[cfg(not(target_os = "macos"))]
    None
}

/// Starts the watchdog. Call first thing in `main`.
pub fn start() {
    let limit = |name: &str, default: u64| std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default);
    let (soft, hard, runaway) = (limit("DEVLIFE_MEM_SOFT_MB", SOFT_MB), limit("DEVLIFE_MEM_HARD_MB", HARD_MB), limit("DEVLIFE_MEM_RUNAWAY_MB", RUNAWAY_MB));
    let log = if std::path::Path::new("target").is_dir() { PathBuf::from("target/memguard.log") } else { std::env::temp_dir().join("devlife-memguard.log") };
    let _ = LOG.set(log);
    let spawned = std::thread::Builder::new().name("memguard".into()).spawn(move || {
        let mut recent = [0u64; WINDOW];
        let mut armed_at = None;
        for i in 0usize.. {
            std::thread::sleep(SAMPLE);
            if GPU_TIMEOUTS.load(Relaxed) >= GPU_STUCK {
                quit_now(86, &format!("the GPU has not finished a frame for {} s", GPU_STUCK * 2));
            }
            let Some(mb) = footprint_mb() else { continue };
            if mb >= hard {
                quit_now(86, &format!("memory {mb} MB, over the hard limit of {hard} MB"));
            }
            if mb >= soft && !SOFT_LIMIT_HIT.swap(true, Relaxed) {
                eprintln!("memguard: memory {mb} MB, over the soft limit of {soft} MB; going easy");
            }
            if LOADED.load(Relaxed) && armed_at.is_none() {
                armed_at = Some(std::time::Instant::now() + SETTLE);
            }
            let armed = armed_at.is_some_and(|at| std::time::Instant::now() >= at);
            if REBASE.swap(false, Relaxed) || !armed {
                recent = [mb; WINDOW];
            }
            let two_seconds_ago = std::mem::replace(&mut recent[i % WINDOW], mb);
            if mb > two_seconds_ago + runaway {
                quit_now(86, &format!("memory grew from {two_seconds_ago} to {mb} MB in 2 s"));
            }
        }
    });
    if let Err(error) = spawned {
        eprintln!("memguard: couldn't start the watchdog ({error})");
    }
}

/// Quits on the spot, skipping all clean-up, after noting why in the log.
pub fn quit_now(code: i32, why: &str) -> ! {
    if let Some(path) = LOG.get()
        && let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path)
    {
        let _ = writeln!(file, "memguard: quit ({why})");
    }
    exit_now(code)
}

/// Deletes `path` when quitting on the spot (clean-up code never runs then).
pub fn remove_on_quit(path: PathBuf) {
    let _ = REMOVE_ON_QUIT.set(path);
}

/// Ends the process without clean-up: the system reclaims all of it, GPU memory included,
/// and nothing can hang on the way out.
pub fn exit_now(code: i32) -> ! {
    if let Some(path) = REMOVE_ON_QUIT.get() {
        let _ = std::fs::remove_file(path);
    }
    // SAFETY: `_exit` ends the process; nothing runs after it.
    unsafe { libc::_exit(code) }
}

/// Called after each frame's wait for the GPU, with whether the GPU finished in time.
pub fn gpu_waited(finished: bool) {
    if finished {
        GPU_TIMEOUTS.store(0, Relaxed);
    } else {
        GPU_TIMEOUTS.fetch_add(1, Relaxed);
    }
}

/// Memory jumped for a good reason (the window resized): don't count it as a runaway.
pub fn rebase() {
    REBASE.store(true, Relaxed);
}

pub struct MemGuardPlugin;

impl Plugin for MemGuardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mark_loaded.run_if(in_state(crate::Screen::Game)), go_easy, pace_while_hidden));
    }
}

/// The scene is built on entering the game and reaches the GPU over the next few frames.
fn mark_loaded(mut frames: Local<u32>) {
    *frames += 1;
    if *frames == 30 {
        LOADED.store(true, Relaxed);
    }
}

/// Over the soft limit: smaller shadow maps (the biggest textures) and slower frames, so
/// the GPU has less to hold and can never fall behind.
fn go_easy(mut done: Local<bool>, mut sun: ResMut<DirectionalLightShadowMap>, mut lamps: ResMut<PointLightShadowMap>, mut winit: ResMut<WinitSettings>) {
    if *done || !SOFT_LIMIT_HIT.load(Relaxed) {
        return;
    }
    *done = true;
    sun.size = sun.size.min(1024);
    lamps.size = lamps.size.min(512);
    (winit.focused_mode, winit.unfocused_mode) = (easy(), easy());
    warn!("memguard: over the soft memory limit: smaller shadow maps, slower frames");
}

/// While the window is fully covered (screen locked, another Space, minimised), nothing
/// paces frames, so the GPU would run flat out for nobody: slow right down until it shows.
fn pace_while_hidden(mut occluded: MessageReader<WindowOccluded>, mut winit: ResMut<WinitSettings>, mut normal: Local<Option<(UpdateMode, UpdateMode)>>) {
    for event in occluded.read() {
        if event.occluded && normal.is_none() {
            *normal = Some((winit.focused_mode, winit.unfocused_mode));
            let idle = UpdateMode::reactive_low_power(Duration::from_millis(100));
            (winit.focused_mode, winit.unfocused_mode) = (idle, idle);
        } else if !event.occluded
            && let Some(modes) = normal.take()
        {
            (winit.focused_mode, winit.unfocused_mode) = if SOFT_LIMIT_HIT.load(Relaxed) { (easy(), easy()) } else { modes };
        }
    }
}

/// About 30 frames a second, and only as needed.
fn easy() -> UpdateMode {
    UpdateMode::reactive_low_power(Duration::from_millis(33))
}
