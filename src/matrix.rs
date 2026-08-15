//! Matrix-style falling character rain, drawn as the background layer.

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::data::random_matrix_char;

struct Column {
    x: f32,
    y: f32,
    speed: f32,
    length: usize,
}

impl Column {
    fn respawn(x: f32) -> Self {
        Self {
            x,
            y: gen_range(-400.0, 0.0),
            speed: gen_range(80.0, 260.0),
            length: gen_range(8, 24),
        }
    }
}

pub struct MatrixRain {
    columns: Vec<Column>,
    font_size: f32,
    col_width: f32,
}

impl MatrixRain {
    pub fn new(screen_w: f32, font_size: f32) -> Self {
        let col_width = font_size * 0.65;
        let n = (screen_w / col_width).ceil() as usize;
        let columns = (0..n)
            .map(|i| Column::respawn(i as f32 * col_width))
            .collect();
        Self {
            columns,
            font_size,
            col_width,
        }
    }

    pub fn resize(&mut self, screen_w: f32) {
        let n = (screen_w / self.col_width).ceil() as usize;
        while self.columns.len() < n {
            let x = self.columns.len() as f32 * self.col_width;
            self.columns.push(Column::respawn(x));
        }
        self.columns.truncate(n.max(1));
    }

    pub fn update(&mut self, dt: f32, screen_h: f32) {
        for c in &mut self.columns {
            c.y += c.speed * dt;
            if c.y - (c.length as f32 * self.font_size) > screen_h {
                let x = c.x;
                *c = Column::respawn(x);
            }
        }
    }

    pub fn draw(&self, screen_h: f32, dim: f32) {
        for c in &self.columns {
            for i in 0..c.length {
                let cy = c.y - i as f32 * self.font_size;
                if cy < -self.font_size || cy > screen_h {
                    continue;
                }
                let ch = random_matrix_char();
                let fade = 1.0 - (i as f32 / c.length as f32);
                let color = if i == 0 {
                    Color::new(0.75, 1.0, 0.8, dim)
                } else {
                    Color::new(0.0, 1.0, 0.35, fade * 0.85 * dim)
                };
                draw_text(&ch.to_string(), c.x, cy, self.font_size, color);
            }
        }
    }
}
