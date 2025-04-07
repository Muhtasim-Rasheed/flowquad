//! This module defines the [`TextInput`] widget that displays text on the screen.
use std::any::Any;

use macroquad::prelude::*;

use super::widget::{Action, Widget};

fn truncate_text_to_fit(text: &str, max_width: f32, font: Option<&Font>, size: u16) -> String {
    let mut result = String::new();
    let mut total_width = 0.0;

    for c in text.chars().rev() {
        let next_width = measure_text(&c.to_string(), font, size, 1.0).width;
        if total_width + next_width > max_width {
            break;
        }
        result.insert(0, c); // Insert at beginning since we’re iterating in reverse
        total_width += next_width;
    }

    result
}

/// The [`TextInput`] widget that displays text on the screen.
pub struct TextInput {
    text: String,
    pos: u32,
    bg: Color,
    fg: Color,
    hover: bool,
    just_clicked: bool,
    selected: bool,
    font: Option<Font>,
    width: f32,
    height: f32,
    cooldown: u32,
}

impl TextInput {
    /// Creates a new [`TextInput`] widget.
    pub fn new(width: f32, height: f32, bg: Color, fg: Color, font: Option<Font>) -> Self {
        Self {
            text: String::new(),
            pos: 0,
            bg,
            fg,
            hover: false,
            just_clicked: false,
            selected: false,
            font,
            width,
            height,
            cooldown: 0,
        }
    }

    /// Returns the text entered in the [`TextInput`] widget.
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Widget for TextInput {
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
        
        let clicked = is_mouse_button_pressed(MouseButton::Left);
        self.hover = mx >= x && mx <= x + self.width && my >= y && my <= y + self.height;
        self.just_clicked = self.hover && clicked;

        if self.just_clicked {
            self.selected = true;
        }
        else if clicked && !self.hover {
            self.selected = false;
        }

        if self.selected && is_key_pressed(KeyCode::Escape) {
            self.selected = false;
        }

        let mut operation_used = false;
        
        if self.cooldown == 0 {
            if self.selected && is_key_down(KeyCode::Backspace) {
                if self.pos > 0 {
                    self.text.remove(self.pos as usize - 1);
                    self.pos -= 1;
                    self.cooldown = 2;
                }
            }
            if self.selected && is_key_down(KeyCode::Delete) {
                if self.pos < self.text.len() as u32 {
                    self.text.remove(self.pos as usize);
                    self.cooldown = 2;
                }
            }
            if self.selected && is_key_down(KeyCode::Left) {
                if self.pos > 0 {
                    self.pos -= 1;
                    self.cooldown = 2;
                }
            }
            if self.selected && is_key_down(KeyCode::Right) {
                if self.pos < self.text.len() as u32 {
                    self.pos += 1;
                    self.cooldown = 2;
                }
            }
        } else if self.cooldown > 0 {
            self.cooldown -= 1;
        }

        if is_key_down(KeyCode::Backspace) || is_key_down(KeyCode::Delete) || is_key_down(KeyCode::Left) || is_key_down(KeyCode::Right) {
            operation_used = true;
        }
        
        if self.selected {
            while let Some(key) = get_char_pressed() {
                if operation_used {
                    return;
                }
                self.text.insert(self.pos as usize, key);
                self.pos += 1;
            }
        }
    }

    fn render(&self, x: f32, y: f32) {
        let bg = if self.hover || self.selected { self.fg } else { self.bg };
        let fg = if self.hover || self.selected { self.bg } else { self.fg };

        draw_rectangle(x, y, self.width, self.height, bg);
        
        let size = (self.height * 0.4) as u16;
        let visible_text = truncate_text_to_fit(&self.text, self.width - 8.0, self.font.as_ref(), size);
        let text_size = measure_text(&visible_text, self.font.as_ref(), size, 1.0);

        draw_text_ex(&visible_text,
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
        
        let text_behind_cursor = self.text.chars().take(self.pos as usize).collect::<String>();
        let text_behind_cursor = truncate_text_to_fit(&text_behind_cursor, self.width - 8.0, self.font.as_ref(), size);
        let text_behind_cursor_size = measure_text(&text_behind_cursor, self.font.as_ref(), size, 1.0);
        let cursor_x = x + text_behind_cursor_size.width + 4.0 + self.width / 2.0 - text_size.width / 2.0;
        draw_line(cursor_x, y + 8.0, cursor_x, y + self.height - 8.0, 3.0, fg);

        draw_rectangle_lines(x, y, self.width, self.height, 4.0, fg);
    }
}

impl Action for TextInput {
    fn is_clicked(&self) -> bool {
        self.just_clicked
    }

    fn is_hovered(&self) -> bool {
        self.hover
    }
}
