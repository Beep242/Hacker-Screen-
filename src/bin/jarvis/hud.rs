//! Drawing primitives for the circular HUD: background grid, corner
//! brackets, rotating instrument rings, the radar sweep, the pulsing core,
//! gauges, the audio waveform, and the satellite-widget connector spokes.
//! Built entirely from macroquad's `draw_arc`/`draw_poly` family so nothing
//! here needs external assets. Every stroke goes through the `glow_*`
//! helpers below, which fake bloom by stacking wider/fainter copies behind
//! the crisp line — plain single-alpha outlines read as flat and thin.

use std::collections::VecDeque;

use macroquad::prelude::*;

use crate::text::{measure, text};

fn glow_line(x0: f32, y0: f32, x1: f32, y1: f32, thickness: f32, color: Color) {
    draw_line(x0, y0, x1, y1, thickness * 4.0, Color::new(color.r, color.g, color.b, color.a * 0.08));
    draw_line(x0, y0, x1, y1, thickness * 2.0, Color::new(color.r, color.g, color.b, color.a * 0.25));
    draw_line(x0, y0, x1, y1, thickness, color);
}

fn glow_circle_lines(cx: f32, cy: f32, r: f32, thickness: f32, color: Color) {
    draw_circle_lines(cx, cy, r, thickness * 4.0, Color::new(color.r, color.g, color.b, color.a * 0.07));
    draw_circle_lines(cx, cy, r, thickness * 2.0, Color::new(color.r, color.g, color.b, color.a * 0.22));
    draw_circle_lines(cx, cy, r, thickness, color);
}

fn glow_arc(cx: f32, cy: f32, sides: u8, r: f32, rotation: f32, thickness: f32, arc: f32, color: Color) {
    draw_arc(cx, cy, sides, r - thickness, rotation, thickness * 3.5, arc, Color::new(color.r, color.g, color.b, color.a * 0.12));
    draw_arc(cx, cy, sides, r, rotation, thickness, arc, color);
}

pub fn draw_grid(screen_w: f32, screen_h: f32) {
    let step = 56.0;
    let color = Color::new(0.15, 0.4, 0.5, 0.08);
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

/// Fixed-seed decorative starfield: positions are derived from the index so
/// they stay put across frames without needing any stored state.
pub fn draw_starfield(screen_w: f32, screen_h: f32, t: f32, count: usize) {
    for i in 0..count {
        let fx = ((i as f32 * 12.9898).sin() * 43758.5453) % 1.0;
        let fy = ((i as f32 * 78.233).sin() * 12543.789) % 1.0;
        let x = fx.abs() * screen_w;
        let y = fy.abs() * screen_h;
        let twinkle = 0.3 + 0.3 * (t * (0.6 + fx.abs()) + i as f32).sin().abs();
        let size = 1.0 + (fy.abs() * 1.6);
        draw_circle(x, y, size * 0.5, Color::new(0.6, 0.9, 1.0, twinkle * 0.5));
    }
}

pub fn draw_corner_brackets(screen_w: f32, screen_h: f32) {
    let len = 38.0;
    let pad = 24.0;
    let thickness = 2.2;
    let color = Color::new(0.4, 0.9, 1.0, 0.75);
    let corners = [
        (pad, pad, 1.0, 1.0),
        (screen_w - pad, pad, -1.0, 1.0),
        (pad, screen_h - pad, 1.0, -1.0),
        (screen_w - pad, screen_h - pad, -1.0, -1.0),
    ];
    for (x, y, dx, dy) in corners {
        glow_line(x, y, x + len * dx, y, thickness, color);
        glow_line(x, y, x, y + len * dy, thickness, color);
    }
}

fn draw_ticks(cx: f32, cy: f32, r: f32, len: f32, count: usize, rotation_deg: f32, color: Color) {
    let step = 360.0 / count as f32;
    for i in 0..count {
        let a = i as f32 * step + rotation_deg;
        draw_arc(cx, cy, 2, r, a, len, 1.6, color);
    }
}

fn draw_dashed_ring(cx: f32, cy: f32, r: f32, count: usize, rotation_deg: f32, color: Color) {
    let step = 360.0 / count as f32;
    let dash = step * 0.55;
    for i in 0..count {
        let a = i as f32 * step + rotation_deg;
        draw_arc(cx, cy, 2, r, a, 2.6, dash, color);
    }
}

pub fn draw_rings(cx: f32, cy: f32, t: f32) {
    let r1 = 230.0;
    glow_circle_lines(cx, cy, r1, 1.6, Color::new(0.35, 0.8, 1.0, 0.55));
    draw_ticks(cx, cy, r1, 12.0, 72, t * 6.0, Color::new(0.45, 0.95, 1.0, 0.8));

    // Dense fine-toothed comb ring for the busier instrument-cluster look.
    let r_comb = 204.0;
    draw_circle_lines(cx, cy, r_comb, 1.0, Color::new(0.35, 0.85, 1.0, 0.35));
    draw_ticks(cx, cy, r_comb, 6.0, 140, t * -9.0, Color::new(0.45, 0.9, 1.0, 0.5));

    let r2 = 178.0;
    draw_dashed_ring(cx, cy, r2, 40, -(t * 16.0), Color::new(0.35, 0.9, 1.0, 0.75));

    let r3 = 128.0;
    glow_circle_lines(cx, cy, r3, 1.4, Color::new(0.35, 0.85, 1.0, 0.55));
    draw_ticks(cx, cy, r3, 8.0, 36, -(t * 24.0), Color::new(0.5, 1.0, 1.0, 0.8));
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
    glow_line(x0, y0, x1, y1, 1.4, color);
    draw_circle(x0, y0, 3.0, color);
    draw_circle(x1, y1, 3.0, color);
}

pub fn draw_sweep(cx: f32, cy: f32, radius: f32, t: f32) {
    let angle = (t * 70.0) % 360.0;
    draw_arc(cx, cy, 40, 0.0, angle - 50.0, radius, 50.0, Color::new(0.3, 0.9, 1.0, 0.09));
    let rad = angle.to_radians();
    glow_line(
        cx,
        cy,
        cx + rad.cos() * radius,
        cy + rad.sin() * radius,
        2.0,
        Color::new(0.6, 1.0, 1.0, 0.9),
    );
}

pub fn draw_ripple(cx: f32, cy: f32, life: f32, max_life: f32) {
    if life <= 0.0 {
        return;
    }
    let progress = 1.0 - (life / max_life);
    let radius = 32.0 + progress * 180.0;
    let alpha = (life / max_life) * 0.7;
    glow_circle_lines(cx, cy, radius, 2.2, Color::new(0.55, 1.0, 1.0, alpha));
}

pub fn draw_bar_gauge(font: Option<&Font>, x: f32, y: f32, w: f32, label: &str, value: f32) {
    text(font, label, x, y, 14.0, Color::new(0.55, 0.9, 1.0, 0.9));
    let bar_y = y + 6.0;
    let h = 7.0;
    draw_rectangle_lines(x, bar_y, w, h, 1.2, Color::new(0.35, 0.75, 0.95, 0.6));
    let fill_w = w * (value / 100.0).clamp(0.0, 1.0);
    draw_rectangle(x, bar_y, fill_w, h, Color::new(0.15, 0.5, 0.65, 0.3));
    glow_line(x, bar_y + h / 2.0, x + fill_w, bar_y + h / 2.0, h * 0.7, Color::new(0.4, 0.95, 1.0, 0.9));
    let pct = format!("{:>3.0}%", value);
    text(font, &pct, x + w + 8.0, y, 14.0, Color::new(0.65, 1.0, 1.0, 0.95));
}

pub fn draw_ring_gauge(font: Option<&Font>, cx: f32, cy: f32, r: f32, label: &str, value: f32) {
    glow_circle_lines(cx, cy, r, 3.0, Color::new(0.3, 0.6, 0.75, 0.5));
    glow_arc(
        cx,
        cy,
        60,
        r,
        -90.0,
        3.2,
        360.0 * (value / 100.0).clamp(0.0, 1.0),
        Color::new(0.45, 1.0, 1.0, 0.95),
    );
    let pct = format!("{value:.0}%");
    let dims = measure(font, &pct, 20.0);
    text(font, &pct, cx - dims.width / 2.0, cy + 7.0, 20.0, Color::new(0.65, 1.0, 1.0, 1.0));
    let dims2 = measure(font, label, 12.0);
    text(
        font,
        label,
        cx - dims2.width / 2.0,
        cy + r + 18.0,
        12.0,
        Color::new(0.55, 0.9, 1.0, 0.85),
    );
}

pub fn draw_sparkline(font: Option<&Font>, x: f32, y: f32, w: f32, h: f32, label: &str, values: &VecDeque<f32>) {
    text(font, label, x, y - 8.0, 13.0, Color::new(0.55, 0.9, 1.0, 0.85));
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.08, 0.11, 0.55));
    draw_rectangle_lines(x, y, w, h, 1.2, Color::new(0.3, 0.7, 0.9, 0.55));

    if values.len() < 2 {
        return;
    }
    let max = 100.0;
    let step = w / (values.len() as f32 - 1.0);
    let points: Vec<(f32, f32)> = values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let px = x + i as f32 * step;
            let py = y + h - (v / max).clamp(0.0, 1.0) * h;
            (px, py)
        })
        .collect();

    // Fill the area under the curve with a proper quad per segment (two
    // triangles) instead of thick vertical strokes, which balloon past the
    // box edges when there are only a few samples and `step` is large.
    let fill = Color::new(0.25, 0.75, 0.95, 0.16);
    for pair in points.windows(2) {
        let (x0, y0) = pair[0];
        let (x1, y1) = pair[1];
        let base = y + h;
        draw_triangle(
            Vec2::new(x0, y0),
            Vec2::new(x1, y1),
            Vec2::new(x1, base),
            fill,
        );
        draw_triangle(
            Vec2::new(x0, y0),
            Vec2::new(x1, base),
            Vec2::new(x0, base),
            fill,
        );
    }

    for pair in points.windows(2) {
        glow_line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, 1.6, Color::new(0.4, 0.95, 1.0, 0.95));
    }
}

pub fn draw_waveform(x: f32, y: f32, w: f32, t: f32, bars: usize) {
    let spacing = w / bars as f32;
    for i in 0..bars {
        let phase = i as f32 * 0.7;
        let h = (14.0
            + (t * 3.0 + phase).sin().abs() * 30.0
            + (t * 5.3 + phase * 1.7).sin().abs() * 16.0)
            .min(52.0);
        let bx = x + i as f32 * spacing;
        let bw = spacing * 0.6;
        let alpha = 0.5 + (h / 52.0) * 0.5;
        draw_rectangle(bx, y - h, bw, h, Color::new(0.4, 0.95, 1.0, alpha));
        draw_rectangle(bx, y - h, bw, 3.0, Color::new(0.75, 1.0, 1.0, alpha));
        // Faint mirrored reflection below the baseline for visual weight.
        draw_rectangle(bx, y + 3.0, bw, h * 0.35, Color::new(0.3, 0.85, 1.0, alpha * 0.2));
    }
}

/// A few small satellites drifting around the instrument rings at
/// different radii/speeds — cheap motion that fills the gap between the
/// hub and the corner widgets.
pub fn draw_orbiters(cx: f32, cy: f32, t: f32) {
    let orbits = [
        (255.0, 0.35, 0.0, 3.5),
        (255.0, -0.22, 2.1, 2.8),
        (280.0, 0.15, 4.4, 3.0),
    ];
    for (r, speed, phase, size) in orbits {
        let a = t * speed + phase;
        let x = cx + a.cos() * r;
        let y = cy + a.sin() * r;
        draw_circle(x, y, size * 2.2, Color::new(0.3, 0.75, 0.9, 0.12));
        draw_circle(x, y, size, Color::new(0.6, 1.0, 1.0, 0.85));
    }
}

/// A vertical strip of tick segments near a screen edge with a slow chase
/// light running through it — decorative instrument filler, matching the
/// thin meter bars common on Rainmeter-style HUD skins.
pub fn draw_edge_meter(x: f32, top: f32, height: f32, segments: usize, t: f32, speed: f32) {
    let seg_h = height / segments as f32;
    let chase = ((t * speed) as usize) % segments;
    for i in 0..segments {
        let y = top + i as f32 * seg_h;
        let dist = (i as i32 - chase as i32).unsigned_abs() as f32;
        let lit = (1.0 - dist / 4.0).max(0.0);
        let color = Color::new(0.35 + 0.4 * lit, 0.8 + 0.2 * lit, 1.0, 0.15 + 0.65 * lit);
        draw_line(x, y, x, y + seg_h * 0.6, 2.0, color);
    }
}

/// A thin strip of day cells across the top of the screen with one
/// highlighted — purely decorative dashboard chrome, not tied to the real
/// calendar (this app never reads system time beyond its own uptime).
pub fn draw_calendar_ribbon(font: Option<&Font>, screen_w: f32, highlight_day: usize) {
    let x0 = 30.0;
    let x1 = screen_w - 30.0;
    let cells = 30;
    let cell_w = (x1 - x0) / cells as f32;
    let y = 10.0;
    for i in 0..cells {
        let day = i + 1;
        let cx = x0 + i as f32 * cell_w;
        let is_hl = day == highlight_day;
        if is_hl {
            draw_rectangle(cx, y - 2.0, cell_w - 2.0, 15.0, Color::new(0.3, 0.85, 1.0, 0.55));
        }
        let label = format!("{day:02}");
        let color = if is_hl {
            Color::new(0.03, 0.08, 0.1, 1.0)
        } else {
            Color::new(0.4, 0.7, 0.85, 0.5)
        };
        text(font, &label, cx + 1.0, y + 10.0, 11.0, color);
    }
}

/// A small labeled node hanging off the instrument ring by a short spoke —
/// the "linked systems" list look from Rainmeter HUD skins, minus any
/// pretense that these are real running processes.
pub fn draw_node_label(
    font: Option<&Font>,
    cx: f32,
    cy: f32,
    angle_deg: f32,
    inner_r: f32,
    outer_r: f32,
    label: &str,
    color: Color,
) {
    let rad = angle_deg.to_radians();
    let (s, c) = rad.sin_cos();
    let x0 = cx + c * inner_r;
    let y0 = cy + s * inner_r;
    let x1 = cx + c * outer_r;
    let y1 = cy + s * outer_r;
    glow_line(x0, y0, x1, y1, 1.0, color);
    draw_circle(x1, y1, 2.5, color);
    let font_size = 13.0;
    let dims = measure(font, label, font_size);
    let tx = if c < -0.15 {
        x1 - dims.width - 8.0
    } else if c > 0.15 {
        x1 + 8.0
    } else {
        x1 - dims.width / 2.0
    };
    text(font, label, tx, y1 + 4.0, font_size, color);
}

/// A decorative playback-style control bar (no real media behind it) with
/// a status label — the "console completeness" touch from the reference
/// skins' bottom bar.
pub fn draw_control_bar(font: Option<&Font>, x: f32, y: f32, w: f32, h: f32, label: &str) {
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.07, 0.1, 0.55));
    draw_rectangle_lines(x, y, w, h, 1.2, Color::new(0.3, 0.7, 0.9, 0.55));

    let cy = y + h / 2.0;
    let color = Color::new(0.55, 0.95, 1.0, 0.9);
    let mut cx = x + 18.0;
    draw_triangle(Vec2::new(cx + 8.0, cy - 7.0), Vec2::new(cx + 8.0, cy + 7.0), Vec2::new(cx, cy), color);
    draw_triangle(Vec2::new(cx, cy - 7.0), Vec2::new(cx, cy + 7.0), Vec2::new(cx - 8.0, cy), color);
    cx += 28.0;
    draw_triangle(Vec2::new(cx, cy - 8.0), Vec2::new(cx, cy + 8.0), Vec2::new(cx + 11.0, cy), color);
    cx += 24.0;
    draw_triangle(Vec2::new(cx, cy - 7.0), Vec2::new(cx, cy + 7.0), Vec2::new(cx + 8.0, cy), color);
    draw_triangle(Vec2::new(cx + 8.0, cy - 7.0), Vec2::new(cx + 8.0, cy + 7.0), Vec2::new(cx + 16.0, cy), color);

    text(font, label, x + 110.0, cy + 5.0, 14.0, Color::new(0.6, 0.95, 1.0, 0.9));
}
