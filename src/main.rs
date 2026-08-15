//! hacker_screen — a fake "hacking in progress" screen: matrix rain, a
//! scrolling exploit log, a live hex dump, and a progress bar that
//! periodically flashes ACCESS GRANTED with a glitch burst.
//!
//! Controls: SPACE forces an immediate breach flash, ESC quits.

mod data;
mod matrix;
mod ui;

use std::collections::VecDeque;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use matrix::MatrixRain;
use ui::LogLine;

const MAX_LOG_LINES: usize = 40;
const FLASH_DURATION: f32 = 1.6;

fn window_conf() -> Conf {
    Conf {
        window_title: "SYSTEM BREACH".to_owned(),
        window_width: 1280,
        window_height: 760,
        window_resizable: true,
        ..Default::default()
    }
}

struct App {
    matrix: MatrixRain,
    last_w: f32,
    log_lines: VecDeque<LogLine>,
    log_spawn_timer: f32,
    typing_timer: f32,
    progress: f32,
    progress_target: String,
    progress_duration: f32,
    flash_timer: f32,
    flash_message: String,
    start_time: f64,
}

impl App {
    fn new() -> Self {
        let w = screen_width();
        Self {
            matrix: MatrixRain::new(w, 18.0),
            last_w: w,
            log_lines: VecDeque::new(),
            log_spawn_timer: 0.0,
            typing_timer: 0.0,
            progress: 0.0,
            progress_target: data::random_target().to_string(),
            progress_duration: gen_range(3.0, 7.0),
            flash_timer: 0.0,
            flash_message: String::new(),
            start_time: get_time(),
        }
    }

    fn trigger_flash(&mut self) {
        self.flash_timer = FLASH_DURATION;
        self.flash_message = data::random_success_message().to_string();
    }

    fn update(&mut self, dt: f32) {
        let w = screen_width();
        if (w - self.last_w).abs() > 1.0 {
            self.matrix.resize(w);
            self.last_w = w;
        }
        self.matrix.update(dt, screen_height());

        self.log_spawn_timer -= dt;
        if self.log_spawn_timer <= 0.0 {
            self.log_lines.push_back(LogLine {
                text: data::random_log_line(),
                revealed: 0,
                spawned_at: get_time() - self.start_time,
            });
            if self.log_lines.len() > MAX_LOG_LINES {
                self.log_lines.pop_front();
            }
            self.log_spawn_timer = gen_range(0.15, 0.45);
        }

        self.typing_timer += dt;
        const REVEAL_INTERVAL: f32 = 0.012;
        while self.typing_timer > REVEAL_INTERVAL {
            if let Some(last) = self.log_lines.back_mut() {
                if last.revealed < last.text.chars().count() {
                    last.revealed += 1;
                }
            }
            self.typing_timer -= REVEAL_INTERVAL;
        }

        if self.flash_timer <= 0.0 {
            self.progress += dt / self.progress_duration * 100.0;
            if self.progress >= 100.0 {
                self.progress = 100.0;
                self.trigger_flash();
            }
        } else {
            self.flash_timer -= dt;
            if self.flash_timer <= 0.0 {
                self.progress = 0.0;
                self.progress_target = data::random_target().to_string();
                self.progress_duration = gen_range(3.0, 7.0);
            }
        }

        if is_key_pressed(KeyCode::Space) {
            self.trigger_flash();
        }
    }

    fn draw(&self) {
        let w = screen_width();
        let h = screen_height();
        let elapsed = get_time() - self.start_time;

        clear_background(BLACK);

        let matrix_dim = if self.flash_timer > 0.0 { 0.35 } else { 1.0 };
        self.matrix.draw(h, matrix_dim);

        ui::draw_header(elapsed, w);

        let log_w = (w * 0.42).min(520.0);
        let panel_top = 70.0;
        let panel_bottom = h - 90.0;
        ui::draw_log_panel(&self.log_lines, 30.0, panel_top, log_w, panel_bottom - panel_top);

        let hex_w = (w * 0.3).min(360.0);
        let hex_x = w - hex_w - 30.0;
        ui::draw_hex_panel(hex_x, panel_top, hex_w, panel_bottom - panel_top);

        ui::draw_progress_bar(30.0, h - 46.0, w - 60.0, 22.0, self.progress, &self.progress_target);

        draw_text(
            "[ESC] Abort     [SPACE] Force Breach",
            30.0,
            h - 8.0,
            14.0,
            Color::new(0.3, 0.6, 0.3, 0.8),
        );

        ui::draw_scanlines(w, h);

        if self.flash_timer > 0.0 {
            ui::draw_flash(w, h, &self.flash_message, self.flash_timer);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();
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
