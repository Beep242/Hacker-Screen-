//! Thin wrappers around macroquad's text drawing that thread an optional
//! custom font through, so the rest of the app doesn't repeat TextParams.

use macroquad::prelude::*;

pub fn text(
    font: Option<&Font>,
    s: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) -> TextDimensions {
    draw_text_ex(
        s,
        x,
        y,
        TextParams {
            font,
            font_size: size as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color,
        },
    )
}

pub fn measure(font: Option<&Font>, s: &str, size: f32) -> TextDimensions {
    measure_text(s, font, size as u16, 1.0)
}
