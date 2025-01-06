// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/icons.toml
// 67dff7ccae42a7c913508aa95d198a7af6402330e404b366ce4a203837f99618
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/icons.ttf");

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270D}")
}

pub fn left<'a>() -> Text<'a> {
    icon("\u{E75D}")
}

pub fn open<'a>() -> Text<'a> {
    icon("\u{1F4C2}")
}

pub fn right<'a>() -> Text<'a> {
    icon("\u{E75E}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{1F4BE}")
}

fn icon<'a>(codepoint: &'a str) -> Text<'a> {
    text(codepoint).font(Font::with_name("icons"))
}
