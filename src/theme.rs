//! Styling functions for the `pk_editor` GUI.
//!
//! Each function returns a style value accepted by the corresponding Iced widget.
//! The overall visual language uses semi-transparent dark backgrounds
//! (`color!(0x000000, 0.5)`) with soft drop shadows and rounded corners,
//! complementing the built-in **Dracula** theme.

use iced::color;
use iced::Vector;
use iced::{Border, Color, Shadow};

pub fn slot_appearance(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.5))),
        border: Border {
            radius: 5.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(2.0, 2.0),
            blur_radius: 4.0,
        },
        ..Default::default()
    }
}

pub fn party_label_appearance(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.5))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(2.0, 2.0),
            blur_radius: 4.0,
        },
        ..Default::default()
    }
}

pub fn info_label_appearance(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(Color::BLACK),
        background: Some(iced::Background::Color(color!(0xffcc00))),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

pub fn item_counter_apperance(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.7))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

pub fn item_counter_button_apperance(
    theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    match status {
        iced::widget::button::Status::Active | iced::widget::button::Status::Disabled => {
            iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
                text_color: iced::widget::button::text(theme, status).text_color,
                border: Border {
                    radius: 20.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                ..iced::widget::button::primary(theme, status)
            }
        }
        _ => iced::widget::button::Style {
            text_color: iced::widget::button::text(theme, status).text_color,
            border: Border {
                radius: 20.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..iced::widget::button::primary(theme, status)
        },
    }
}

pub fn pokemon_info_appearance(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.5))),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
        ..Default::default()
    }
}

pub fn tab_bar_button_primary(
    theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    match status {
        iced::widget::button::Status::Active => iced::widget::button::Style {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            text_color: iced::widget::button::text(theme, status).text_color,
            ..iced::widget::button::primary(theme, status)
        },
        _ => iced::widget::button::Style {
            text_color: iced::widget::button::text(theme, status).text_color,
            ..iced::widget::button::primary(theme, status)
        },
    }
}

pub fn pick_list_default(
    theme: &iced::Theme,
    status: iced::widget::pick_list::Status,
) -> iced::widget::pick_list::Style {
    let palette = theme.extended_palette();
    match status {
        iced::widget::pick_list::Status::Active
        | iced::widget::pick_list::Status::Opened { is_hovered: true } => {
            iced::widget::pick_list::Style {
                background: iced::Background::Color(iced::Color::TRANSPARENT),
                placeholder_color: palette.background.base.color,
                border: iced::Border::default(),
                ..iced::widget::pick_list::default(theme, status)
            }
        }
        _ => iced::widget::pick_list::Style {
            background: iced::Background::Color(iced::Color::TRANSPARENT),
            placeholder_color: palette.background.base.color,
            ..iced::widget::pick_list::default(theme, status)
        },
    }
}

pub fn text_input_default(
    theme: &iced::Theme,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let palette = theme.extended_palette();
    match status {
        iced::widget::text_input::Status::Active
        | iced::widget::text_input::Status::Focused { is_hovered: true } => {
            iced::widget::text_input::Style {
                background: iced::Background::Color(iced::Color::TRANSPARENT),
                placeholder: palette.background.base.color,
                border: iced::Border::default(),
                ..iced::widget::text_input::default(theme, status)
            }
        }
        _ => iced::widget::text_input::Style {
            background: iced::Background::Color(iced::Color::TRANSPARENT),
            placeholder: palette.background.base.color,
            ..iced::widget::text_input::default(theme, status)
        },
    }
}

pub fn tab_bar_tab(theme: &iced::Theme, status: crate::tab::Status) -> crate::tab::Style {
    crate::tab::primary(theme, status)
}

pub fn menu_bar_default(theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::color!(0x000000, 0.5))),
        ..iced::widget::container::rounded_box(theme)
    }
}

pub fn default_box(theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::color!(0x000000, 0.5))),
        border: iced::Border {
            radius: 5.0.into(),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
        ..iced::widget::container::rounded_box(theme)
    }
}

pub fn shadow_box(theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::color!(0x000000, 0.5))),
        border: iced::Border {
            radius: 5.0.into(),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
        shadow: iced::Shadow {
            color: iced::Color::BLACK,
            offset: iced::Vector::new(2.0, 2.0),
            blur_radius: 4.0,
        },
        ..iced::widget::container::rounded_box(theme)
    }
}

pub fn shadow_box_light(theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::color!(0x000000, 0.2))),
        border: iced::Border {
            radius: 5.0.into(),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
        shadow: iced::Shadow {
            color: iced::Color::BLACK,
            offset: iced::Vector::new(2.0, 2.0),
            blur_radius: 4.0,
        },
        ..iced::widget::container::rounded_box(theme)
    }
}

pub fn level_appearance(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.7))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

pub fn input_appearance(
    theme: &iced::Theme,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    match status {
        iced::widget::text_input::Status::Active => iced::widget::text_input::Style {
            background: iced::Background::Color(Color::TRANSPARENT),
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..iced::widget::text_input::default(theme, status)
        },
        _ => iced::widget::text_input::Style {
            background: iced::Background::Color(Color::TRANSPARENT),
            ..iced::widget::text_input::default(theme, status)
        },
    }
}

pub fn pokeball_picker_apperance(
    theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: None,
        ..iced::widget::button::subtle(theme, status)
    }
}
