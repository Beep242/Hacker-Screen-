# CLAUDE.md

## What this is

Two fullscreen Windows desktop toys plus a launcher, in one Rust crate, drawn
entirely with [macroquad](https://github.com/not-fl3/macroquad): `hacker_screen`
(fake "hacking in progress" terminal — matrix rain, exploit log, hex dump, breach bar)
and `jarvis` (circular Iron Man-style holographic HUD — rings, radar sweep, a rotating
wireframe "code orb", gauges). Every value is fake and generated at runtime; no assets,
no config files, no env vars, no network.

## Layout

One package, three binaries, **no lib crate**, ~1.8k lines total.

- `src/main.rs` + `src/{data,hex,matrix,text,ui}.rs` — `hacker_screen` and its
  private modules; `data.rs` is the word bank.
- `src/bin/jarvis/` — `main.rs` (App + frame loop), `hud.rs` (drawing primitives,
  ~400 lines, the bulk of the visual), `orb.rs` (3D core), `telemetry.rs`, `text.rs`.
- `src/bin/launcher.rs` — windows-rs only, no macroquad.

## Commands

```
cargo build --release                      # builds all three binaries
cargo run --release --bin hacker_screen
cargo run --release --bin jarvis
cargo run --release --bin launcher
```

No tests, CI, lint config, or `rust-toolchain.toml` — `edition = "2024"` needs Rust
1.85+ from whatever toolchain is ambient. Two deps only: `macroquad` (locked 0.4.16),
plus `windows` 0.58 under `cfg(windows)`.

## Architecture notes

- The two macroquad binaries share one shape: an `App` struct holding every piece of
  state and a `#[macroquad::main(window_conf)]` loop calling `app.update(dt)` then
  `app.draw()`, ESC to break. `launcher.rs` is not like this — no macroquad, no window,
  no loop, just a `fn main()` that spawns and positions. Randomness everywhere is
  `macroquad::rand::gen_range`; nothing pulls in the `rand` crate.
- `hud.rs`'s `glow_*` helpers fake bloom by stacking wider/fainter copies behind the
  crisp line. Despite hud.rs's own module doc, they are *not* used for every stroke —
  the grid, starfield, comb ring, edge-meter ticks, waveform bars, panel frames and
  triangles call macroquad `draw_*` directly. Use `glow_*` for anything that should
  read as a lit instrument line; raw calls read flat and thin.
- Only hud/ui helpers that actually draw text take `font: Option<&Font>` as the first
  param (7 of 18 `pub fn`s in `hud.rs`); the purely geometric ones take none. `ui.rs`
  also exports the `LogLine` struct, so it is not purely free fns.
- `orb.rs` has no 3D pipeline — Fibonacci-sphere points, manual pitch/yaw rotation,
  orthographic projection, painter's-algorithm depth sort.
- `hex.rs` keeps a persistent cell grid and mutates only 1-3 cells per frame on
  purpose — regenerating every frame flickers. Same idea in `telemetry.rs`: each
  `Stat` eases toward a rerolled target.
- `data.rs::fill_template` substitutes `{ip}`/`{port}`/`{file}`/`{n}` into the word
  banks; `random_log_entry` pre-tags Info/Success/Error with `[*]`/`[+]`/`[!]` but
  leaves `Command` lines bare for `main.rs` to prefix with a shell prompt.
- `launcher.rs` has no window: `EnumDisplayMonitors` → sort by `top` → spawn both `.exe`s
  from `current_exe()`'s dir → `FindWindowW` + `SetWindowPos` → exit.

## Conventions & gotchas

- **No shared library.** `src/text.rs` and `src/bin/jarvis/text.rs` are byte-identical
  copies, and `format_timestamp` is duplicated in `src/ui.rs` and `src/bin/jarvis/main.rs`.
  `src/*.rs` is reachable only from `hacker_screen` — edit both sides.
- **The launcher matches windows by exact title.** `"JARVIS"` and `"SYSTEM BREACH"`
  are hardcoded there and must keep matching each `window_conf().window_title`, or
  placement silently fails (6s timeout, no error).
- **Build all three before running the launcher** — it exec's sibling `.exe`s from its
  own directory, and `cargo run --bin launcher` builds only itself.
- **Release builds have no console:** every binary starts with
  `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`, so panics and
  `eprintln!` vanish in release. Use debug builds to diagnose.
- **Windows-only.** Fonts are read by absolute path from `C:/Windows/Fonts`
  (`consola`/`CascadiaMono`/`lucon` for hacker_screen; `bahnschrift`/`segoeui`/
  `consola` for jarvis), falling back to macroquad's default.
- Windows are `fullscreen: true, window_resizable: false` (miniquad has no borderless flag).
  `hacker_screen` re-derives its matrix/hex grids when the screen size changes;
  `jarvis/main.rs::draw` mixes screen-relative positions with hardcoded pixel offsets
  (the SYSTEM STATUS block sits at y=508), so it does not reflow.
