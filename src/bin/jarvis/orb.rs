//! The centerpiece: a rotating wireframe sphere built from points scattered
//! evenly across its surface (a Fibonacci sphere), each carrying a "code"
//! glyph that occasionally mutates. Painter's-algorithm depth sorting plus
//! depth-scaled size/alpha gives it a real sense of volume without any
//! actual 3D pipeline — just rotation matrices and an orthographic
//! projection. A slow multi-frequency breathing pulse on the radius is what
//! sells "alive" rather than "spinning logo".

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::text::text;

const GLYPHS: &str = "01{}<>/=+-#$%^&*";

fn random_glyph() -> char {
    let idx = gen_range(0, GLYPHS.len());
    GLYPHS.chars().nth(idx).unwrap()
}

/// Evenly distributes `n` points across a unit sphere.
fn fibonacci_sphere(n: usize) -> Vec<Vec3> {
    let golden_angle = std::f32::consts::PI * (3.0 - (5.0f32).sqrt());
    let denom = (n.max(2) - 1) as f32;
    (0..n)
        .map(|i| {
            let y = 1.0 - (i as f32 / denom) * 2.0;
            let r = (1.0 - y * y).max(0.0).sqrt();
            let theta = golden_angle * i as f32;
            Vec3::new(theta.cos() * r, y, theta.sin() * r)
        })
        .collect()
}

fn rotate(p: Vec3, pitch: f32, yaw: f32) -> Vec3 {
    let (sp, cp) = pitch.sin_cos();
    let y1 = p.y * cp - p.z * sp;
    let z1 = p.y * sp + p.z * cp;

    let (sy, cy) = yaw.sin_cos();
    let x2 = p.x * cy + z1 * sy;
    let z2 = -p.x * sy + z1 * cy;

    Vec3::new(x2, y1, z2)
}

struct OrbPoint {
    pos: Vec3,
    ch: char,
    mutate_timer: f32,
}

pub struct CodeOrb {
    points: Vec<OrbPoint>,
}

impl CodeOrb {
    pub fn new(count: usize) -> Self {
        let points = fibonacci_sphere(count)
            .into_iter()
            .map(|pos| OrbPoint {
                pos,
                ch: random_glyph(),
                mutate_timer: gen_range(0.0, 2.5),
            })
            .collect();
        Self { points }
    }

    pub fn update(&mut self, dt: f32) {
        for p in &mut self.points {
            p.mutate_timer -= dt;
            if p.mutate_timer <= 0.0 {
                p.ch = random_glyph();
                p.mutate_timer = gen_range(1.2, 3.5);
            }
        }
    }

    pub fn draw(&self, font: Option<&Font>, cx: f32, cy: f32, t: f32, energy: f32) {
        let breathe = 1.0 + 0.06 * (t * 1.3).sin() + 0.02 * (t * 4.7).sin();
        let radius = (108.0 + energy * 30.0) * breathe;

        let yaw = t * 0.4;
        let pitch = (t * 0.27).sin() * 0.25;

        for (r, a) in [
            (radius * 1.4, 0.02),
            (radius * 1.15, 0.04),
            (radius * 0.9, 0.06),
        ] {
            draw_circle(cx, cy, r, Color::new(0.15, 0.55, 0.7, a + energy * 0.03));
        }

        let mut projected: Vec<(f32, f32, f32, char)> = self
            .points
            .iter()
            .map(|p| {
                let r = rotate(p.pos, pitch, yaw);
                let depth = (r.z + 1.0) / 2.0;
                (cx + r.x * radius, cy - r.y * radius, depth, p.ch)
            })
            .collect();
        projected.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        for (sx, sy, depth, ch) in projected {
            let size = 7.0 + depth * 9.0;
            let alpha = (0.12 + depth * 0.8).min(1.0);
            let glow = 0.55 + depth * 0.35;
            let color = Color::new(0.35 + 0.25 * depth, 0.85 + glow * 0.1, 1.0, alpha);
            text(font, &ch.to_string(), sx - size * 0.3, sy + size * 0.35, size, color);
        }

        draw_circle(cx, cy, 5.0, Color::new(0.85, 1.0, 1.0, 0.95));
        draw_circle_lines(cx, cy, 8.0, 1.5, Color::new(0.6, 1.0, 1.0, 0.6 + energy * 0.3));
    }
}
