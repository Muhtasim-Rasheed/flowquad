//! This module defines the [`Button`] widget that can be clicked to perform an action.
use std::any::Any;

use macroquad::prelude::*;

use super::widget::{Widget, Action};

/// A [`Button`] widget that can be clicked to perform an action.
pub struct Button {
    width: f32,
    height: f32,
    text: String,
    bg: Color,
    fg: Color,
    hover: bool,
    click: bool,
    font: Option<Font>,
}

impl Button {
    /// Creates a new [`Button`] widget.
    pub fn new(width: f32, height: f32, text: String, bg: Color, fg: Color, font: Option<Font>) -> Self {
        Self {
            width,
            height,
            text,
            bg,
            fg,
            hover: false,
            click: false,
            font,
        }
    }
}

impl Widget for Button {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }

    fn bg(&self) -> Color {
        self.bg
    }

    fn update(&mut self, x: f32, y: f32) {
        let mouse_pos = mouse_position();
        let mx = mouse_pos.0;
        let my = mouse_pos.1;

        self.hover = mx >= x && mx <= x + self.width && my >= y && my <= y + self.height;
        self.click = self.hover && is_mouse_button_pressed(MouseButton::Left);
    }

    fn render(&self, x: f32, y: f32) {
        let bg = if self.hover { self.fg } else { self.bg };
        let fg = if self.hover { self.bg } else { self.fg };

        draw_rectangle(x, y, self.width, self.height, bg);
        
        let size = (self.height * 0.4) as u16;
        
        let text_size = measure_text(&self.text, self.font.as_ref(), size, 1.0);
        draw_text_ex(&self.text,
            x + self.width / 2.0 - text_size.width / 2.0,
            y + self.height / 2.0 + text_size.height / 4.0,
            TextParams {
                font: self.font.as_ref(),
                font_size: size,
                font_scale: 1.0,
                color: fg,
                ..Default::default()
            }
        );

        draw_rectangle_lines(x, y, self.width, self.height, 4.0, fg);
    }
}

impl Action for Button {
    fn is_clicked(&self) -> bool {
        self.click
    }

    fn is_hovered(&self) -> bool {
        self.hover
    }
}
