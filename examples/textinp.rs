use macroquad::prelude::*;
use flowquad::widgets::container::{ Align, Direction, Container };
use flowquad::widgets::label::Label;
use flowquad::widgets::textinput::TextInput;
use flowquad::widgets::widget::Widget;

#[macroquad::main("Images!")]
async fn main() {
    let poppins = load_ttf_font("examples/poppins.ttf").await.unwrap();
    let label = Label::new("Text Inputs!".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.5, 1.0, 1.0), Some(poppins.clone()), 48.0);
    let textinp = TextInput::new(512.0, 64.0, Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.5, 1.0, 1.0), Some(poppins.clone()));
    let textinp2 = TextInput::new(512.0, 64.0, Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.5, 1.0, 1.0), Some(poppins.clone()));
    let mut container = Container::new(Direction::Vertical, Align::Center, 20.0, Color::new(0.05, 0.05, 0.1, 1.0), None, None);
    container.add_child(Box::new(label));
    container.add_child(Box::new(textinp));
    container.add_child(Box::new(textinp2));

    let mut previous_text = String::new();
    let mut previous_text2 = String::new();

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));
        
        container.update(screen_width() / 2.0 - container.width() / 2.0, screen_height() / 2.0 - container.height() / 2.0);
        container.render(screen_width() / 2.0 - container.width() / 2.0, screen_height() / 2.0 - container.height() / 2.0);
        // println!("Text: {}", container.get_child_as::<TextInput>(1).unwrap().get_text());
        // println!("Text2: {}", container.get_child_as::<TextInput>(2).unwrap().get_text());
        let textinp = container.get_child_as::<TextInput>(1).unwrap();
        let textinp2 = container.get_child_as::<TextInput>(2).unwrap();
        if textinp.get_text() != previous_text {
            println!("Text: {}", textinp.get_text());
            previous_text = textinp.get_text();
        }
        if textinp2.get_text() != previous_text2 {
            println!("Text2: {}", textinp2.get_text());
            previous_text2 = textinp2.get_text();
        }

        next_frame().await;
    }
}
