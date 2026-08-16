# hacker_screen / jarvis

Two fullscreen desktop toys, no dialogs, no console window.

## hacker_screen

A fake "hacking in progress" terminal — matrix rain, a scrolling exploit log
styled like a real terminal session (`nmap`/`hydra`/`sqlmap`/etc.), a live
hex dump, and an ASCII progress bar that rolls from target to target.

```
cargo run --release --bin hacker_screen
```

- `SPACE` — fast-forward the current breach
- `ESC` — quit

## jarvis

A circular holographic HUD in the Iron Man / JARVIS vein — rotating
instrument rings, a radar sweep, a rotating 3D wireframe code orb at the
center (a sphere of shifting glyphs that breathes like it's alive),
live-drifting telemetry gauges, and an audio waveform.

```
cargo run --release --bin jarvis
```

- `SPACE` — ping the core (an expanding ripple)
- `ESC` — quit

## launcher

Starts both at once on a vertically-stacked dual-monitor setup: jarvis on
the topmost monitor, hacker_screen on the bottommost. Detects monitor
layout via the Windows API, positions each window, then exits — no window
of its own.

```
cargo run --release --bin launcher
```

Built with [macroquad](https://github.com/not-fl3/macroquad) and
[windows-rs](https://github.com/microsoft/windows-rs) (launcher only).
