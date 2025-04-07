use macroquad::prelude::*;

use super::widget::{Widget, Action};

pub struct Toggle {
    width: f32,
    height: f32,
    text: String,
    bg: Color,
    fg: Color,
    hover: bool,
    toggle: bool,
    click: bool,
    just_clicked: bool,
    font: Option<Font>,
}

impl Toggle {
    pub fn new(width: f32, height: f32, text: String, bg: Color, fg: Color, font: Option<Font>) -> Self {
        Self {
            width,
            height,
            text,
            bg,
            fg,
            hover: false,
            toggle: false,
            click: false,
            just_clicked: false,
            font,
        }
    }
}

impl Widget for Toggle {
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
        self.click = self.hover && is_mouse_button_down(MouseButton::Left);
        self.just_clicked = self.hover && is_mouse_button_pressed(MouseButton::Left);
        self.toggle = if self.just_clicked { !self.toggle } else { self.toggle };
    }

    fn render(&self, x: f32, y: f32) {
        let bg = if self.hover || self.toggle { self.fg } else { self.bg };
        let fg = if self.hover || self.toggle { self.bg } else { self.fg };

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

impl Action for Toggle {
    fn is_clicked(&self) -> bool {
        self.just_clicked
    }

    fn is_hovered(&self) -> bool {
        self.hover
    }
}
