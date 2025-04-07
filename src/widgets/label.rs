use macroquad::prelude::*;

use super::widget::Widget;

pub struct Label {
    text: String,
    bg: Color,
    fg: Color,
    font: Option<Font>,
    size: f32,
}

impl Label {
    pub fn new(text: String, bg: Color, fg: Color, font: Option<Font>, size: f32) -> Self {
        Self {
            text,
            bg,
            fg,
            font,
            size,
        }
    }
}

impl Widget for Label {
    fn width(&self) -> f32 {
        let size = self.height() as u16;
        let text_size = measure_text(&self.text, self.font.as_ref(), size, 1.0);
        text_size.width
    }

    fn height(&self) -> f32 {
        self.size
    }

    fn bg(&self) -> Color {
        self.bg
    }

    fn update(&mut self, _x: f32, _y: f32) {
        // Nothing :D
    }

    fn render(&self, x: f32, y: f32) {
        let bg = self.bg;
        let fg = self.fg;

        let size = self.height() as u16;
        
        let text_size = measure_text(&self.text, self.font.as_ref(), size, 1.0);
        draw_rectangle(x, y, self.width(), self.height(), bg);
        draw_text_ex(&self.text,
            x,
            y + text_size.height,
            TextParams {
                font: self.font.as_ref(),
                font_size: size,
                font_scale: 1.0,
                color: fg,
                ..Default::default()
            }
        );
    }
}
