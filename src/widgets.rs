pub mod gender;
pub mod level;
pub mod menu_bar;
pub mod party;
pub mod party_slot;
pub mod pc;
pub mod pc_slot;
pub mod pokemon_info;
pub mod stat_bar;
pub mod tab;

pub use gender::gender;
pub use level::level;
pub use menu_bar::menu_bar;
pub use party::party;
pub use party_slot::party_slot;
pub use pc::pc_box;
pub use pc_slot::pc_slot;
pub use pokemon_info::pokemon_info;
pub use stat_bar::stat_bar;
pub use tab::*;

use iced::widget::Container;
use iced::widget::{container, image, pick_list, row, text};
use iced::{Alignment, Element};

use crate::message::Message;
use crate::misc::{PROJECT_DIR, WINDOW_WIDTH};
use crate::theme::default_box;

use crate::pick_list_default;
use pk_edit::misc::moves;

pub fn move_slot(
    index: usize,
    move_type: &str,
    move_name: &str,
    pp_used: u8,
    pp_total: u8,
) -> Element<'static, Message> {
    let handle = image::Handle::from_bytes(
        PROJECT_DIR
            .get_file(format!("Types/{}_icon_SV.png", move_type))
            .unwrap()
            .contents(),
    );

    let move_icon = image(handle)/*.width(45).height(45)*/;

    let moves = match moves() {
        Ok(ms) => ms,
        Err(err) => {
            println!("{}", err);
            vec![String::from("")]
        }
    };

    container(
        row![
            move_icon,
            pick_list(moves, Some(move_name.to_string()), move |selection| {
                Message::MoveSelected(index, selection)
            })
            .width(160)
            .style(pick_list_default),
            pp(pp_used, pp_total),
            iced::widget::Space::with_width(100)
        ]
        .align_y(Alignment::Center)
        .spacing(10),
    )
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .padding([5, 15])
    .into()
}

fn pp(pp_used: u8, pp_total: u8) -> Container<'static, Message> {
    container(text(format!("{}/{}", pp_used, pp_total)))
        .width(60)
        .height(30.0)
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Center)
        .style(default_box)
}

use iced::color;
use iced::widget::text_input;
use iced::widget::text_input::Status;
use iced::{Border, Color, Shadow};

fn level_appearance(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
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

fn input_appearance(theme: &iced::Theme, status: Status) -> iced::widget::text_input::Style {
    match status {
        Status::Active => iced::widget::text_input::Style {
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

pub fn input_level(level: u8) -> Container<'static, Message> {
    let input = text_input(&level.to_string(), &level.to_string())
        .on_input(Message::LevelInputChanged)
        .line_height(text::LineHeight::Absolute(14.into()))
        .style(input_appearance)
        .width(35)
        .size(14);

    container(row![text("Lv."), input].width(50))
        .center_x(80.0)
        .center_y(26.0)
        .style(level_appearance)
}
