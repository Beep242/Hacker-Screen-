//! hacker_screen — a fake "hacking in progress" screen: matrix rain, a
//! scrolling exploit log styled like a real terminal session, a live hex
//! dump, and a progress bar that rolls from target to target.
//!
//! Controls: SPACE fast-forwards the current breach, ESC quits.

// Rust binaries default to the console subsystem on Windows, which pops up
// a terminal window alongside the graphical one when double-clicked from
// Explorer. Suppress it in release builds; keep it in debug so `cargo run`
// still shows panics/eprintln in the terminal.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;
mod hex;
mod matrix;
mod text;
mod ui;

use std::collections::VecDeque;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use data::LineKind;
use hex::HexPanel;
use matrix::MatrixRain;
use ui::LogLine;

const MAX_LOG_LINES: usize = 60;
const HEX_ROW_H: f32 = 18.0;
const HEX_COL_W: f32 = 24.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "SYSTEM BREACH".to_owned(),
        window_width: 1280,
        window_height: 760,
        // miniquad has no windowed-borderless flag on Windows; fullscreen
        // (WS_POPUP) is the only way to drop the title bar.
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

/// Try a handful of monospace fonts that ship with Windows, in order of
/// preference. Falls back to macroquad's default font if none are found.
fn load_system_font() -> Option<Font> {
    const CANDIDATES: &[&str] = &[
        "C:/Windows/Fonts/consola.ttf",
        "C:/Windows/Fonts/CascadiaMono.ttf",
        "C:/Windows/Fonts/lucon.ttf",
    ];
    for path in CANDIDATES {
        if let Ok(bytes) = std::fs::read(path) {
            if let Ok(font) = load_ttf_font_from_bytes(&bytes) {
                return Some(font);
            }
        }
    }
    None
}

struct App {
    font: Option<Font>,
    hostname: String,
    matrix: MatrixRain,
    last_w: f32,
    last_h: f32,
    log_lines: VecDeque<LogLine>,
    log_spawn_timer: f32,
    typing_timer: f32,
    hex: HexPanel,
    progress: f32,
    progress_target: String,
    progress_duration: f32,
    start_time: f64,
}

impl App {
    fn new(font: Option<Font>) -> Self {
        let w = screen_width();
        let h = screen_height();
        let (hex_rows, hex_cols) = Self::hex_grid_size(w, h);
        Self {
            font,
            hostname: data::random_hostname(),
            matrix: MatrixRain::new(w, 18.0),
            last_w: w,
            last_h: h,
            log_lines: VecDeque::new(),
            log_spawn_timer: 0.0,
            typing_timer: 0.0,
            hex: HexPanel::new(hex_rows, hex_cols),
            progress: 0.0,
            progress_target: data::random_target().to_string(),
            progress_duration: gen_range(3.0, 7.0),
            start_time: get_time(),
        }
    }

    fn hex_grid_size(screen_w: f32, screen_h: f32) -> (usize, usize) {
        let hex_w = (screen_w * 0.3).min(360.0);
        let cols = (((hex_w - 78.0) / HEX_COL_W) as usize).max(1);
        let rows = ((((screen_h - 90.0) - 70.0) / HEX_ROW_H) as usize).max(1);
        (rows, cols)
    }

    fn complete_breach(&mut self) {
        self.log_lines.push_back(LogLine {
            prefix: String::new(),
            body: format!("[+] {} compromised", self.progress_target),
            revealed: 0,
            color: Color::new(0.35, 1.0, 0.65, 1.0),
        });
        if self.log_lines.len() > MAX_LOG_LINES {
            self.log_lines.pop_front();
        }
        self.progress = 0.0;
        self.progress_target = data::random_target().to_string();
        self.progress_duration = gen_range(3.0, 7.0);
    }

    fn update(&mut self, dt: f32) {
        let w = screen_width();
        let h = screen_height();
        if (w - self.last_w).abs() > 1.0 || (h - self.last_h).abs() > 1.0 {
            self.matrix.resize(w);
            let (rows, cols) = Self::hex_grid_size(w, h);
            self.hex.resize(rows, cols);
            self.last_w = w;
            self.last_h = h;
        }
        self.matrix.update(dt, h);
        self.hex.update(dt);

        self.log_spawn_timer -= dt;
        if self.log_spawn_timer <= 0.0 {
            let (kind, body) = data::random_log_entry();
            let (prefix, color) = match kind {
                LineKind::Command => (
                    format!("root@{}:~# ", self.hostname),
                    Color::new(0.45, 1.0, 0.55, 1.0),
                ),
                LineKind::Info => (String::new(), Color::new(0.55, 0.75, 0.6, 0.85)),
                LineKind::Success => (String::new(), Color::new(0.35, 1.0, 0.65, 1.0)),
                LineKind::Error => (String::new(), Color::new(1.0, 0.4, 0.4, 0.9)),
            };
            self.log_lines.push_back(LogLine {
                prefix,
                body,
                revealed: 0,
                color,
            });
            if self.log_lines.len() > MAX_LOG_LINES {
                self.log_lines.pop_front();
            }
            self.log_spawn_timer = gen_range(0.2, 0.6);
        }

        self.typing_timer += dt;
        const REVEAL_INTERVAL: f32 = 0.012;
        while self.typing_timer > REVEAL_INTERVAL {
            if let Some(last) = self.log_lines.back_mut() {
                if last.revealed < last.body.chars().count() {
                    last.revealed += 1;
                }
            }
            self.typing_timer -= REVEAL_INTERVAL;
        }

        self.progress += dt / self.progress_duration * 100.0;
        if is_key_pressed(KeyCode::Space) {
            self.progress = 100.0;
        }
        if self.progress >= 100.0 {
            self.complete_breach();
        }
    }

    fn draw(&self) {
        let w = screen_width();
        let h = screen_height();
        let elapsed = get_time() - self.start_time;
        let font = self.font.as_ref();

        clear_background(BLACK);

        self.matrix.draw(font, h, 1.0);

        ui::draw_header(font, &self.hostname, elapsed, w);

        let log_w = (w * 0.42).min(520.0);
        let panel_top = 70.0;
        let panel_bottom = h - 90.0;
        ui::draw_log_panel(font, &self.log_lines, 30.0, panel_top, log_w, panel_bottom - panel_top);

        let hex_w = (w * 0.3).min(360.0);
        let hex_x = w - hex_w - 30.0;
        draw_rectangle(hex_x, panel_top, hex_w, panel_bottom - panel_top, Color::new(0.0, 0.02, 0.0, 0.5));
        draw_rectangle_lines(hex_x, panel_top, hex_w, panel_bottom - panel_top, 1.0, Color::new(0.15, 0.4, 0.2, 0.5));
        text::text(font, "MEMORY DUMP", hex_x + 10.0, panel_top + 20.0, 16.0, Color::new(0.5, 0.85, 0.55, 1.0));
        self.hex.draw(font, hex_x + 10.0, panel_top + 44.0);

        ui::draw_progress_bar(font, 30.0, h - 46.0, 40, self.progress, &self.progress_target, &self.hostname);

        text::text(
            font,
            "[ESC] Abort   [SPACE] Force Breach",
            30.0,
            h - 16.0,
            14.0,
            Color::new(0.3, 0.55, 0.3, 0.75),
        );

        ui::draw_scanlines(w, h);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_system_font();
    let mut app = App::new(font);
    loop {
        let dt = get_frame_time();
        app.update(dt);
        app.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
