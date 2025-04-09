use macroquad::prelude::*;
use flowquad::prelude::*;

#[macroquad::main("Images!")]
async fn main() {
    let poppins = load_ttf_font("examples/poppins.ttf").await.unwrap();
    let label = Label::new("Images!".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.5, 1.0, 1.0), Some(poppins.clone()), 48.0);
    let texture = load_texture("examples/flowquad.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let image = flowquad::widgets::image::Image::new(256.0, 384.0, texture);
    let mut container = Container::new(Direction::Vertical, Align::Center, 20.0, Color::new(0.05, 0.05, 0.1, 1.0), None, None);
    container.add_child(Box::new(label));
    container.add_child(Box::new(image));

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));
        
        container.update(screen_width() / 2.0 - container.width() / 2.0, screen_height() / 2.0 - container.height() / 2.0);
        container.render(screen_width() / 2.0 - container.width() / 2.0, screen_height() / 2.0 - container.height() / 2.0);

        next_frame().await;
    }
}
