# DevLife Tycoon

A game-development tycoon with extreme depth. You live one whole life, from a teenager
with a weak laptop in the late 1970s to the head of a studio, through every era of the
games industry. It is hard in the way Dark Souls is hard: punishing, and rewarding.

The world is isometric, built from voxels and drawn as crisp pixel art. The cast are
animals.

**Status:** early. The foundation runs: a street and a computer shop to walk around in,
with day and night. The design lives in [design/GAME_DESIGN.md](design/GAME_DESIGN.md).
Some parts of the design are private and are not in this repository.

## Run it

You need [Rust](https://rustup.rs). The repository pins the Rust version it needs
(`rust-toolchain.toml`), and `rustup` installs it on the first build.

```bash
cargo run
```

| Input | Action |
|---|---|
| Click | Walk there, or use what you clicked |
| Q / E | Turn the street view |
| Mouse wheel | Zoom in or out |
| T | Fast-forward time |

## Who made it

Designed by Positively Ghost. Built with Claude (Fable and Opus, AI models made by
Anthropic): code, art and engineering.

## Licence

[PolyForm Noncommercial 1.0.0](LICENSE.md). You may read, run, change and share this
project for non-commercial purposes, and you must keep the `Required Notice` lines in
[NOTICE](NOTICE), which credit both of us. Commercial use needs permission.

The fonts, the Clawd credit art and the Rust crates keep their own terms: see
[NOTICE](NOTICE).
