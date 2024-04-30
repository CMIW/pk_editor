use iced::color;
use iced::widget::Container;
use iced::widget::{container, row, text, text_input};
use iced::{Border, Color, Element, Shadow};
use pk_edit::data_structure::pokemon::Gender;

use crate::message::Message;

fn gender_f_apperance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0xd65c63))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn gender_n_apperance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::TRANSPARENT),
        background: Some(iced::Background::Color(Color::TRANSPARENT)),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn gender_m_apperance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x4186d7))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn level_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.7))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn input_appearance() -> iced::widget::text_input::Appearance {
    iced::widget::text_input::Appearance {
        background: iced::Background::Color(Color::TRANSPARENT),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        icon_color: Color::TRANSPARENT,
    }
}

pub fn gender(gender: Gender) -> Container<'static, Message> {
    let (_text, style) = match gender {
        Gender::F => ("", gender_f_apperance()),
        Gender::M => ("", gender_m_apperance()),
        Gender::None => ("", gender_n_apperance()),
    };

    container("").width(26.0).height(26.0).style(style)
}

pub fn level(level: u8) -> Container<'static, Message> {
    container(text(format!("Lv. {}", level)))
        .width(80.0)
        .height(26.0)
        .center_x()
        .center_y()
        .style(level_appearance())
}

pub fn input_level(level: u8) -> Container<'static, Message> {
    let input = text_input(&level.to_string(), &level.to_string())
        .on_input(Message::LevelInputChanged)
        .line_height(text::LineHeight::Absolute(10.into()))
        //.style(input_appearance())
        .width(35)
        .size(12);

    container(row![text("Lv. "), input])
        .width(80.0)
        .height(26.0)
        .center_x()
        .center_y()
        .style(level_appearance())
}
