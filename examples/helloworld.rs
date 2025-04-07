use macroquad::prelude::*;
use flowquad::widgets::button::Button;
use flowquad::widgets::label::Label;
use flowquad::widgets::toggle::Toggle;
use flowquad::widgets::widget::{Widget, Action};

#[macroquad::main("Hello World Example")]
async fn main() { 
    let poppins = load_ttf_font("examples/poppins.ttf").await.unwrap();
    let label = Label::new("Hello, world!".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.5, 1.0, 1.0), Some(poppins.clone()), 36.0);
    let mut button = Button::new(500.0, 80.0, "Clickity Clickity Click".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.75, 0.5, 1.0), Some(poppins.clone()));
    let mut toggle = Toggle::new(150.0, 50.0, "Toggle Me".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(1.0, 1.0, 0.5, 1.0), Some(poppins.clone()));

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));
        
        label.render(screen_width() / 2.0 - label.width() / 2.0, screen_height() / 2.0 - label.height() / 2.0 - 100.0);
        button.update(screen_width() / 2.0 - button.width() / 2.0, screen_height() / 2.0 - button.height() / 2.0 + 100.0);
        button.render(screen_width() / 2.0 - button.width() / 2.0, screen_height() / 2.0 - button.height() / 2.0 + 100.0);
        toggle.update(screen_width() - toggle.width() - 10.0, 10.0);
        toggle.render(screen_width() - toggle.width() - 10.0, 10.0);

        if button.is_clicked() {
            println!("Button clicked!");
        }

        next_frame().await;
    }
}
