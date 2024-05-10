use iced::color;
use iced::{Border, Color, Shadow};

pub fn gender_f_apperance() -> iced::widget::container::Appearance {
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

pub fn gender_n_apperance() -> iced::widget::container::Appearance {
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

pub fn gender_m_apperance() -> iced::widget::container::Appearance {
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

pub fn slot_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.5))),
        border: Border {
            radius: 5.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

pub fn slot_selected_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0xffcc00))),
        border: Border {
            radius: 5.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

pub fn level_appearance() -> iced::widget::container::Appearance {
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

pub fn party_label_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.5))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

pub fn info_label_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::BLACK),
        background: Some(iced::Background::Color(color!(0xffcc00))),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

pub fn pokemon_info_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.5))),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}
