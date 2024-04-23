use iced::application::Application;

use iced::{Font, Settings, Size};

use test_ui::ui::{State, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() -> iced::Result {
    State::run(Settings {
        window: iced::window::Settings {
            size: Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: false,
            ..iced::window::Settings::default()
        },
        default_font: Font::with_name("Mukta Vaani"),
        ..Settings::default()
    })
}
