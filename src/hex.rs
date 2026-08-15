//! Live-updating hex dump panel. Unlike a naive "regenerate everything every
//! frame" approach, this keeps a persistent grid and only mutates a handful
//! of cells per frame — closer to how a real memory/packet dump looks when
//! it's actively changing instead of flickering as random noise.

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::data::random_hex_byte;
use crate::text::text;

pub struct HexPanel {
    rows: usize,
    cols: usize,
    cells: Vec<String>,
    highlight: Vec<f32>,
    base_addr: u32,
}

impl HexPanel {
    pub fn new(rows: usize, cols: usize) -> Self {
        let rows = rows.max(1);
        let cols = cols.max(1);
        let cells = (0..rows * cols).map(|_| random_hex_byte()).collect();
        let highlight = vec![0.0; rows * cols];
        Self {
            rows,
            cols,
            cells,
            highlight,
            base_addr: gen_range(0, 0xFFFF),
        }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        if rows == self.rows && cols == self.cols {
            return;
        }
        *self = Self::new(rows, cols);
    }

    pub fn update(&mut self, dt: f32) {
        for h in &mut self.highlight {
            if *h > 0.0 {
                *h -= dt;
            }
        }
        let mutations = gen_range(1, 4);
        for _ in 0..mutations {
            let idx = gen_range(0, self.cells.len());
            self.cells[idx] = random_hex_byte();
            self.highlight[idx] = 0.4;
        }
    }

    pub fn draw(&self, font: Option<&Font>, x: f32, y: f32) {
        let font_size = 14.0;
        let line_h = 18.0;
        for row in 0..self.rows {
            let addr = self.base_addr.wrapping_add((row * self.cols) as u32);
            let cy = y + row as f32 * line_h;
            text(
                font,
                &format!("0x{addr:04X}"),
                x,
                cy,
                font_size,
                Color::new(0.25, 0.5, 0.28, 0.85),
            );
            for col in 0..self.cols {
                let idx = row * self.cols + col;
                let color = if self.highlight[idx] > 0.0 {
                    Color::new(0.8, 0.95, 0.85, 1.0)
                } else {
                    Color::new(0.25, 0.7, 0.35, 0.9)
                };
                text(
                    font,
                    &self.cells[idx],
                    x + 78.0 + col as f32 * 24.0,
                    cy,
                    font_size,
                    color,
                );
            }
        }
    }
}
