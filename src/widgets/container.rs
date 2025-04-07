//! This module defines the [`Container`] widget, which is a container for other widgets.
//!
//! This module contains the following:
//! 1. The [`Container`] struct, which is a container for other widgets, it itself implements the
//!     [`Widget`] trait.
//! 2. The [`Direction`] enum, which defines the direction of the container, either horizontal or
//!     vertical.
//! 3. The [`Align`] enum, which defines the alignment of the container, either start, center or
//!     end.
use std::any::Any;

use macroquad::prelude::*;

use super::widget::Widget;

/// The [`Direction`] enum defines the direction of the container, either horizontal or vertical.
pub enum Direction {
    Horizontal,
    Vertical,
}

/// The [`Align`] enum defines the alignment of the container, either start, center or end.
pub enum Align {
    Start,
    Center,
    End,
}

/// The [`Container`] struct is a container for other widgets, it itself implements the [`Widget`],
/// making it nest-able. It can contain other widgets and arrange them in a specified [`Direction`]
/// and [`Align`]ment.
pub struct Container {
    direction: Direction,
    align: Align,
    gap: f32,
    children: Vec<Box<dyn Widget>>,
    bg: Color,
    padding: Option<(f32, f32, f32, f32)>,
    border: Option<(f32, Color)>,
}

impl Container {
    /// Creates a new [`Container`] widget.
    pub fn new(direction: Direction, align: Align, gap: f32, bg: Color, padding: Option<(f32, f32, f32, f32)>, border: Option<(f32, Color)>) -> Self {
        Self {
            direction,
            align,
            gap,
            children: Vec::new(),
            bg,
            padding,
            border,
        }
    }
    
    /// Adds a child [`Widget`] to the container.
    pub fn add_child(&mut self, child: Box<dyn Widget>) {
        self.children.push(child);
    }

    /// Gets a child [`Widget`] from the container.
    pub fn get_child(&self, index: usize) -> Option<&Box<dyn Widget>> {
        self.children.get(index)
    }

    /// Gets a child [`Widget`] from the container and downcasts it to the specified type.
    pub fn get_child_as<T: 'static>(&self, index: usize) -> Option<&T> {
        self.children.get(index)?
            .as_any()
            .downcast_ref::<T>()
    }
}

impl Widget for Container {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn width(&self) -> f32 {
        let base_width = match self.direction {
            Direction::Horizontal => {
                let mut width = 0.0;
                for child in &self.children {
                    width += child.width() + self.gap;
                }
                width - self.gap
            }
            Direction::Vertical => self.children.iter().map(|c| c.width()).fold(0.0, f32::max),
        };

        if let Some((left, right, _, _)) = self.padding {
            base_width + left + right
        } else {
            base_width
        }
    }

    fn height(&self) -> f32 {
        let base_height = match self.direction {
            Direction::Horizontal => self.children.iter().map(|c| c.height()).fold(0.0, f32::max),
            Direction::Vertical => {
                let mut height = 0.0;
                for child in &self.children {
                    height += child.height() + self.gap;
                }
                height - self.gap
            }
        };

        if let Some((_, _, top, bottom)) = self.padding {
            base_height + top + bottom
        } else {
            base_height
        }
    }

    fn bg(&self) -> Color {
        self.bg
    }

    fn update(&mut self, x: f32, y: f32) {
        let (pad_left, pad_right, pad_top, pad_bottom) = self.padding.unwrap_or((0.0, 0.0, 0.0, 0.0));

        let container_width = self.width() - pad_left - pad_right;
        let container_height = self.height() - pad_top - pad_bottom;

        let mut x = x + pad_left;
        let mut y = y + pad_top;

        for child in self.children.iter_mut() {
            let child_width = child.width();
            let child_height = child.height();

            let offset_x = match self.direction {
                Direction::Horizontal => 0.0,
                Direction::Vertical => match self.align {
                    Align::Start => 0.0,
                    Align::Center => (container_width - child_width) / 2.0,
                    Align::End => container_width - child_width,
                },
            };

            let offset_y = match self.direction {
                Direction::Horizontal => match self.align {
                    Align::Start => 0.0,
                    Align::Center => (container_height - child_height) / 2.0,
                    Align::End => container_height - child_height,
                },
                Direction::Vertical => 0.0,
            };

            // Apply alignment offsets
            child.update(x + offset_x, y + offset_y);

            match self.direction {
                Direction::Horizontal => x += child_width + self.gap,
                Direction::Vertical => y += child_height + self.gap,
            }
        }
    }

    fn render(&self, x: f32, y: f32) {
        let (pad_left, pad_right, pad_top, pad_bottom) = self.padding.unwrap_or((0.0, 0.0, 0.0, 0.0));
        let padded_x = x + pad_left;
        let padded_y = y + pad_top;
        let original_x = x;
        let original_y = y;
        let width = self.width();
        let height = self.height();

        draw_rectangle(x, y, width, height, self.bg);

        let mut x = padded_x;
        let mut y = padded_y;

        for child in &self.children {
            let offset_x = match self.direction {
                Direction::Horizontal => 0.0,
                Direction::Vertical => match self.align {
                    Align::Start => 0.0,
                    Align::Center => (self.width() - pad_left - pad_right - child.width()) / 2.0,
                    Align::End => self.width() - pad_left - pad_right - child.width(),
                },
            };

            let offset_y = match self.direction {
                Direction::Horizontal => match self.align {
                    Align::Start => 0.0,
                    Align::Center => (self.height() - pad_top - pad_bottom - child.height()) / 2.0,
                    Align::End => self.height() - pad_top - pad_bottom - child.height(),
                },
                Direction::Vertical => 0.0,
            };

            child.render(x + offset_x, y + offset_y);

            match self.direction {
                Direction::Horizontal => x += child.width() + self.gap,
                Direction::Vertical => y += child.height() + self.gap,
            }
        }

        if let Some((border_width, border_color)) = self.border {
            draw_rectangle_lines(original_x, original_y, width, height, border_width, border_color);
        }
    }
}
