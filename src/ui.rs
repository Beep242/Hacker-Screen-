//! Drawing helpers for the HUD layered on top of the matrix rain: the fake
//! exploit log, hex dump, progress bar, and the full-screen "breach" flash.

use std::collections::VecDeque;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::data::random_hex_byte;

pub struct LogLine {
    pub text: String,
    pub revealed: usize,
    pub spawned_at: f64,
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

pub fn draw_header(elapsed: f64, screen_w: f32) {
    let flicker = if gen_range(0, 40) == 0 { 0.55 } else { 1.0 };
    let title = "SYSTEM INFILTRATION IN PROGRESS";
    let font_size = 28.0;
    let dims = measure_text(title, None, font_size as u16, 1.0);
    let x = (screen_w - dims.width) / 2.0;
    draw_text(
        title,
        x,
        44.0,
        font_size,
        Color::new(0.2, 1.0, 0.4, flicker),
    );
    let cursor = if (elapsed * 2.0) as u64 % 2 == 0 { "_" } else { " " };
    draw_text(cursor, x + dims.width + 4.0, 44.0, font_size, GREEN);
}

pub fn draw_log_panel(lines: &VecDeque<LogLine>, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.05, 0.0, 0.55));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.0, 0.8, 0.3, 0.6));
    draw_text(
        "EXPLOIT LOG",
        x + 10.0,
        y + 20.0,
        18.0,
        Color::new(0.6, 1.0, 0.6, 1.0),
    );

    let font_size = 15.0;
    let line_h = 20.0;
    let mut cy = y + 46.0;
    let start = lines.len().saturating_sub(((h - 50.0) / line_h) as usize);
    for line in lines.iter().skip(start) {
        let visible: String = line.text.chars().take(line.revealed).collect();
        let stamp = format!("[{}]", format_timestamp(line.spawned_at));
        draw_text(
            &stamp,
            x + 10.0,
            cy,
            font_size,
            Color::new(0.3, 0.6, 0.3, 0.8),
        );
        draw_text(
            &visible,
            x + 92.0,
            cy,
            font_size,
            Color::new(0.4, 1.0, 0.5, 1.0),
        );
        cy += line_h;
        if cy > y + h - 8.0 {
            break;
        }
    }
}

pub fn draw_hex_panel(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.05, 0.0, 0.55));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.0, 0.8, 0.3, 0.6));
    draw_text(
        "MEMORY DUMP",
        x + 10.0,
        y + 20.0,
        18.0,
        Color::new(0.6, 1.0, 0.6, 1.0),
    );

    let font_size = 14.0;
    let line_h = 18.0;
    let bytes_per_row = 8;
    let mut cy = y + 44.0;
    let mut addr: u32 = gen_range(0, 0xFFFF);
    while cy < y + h - 6.0 {
        draw_text(
            &format!("0x{addr:04X}"),
            x + 10.0,
            cy,
            font_size,
            Color::new(0.3, 0.6, 0.3, 0.8),
        );
        let mut cx = x + 78.0;
        for _ in 0..bytes_per_row {
            let color = if gen_range(0, 12) == 0 {
                Color::new(1.0, 0.3, 0.3, 0.9)
            } else {
                Color::new(0.3, 1.0, 0.4, 0.9)
            };
            draw_text(&random_hex_byte(), cx, cy, font_size, color);
            cx += 24.0;
        }
        addr = addr.wrapping_add(bytes_per_row as u32);
        cy += line_h;
    }
}

pub fn draw_progress_bar(x: f32, y: f32, w: f32, h: f32, progress: f32, target: &str) {
    let label = format!("BREACHING: {} - {:>3}%", target.to_uppercase(), progress as u32);
    draw_text(&label, x, y - 10.0, 16.0, Color::new(0.6, 1.0, 0.6, 1.0));
    draw_rectangle_lines(x, y, w, h, 2.0, Color::new(0.0, 0.9, 0.3, 0.9));
    let fill_w = w * (progress / 100.0).clamp(0.0, 1.0);
    draw_rectangle(x, y, fill_w, h, Color::new(0.0, 0.9, 0.3, 0.35));
    for i in 0..(fill_w as i32 / 6) {
        let bx = x + i as f32 * 6.0;
        draw_rectangle(bx, y, 3.0, h, Color::new(0.2, 1.0, 0.4, 0.7));
    }
}

pub fn draw_scanlines(screen_w: f32, screen_h: f32) {
    let mut y = 0.0;
    while y < screen_h {
        draw_rectangle(0.0, y, screen_w, 1.0, Color::new(0.0, 0.0, 0.0, 0.12));
        y += 3.0;
    }
}

pub fn draw_flash(screen_w: f32, screen_h: f32, message: &str, life: f32) {
    let alpha = (life / 1.6).clamp(0.0, 1.0);
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.55 * alpha));

    for _ in 0..14 {
        let gy = gen_range(0.0, screen_h);
        let gh = gen_range(2.0, 18.0);
        let gx = gen_range(-30.0, 30.0);
        let color = if gen_range(0, 2) == 0 {
            Color::new(1.0, 0.1, 0.1, 0.35 * alpha)
        } else {
            Color::new(0.1, 1.0, 0.3, 0.35 * alpha)
        };
        draw_rectangle(gx, gy, screen_w, gh, color);
    }

    let pulse = 0.6 + 0.4 * ((life * 14.0).sin());
    let font_size = 64.0;
    let dims = measure_text(message, None, font_size as u16, 1.0);
    let mx = (screen_w - dims.width) / 2.0 + gen_range(-3.0, 3.0);
    let my = screen_h / 2.0 + gen_range(-3.0, 3.0);
    draw_text(
        message,
        mx,
        my,
        font_size,
        Color::new(1.0, 0.15 * pulse, 0.15 * pulse, alpha),
    );
}
