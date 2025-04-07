//! This module defines the [`Widget`] trait and the [`Action`] trait for UI elements.
use macroquad::prelude::*;
use std::any::Any;

/// The [`Widget`] trait which defines the basic properties and methods for UI elements.
pub trait Widget: Any {
    /// Returns the type of the widget as an [`Any`] type.
    fn as_any(&self) -> &dyn Any;
    /// Returns the width of the widget.
    fn width(&self) -> f32;
    /// Returns the height of the widget.
    fn height(&self) -> f32;
    /// Returns the background color of the widget.
    fn bg(&self) -> Color;
    /// Updates the widget's state based on its position.
    fn update(&mut self, x: f32, y: f32);
    /// Renders the widget at the specified position.
    fn render(&self, x: f32, y: f32);
}

/// The [`Action`] trait which defines the actions that can be performed on UI elements.
pub trait Action {
    /// Returns if the widget is clicked.
    fn is_clicked(&self) -> bool;
    /// Returns if the widget is hovered.
    fn is_hovered(&self) -> bool;
}
