"""Runs the game under an outer memory and time guard, for capture runs.

The game has its own memory cap (src/memguard.rs); this is the second layer, outside the
process: it kills the game's whole process group if its memory footprint passes LIMIT_MB
or the run passes BUDGET_S, even if the game itself is stuck. It also logs the footprint
every quarter second, the peak, and full `footprint` tables, into OUT_DIR. Build first, so
the budget times the run and not the compiler:

    cargo build && python3 -I tools/guard.py 2500 150 /tmp/run -- target/debug/bevy-game --capture target/shots

Options (environment): GUARD_TABLES="5,8" takes `footprint` tables at those seconds;
GUARD_SAMPLE_AT=4.6 records a 1 s call-stack sample then; GUARD_NO_ASSETS=1 runs without
the assets folder. DEVLIFE_MEMLOG=1 is always set, so the game logs memory as it builds.
"""
import ctypes, os, signal, struct, subprocess, sys, time

limit_mb, budget, out = int(sys.argv[1]), float(sys.argv[2]), sys.argv[3]
cmd = sys.argv[sys.argv.index("--") + 1:]
lib = ctypes.CDLL("/usr/lib/libSystem.B.dylib")


def footprint_mb(pid):
    buf = ctypes.create_string_buffer(512)
    # RUSAGE_INFO_V4; ri_phys_footprint sits at byte 72.
    if lib.proc_pid_rusage(pid, 4, buf) != 0:
        return None
    return struct.unpack_from("<Q", buf, 72)[0] / 2**20


log = open(os.path.join(out, "guard.log"), "w")
game_log = open(os.path.join(out, "game.log"), "w")
env = dict(os.environ, DEVLIFE_MEMLOG="1")
if not os.environ.get("GUARD_NO_ASSETS"):
    env["BEVY_ASSET_ROOT"] = os.getcwd()
p = subprocess.Popen(cmd, stdout=game_log, stderr=subprocess.STDOUT, env=env, start_new_session=True)
t0 = time.monotonic()
peak, tables, verdict = 0.0, set(float(x) for x in os.environ.get("GUARD_TABLES", "8,14").split(",")), "finished"
while p.poll() is None:
    t = time.monotonic() - t0
    mb = footprint_mb(p.pid)
    if mb is not None:
        peak = max(peak, mb)
        print(f"t={t:5.2f}s {mb:7.0f} MB", file=log, flush=True)
        if mb > limit_mb:
            verdict = f"KILLED: {mb:.0f} MB over the {limit_mb} MB limit"
    if t > budget:
        verdict = f"KILLED: over {budget:.0f} s"
    if verdict != "finished":
        os.killpg(p.pid, signal.SIGKILL)
        break
    if os.environ.get("GUARD_SAMPLE_AT") and t >= float(os.environ["GUARD_SAMPLE_AT"]) and not os.path.exists(os.path.join(out, "sample.txt")):
        subprocess.run(["/usr/bin/sample", str(p.pid), "1", "-mayDie", "-file", os.path.join(out, "sample.txt")], capture_output=True)
    for at in sorted(tables):
        if t >= at:
            tables.discard(at)
            table = subprocess.run(["/usr/bin/footprint", "-p", str(p.pid)], capture_output=True, text=True).stdout
            open(os.path.join(out, f"footprint.{at}s.txt"), "w").write(table)
    time.sleep(0.25)
code = p.wait()
print(f"{verdict}; exit {code}; peak {peak:.0f} MB", file=log, flush=True)
print(f"{verdict}; exit {code}; peak {peak:.0f} MB")
