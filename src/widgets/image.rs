use macroquad::prelude::*;

use crate::widgets::widget::Widget;

pub struct Image {
    height: f32,
    width: f32,
    texture: Texture2D
}

impl Image {
    pub fn new(height: f32, width: f32, texture: Texture2D) -> Self {
        Self {
            height,
            width,
            texture
        }
    }
}

impl Widget for Image {
    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }

    fn bg(&self) -> Color {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    fn update(&mut self, _x: f32, _y: f32) {
        // Nothing :D
    }

    fn render(&self, x: f32, y: f32) {
        draw_texture_ex(&self.texture, x, y, WHITE, DrawTextureParams {
            dest_size: Some(vec2(self.width, self.height)),
            ..Default::default()
        });
    }
}
