use macroquad::prelude::*;

pub trait Widget {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn bg(&self) -> Color;
    fn update(&mut self, x: f32, y: f32);
    fn render(&self, x: f32, y: f32);
}

pub trait Action {
    fn is_clicked(&self) -> bool;
    fn is_hovered(&self) -> bool;
}
