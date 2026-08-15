//! Drawing helpers for the HUD layered on top of the matrix rain: the status
//! line, the fake exploit log, the ASCII progress bar, and the full-screen
//! "breach" banner.

use std::collections::VecDeque;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::text::{measure, text};

pub struct LogLine {
    pub prefix: String,
    pub body: String,
    pub revealed: usize,
    pub color: Color,
}

pub fn format_timestamp(t: f64) -> String {
    let total = t as u64;
    format!(
        "{:02}:{:02}:{:02}",
        (total / 3600) % 24,
        (total / 60) % 60,
        total % 60
    )
}

pub fn draw_header(font: Option<&Font>, host: &str, elapsed: f64, screen_w: f32) {
    let left = format!("root@{host}:~# tail -f /var/log/breach.log");
    text(font, &left, 20.0, 26.0, 16.0, Color::new(0.3, 0.85, 0.4, 0.9));

    let right = format!("UPTIME {}   STATUS ACTIVE", format_timestamp(elapsed));
    let dims = measure(font, &right, 16.0);
    text(
        font,
        &right,
        screen_w - dims.width - 20.0,
        26.0,
        16.0,
        Color::new(0.3, 0.65, 0.35, 0.8),
    );

    draw_rectangle(0.0, 36.0, screen_w, 1.0, Color::new(0.2, 0.55, 0.28, 0.4));
}

pub fn draw_log_panel(
    font: Option<&Font>,
    lines: &VecDeque<LogLine>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.02, 0.0, 0.5));
    draw_rectangle_lines(x, y, w, h, 1.0, Color::new(0.15, 0.4, 0.2, 0.5));

    let font_size = 15.0;
    let line_h = 19.0;
    let mut cy = y + 20.0;
    let max_lines = ((h - 20.0) / line_h).max(1.0) as usize;
    let start = lines.len().saturating_sub(max_lines);
    let last_idx = lines.len().saturating_sub(1);

    for (i, line) in lines.iter().enumerate().skip(start) {
        let total_chars = line.body.chars().count();
        let visible: String = line.body.chars().take(line.revealed).collect();
        let show_cursor = i == last_idx && line.revealed < total_chars;

        let mut cx = x + 10.0;
        if !line.prefix.is_empty() {
            text(
                font,
                &line.prefix,
                cx,
                cy,
                font_size,
                Color::new(0.3, 0.55, 0.35, 0.85),
            );
            cx += measure(font, &line.prefix, font_size).width;
        }

        let rendered = if show_cursor {
            format!("{visible}\u{2588}")
        } else {
            visible
        };
        text(font, &rendered, cx, cy, font_size, line.color);

        cy += line_h;
        if cy > y + h - 6.0 {
            break;
        }
    }
}

pub fn draw_progress_bar(
    font: Option<&Font>,
    x: f32,
    y: f32,
    bar_len: usize,
    progress: f32,
    target: &str,
    host: &str,
) {
    let filled = (((progress / 100.0) * bar_len as f32) as usize).min(bar_len);
    let bar: String = "#".repeat(filled) + &".".repeat(bar_len - filled);
    let line = format!(
        "root@{host}:~# breaching {} [{bar}] {:>3}%",
        target, progress as u32
    );
    text(font, &line, x, y, 16.0, Color::new(0.4, 1.0, 0.5, 1.0));
}

pub fn draw_scanlines(screen_w: f32, screen_h: f32) {
    let mut y = 0.0;
    while y < screen_h {
        draw_rectangle(0.0, y, screen_w, 1.0, Color::new(0.0, 0.0, 0.0, 0.08));
        y += 3.0;
    }
}

pub fn draw_flash(font: Option<&Font>, screen_w: f32, screen_h: f32, message: &str, life: f32, max_life: f32) {
    let alpha = (life / max_life).clamp(0.0, 1.0);
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.4 * alpha));

    let font_size = 32.0;
    let border = "=".repeat(message.len() + 8);
    let mid = format!("    {message}    ");
    let lines = [border.as_str(), mid.as_str(), border.as_str()];
    let line_h = font_size * 1.3;
    let total_h = line_h * lines.len() as f32;
    let mut cy = screen_h / 2.0 - total_h / 2.0 + font_size;
    for line in lines {
        let dims = measure(font, line, font_size);
        let cx = (screen_w - dims.width) / 2.0;
        text(font, line, cx, cy, font_size, Color::new(0.35, 1.0, 0.5, alpha));
        cy += line_h;
    }

    if gen_range(0, 3) == 0 {
        let gy = gen_range(0.0, screen_h);
        draw_rectangle(
            0.0,
            gy,
            screen_w,
            gen_range(2.0, 5.0),
            Color::new(0.3, 1.0, 0.5, 0.18 * alpha),
        );
    }
}
