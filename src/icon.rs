// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/icons.toml
// d40580319de235ddd4c22c79cb161e4526bab5e6edc8c3fd36500d61c7e8b83b
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/icons.ttf");

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270D}")
}

pub fn left<'a>() -> Text<'a> {
    icon("\u{E75D}")
}

pub fn minus<'a>() -> Text<'a> {
    icon("\u{2D}")
}

pub fn open<'a>() -> Text<'a> {
    icon("\u{1F4C2}")
}

pub fn plus<'a>() -> Text<'a> {
    icon("\u{2B}")
}

pub fn right<'a>() -> Text<'a> {
    icon("\u{E75E}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{1F4BE}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("icons"))
}
