//! Drawing primitives for the circular HUD: background grid, corner
//! brackets, rotating instrument rings, the radar sweep, the pulsing core,
//! gauges, the audio waveform, and the satellite-widget connector spokes.
//! Built entirely from macroquad's `draw_arc`/`draw_poly` family so nothing
//! here needs external assets.

use std::collections::VecDeque;

use macroquad::prelude::*;

use crate::text::{measure, text};

pub fn draw_grid(screen_w: f32, screen_h: f32) {
    let step = 56.0;
    let color = Color::new(0.15, 0.4, 0.5, 0.06);
    let mut x = (screen_w / 2.0) % step;
    while x < screen_w {
        draw_line(x, 0.0, x, screen_h, 1.0, color);
        x += step;
    }
    let mut y = (screen_h / 2.0) % step;
    while y < screen_h {
        draw_line(0.0, y, screen_w, y, 1.0, color);
        y += step;
    }
}

pub fn draw_corner_brackets(screen_w: f32, screen_h: f32) {
    let len = 34.0;
    let pad = 26.0;
    let thickness = 2.0;
    let color = Color::new(0.35, 0.85, 1.0, 0.5);
    let corners = [
        (pad, pad, 1.0, 1.0),
        (screen_w - pad, pad, -1.0, 1.0),
        (pad, screen_h - pad, 1.0, -1.0),
        (screen_w - pad, screen_h - pad, -1.0, -1.0),
    ];
    for (x, y, dx, dy) in corners {
        draw_line(x, y, x + len * dx, y, thickness, color);
        draw_line(x, y, x, y + len * dy, thickness, color);
    }
}

fn draw_ticks(cx: f32, cy: f32, r: f32, len: f32, count: usize, rotation_deg: f32, color: Color) {
    let step = 360.0 / count as f32;
    for i in 0..count {
        let a = i as f32 * step + rotation_deg;
        draw_arc(cx, cy, 2, r, a, len, 1.4, color);
    }
}

fn draw_dashed_ring(cx: f32, cy: f32, r: f32, count: usize, rotation_deg: f32, color: Color) {
    let step = 360.0 / count as f32;
    let dash = step * 0.55;
    for i in 0..count {
        let a = i as f32 * step + rotation_deg;
        draw_arc(cx, cy, 2, r, a, 2.2, dash, color);
    }
}

pub fn draw_rings(cx: f32, cy: f32, t: f32) {
    let r1 = 230.0;
    draw_circle_lines(cx, cy, r1, 1.2, Color::new(0.3, 0.75, 1.0, 0.3));
    draw_ticks(cx, cy, r1, 10.0, 72, t * 6.0, Color::new(0.35, 0.9, 1.0, 0.5));

    // Dense fine-toothed comb ring for the busier instrument-cluster look.
    let r_comb = 204.0;
    draw_circle_lines(cx, cy, r_comb, 0.8, Color::new(0.3, 0.8, 1.0, 0.2));
    draw_ticks(cx, cy, r_comb, 5.0, 140, t * -9.0, Color::new(0.4, 0.9, 1.0, 0.3));

    let r2 = 178.0;
    draw_dashed_ring(cx, cy, r2, 40, -(t * 16.0), Color::new(0.3, 0.85, 1.0, 0.55));

    let r3 = 128.0;
    draw_circle_lines(cx, cy, r3, 1.0, Color::new(0.3, 0.8, 1.0, 0.35));
    draw_ticks(cx, cy, r3, 7.0, 36, -(t * 24.0), Color::new(0.4, 0.95, 1.0, 0.55));
}

/// Draws a connector "trace" between the central hub's outer ring and a
/// satellite widget's edge, with a small solder-pad dot at each end —
/// the circuit-board look from Rainmeter-style HUD skins.
pub fn draw_spoke(cx: f32, cy: f32, hub_r: f32, sat_x: f32, sat_y: f32, sat_r: f32, color: Color) {
    let dx = sat_x - cx;
    let dy = sat_y - cy;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist < hub_r + sat_r {
        return;
    }
    let ux = dx / dist;
    let uy = dy / dist;
    let x0 = cx + ux * hub_r;
    let y0 = cy + uy * hub_r;
    let x1 = sat_x - ux * sat_r;
    let y1 = sat_y - uy * sat_r;
    draw_line(x0, y0, x1, y1, 1.2, color);
    draw_circle(x0, y0, 2.5, color);
    draw_circle(x1, y1, 2.5, color);
}

pub fn draw_sweep(cx: f32, cy: f32, radius: f32, t: f32) {
    let angle = (t * 70.0) % 360.0;
    draw_arc(
        cx,
        cy,
        40,
        0.0,
        angle - 46.0,
        radius,
        46.0,
        Color::new(0.25, 0.85, 1.0, 0.05),
    );
    let rad = angle.to_radians();
    draw_line(
        cx,
        cy,
        cx + rad.cos() * radius,
        cy + rad.sin() * radius,
        1.6,
        Color::new(0.55, 1.0, 1.0, 0.7),
    );
}

pub fn draw_ripple(cx: f32, cy: f32, life: f32, max_life: f32) {
    if life <= 0.0 {
        return;
    }
    let progress = 1.0 - (life / max_life);
    let radius = 32.0 + progress * 160.0;
    let alpha = (life / max_life) * 0.6;
    draw_circle_lines(cx, cy, radius, 2.0, Color::new(0.5, 1.0, 1.0, alpha));
}

pub fn draw_bar_gauge(font: Option<&Font>, x: f32, y: f32, w: f32, label: &str, value: f32) {
    text(font, label, x, y, 14.0, Color::new(0.5, 0.85, 0.95, 0.85));
    let bar_y = y + 6.0;
    let h = 6.0;
    draw_rectangle_lines(x, bar_y, w, h, 1.0, Color::new(0.3, 0.7, 0.9, 0.5));
    draw_rectangle(
        x,
        bar_y,
        w * (value / 100.0).clamp(0.0, 1.0),
        h,
        Color::new(0.35, 0.9, 1.0, 0.55),
    );
    let pct = format!("{:>3.0}%", value);
    text(font, &pct, x + w + 8.0, y, 14.0, Color::new(0.6, 0.95, 1.0, 0.9));
}

pub fn draw_ring_gauge(font: Option<&Font>, cx: f32, cy: f32, r: f32, label: &str, value: f32) {
    draw_circle_lines(cx, cy, r, 3.0, Color::new(0.25, 0.55, 0.7, 0.35));
    draw_arc(
        cx,
        cy,
        60,
        r - 1.5,
        -90.0,
        3.0,
        360.0 * (value / 100.0).clamp(0.0, 1.0),
        Color::new(0.4, 0.95, 1.0, 0.85),
    );
    let pct = format!("{value:.0}%");
    let dims = measure(font, &pct, 20.0);
    text(font, &pct, cx - dims.width / 2.0, cy + 7.0, 20.0, Color::new(0.6, 1.0, 1.0, 0.95));
    let dims2 = measure(font, label, 12.0);
    text(
        font,
        label,
        cx - dims2.width / 2.0,
        cy + r + 18.0,
        12.0,
        Color::new(0.5, 0.85, 0.95, 0.75),
    );
}

pub fn draw_sparkline(font: Option<&Font>, x: f32, y: f32, w: f32, h: f32, label: &str, values: &VecDeque<f32>) {
    text(font, label, x, y - 8.0, 13.0, Color::new(0.5, 0.85, 0.95, 0.8));
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.05, 0.07, 0.4));
    draw_rectangle_lines(x, y, w, h, 1.0, Color::new(0.25, 0.6, 0.75, 0.4));

    if values.len() < 2 {
        return;
    }
    let max = 100.0;
    let step = w / (values.len() as f32 - 1.0);
    let mut prev: Option<(f32, f32)> = None;
    for (i, v) in values.iter().enumerate() {
        let px = x + i as f32 * step;
        let py = y + h - (v / max).clamp(0.0, 1.0) * h;
        if let Some((lx, ly)) = prev {
            draw_line(lx, ly, px, py, 1.4, Color::new(0.35, 0.9, 1.0, 0.8));
        }
        prev = Some((px, py));
    }
}

pub fn draw_waveform(x: f32, y: f32, w: f32, t: f32, bars: usize) {
    let spacing = w / bars as f32;
    for i in 0..bars {
        let phase = i as f32 * 0.7;
        let h = (12.0
            + (t * 3.0 + phase).sin().abs() * 26.0
            + (t * 5.3 + phase * 1.7).sin().abs() * 14.0)
            .min(46.0);
        let bx = x + i as f32 * spacing;
        let alpha = 0.35 + (h / 46.0) * 0.5;
        draw_rectangle(bx, y - h, spacing * 0.6, h, Color::new(0.35, 0.9, 1.0, alpha));
    }
}
