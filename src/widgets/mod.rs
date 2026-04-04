//! Composite and helper widgets for the `pk_editor` GUI.
//!
//! This module re-exports all custom widget constructors and provides shared
//! helper functions used across multiple screens:
//!
//! - [`move_slot`] — a single move row with type icon, name picker, and PP counter.
//! - [`input_level`] — an editable level pill (`Lv. N`).
//! - [`item_counter`] — a quantity control with `−` / text input / `+` buttons.

pub mod gender;
pub mod level;
pub mod menu_bar;
pub mod party;
pub mod party_slot;
pub mod pc;
pub mod pc_slot;
pub mod pokemon_info;
pub mod slot;
pub mod stat_bar;
pub mod tab;

pub use gender::gender;
use iced::alignment::Horizontal;
pub use level::level;
pub use menu_bar::view;
pub use party::party;
pub use party_slot::party_slot;
pub use pc::pc_box;
pub use pc_slot::pc_slot;
pub use pokemon_info::pokemon_info;
pub use stat_bar::stat_bar;
pub use tab::tab;

use std::collections::HashMap;

use crate::bag::Operation;
use crate::icon;
use crate::misc::WINDOW_WIDTH;
use crate::theme::default_box;
use crate::{bag, item_counter_button_apperance};
use iced::widget::Container;
use iced::widget::{button, text_input};
use iced::widget::{container, image, pick_list, row, text};
use iced::{Alignment, Element};
use pk_edit::Gen3Pocket as Pocket;

use crate::pick_list_default;
use crate::theme::{input_appearance, level_appearance};

pub fn move_slot(
    index: usize,
    move_type: &str,
    move_name: &str,
    pp_used: u8,
    pp_total: u8,
    all_moves: Vec<String>,
    images: &HashMap<String, image::Handle>,
) -> Element<'static, pokemon_info::Message> {
    let move_icon = image(images.get(&format!("{}_icon_SV", move_type)).unwrap_or({
        let width = 10;
        let height = 10;
        let size = (width * height) as usize;
        let pixels = vec![0u8; size * 4];
        &image::Handle::from_rgba(width, height, pixels)
    }))/*.width(45).height(45)*/;

    container(
        row![
            move_icon,
            pick_list(all_moves, Some(move_name.to_string()), move |selection| {
                pokemon_info::Message::MoveSelected(index, selection)
            })
            .width(160)
            .style(pick_list_default),
            pp(pp_used, pp_total),
            iced::widget::Space::new().width(100)
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

fn pp(pp_used: u8, pp_total: u8) -> Container<'static, pokemon_info::Message> {
    container(text(format!("{}/{}", pp_total.saturating_sub(pp_used), pp_total)))
        .width(60)
        .height(30.0)
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Center)
        .style(default_box)
}

pub fn input_level(level: u8) -> Container<'static, pokemon_info::Message> {
    let input = text_input(&level.to_string(), &level.to_string())
        .on_input(pokemon_info::Message::LevelInputChanged)
        .line_height(text::LineHeight::Absolute(14.into()))
        .style(input_appearance)
        .width(35)
        .size(14);

    container(row![text("Lv."), input].width(50))
        .center_x(80.0)
        .center_y(26.0)
        .style(level_appearance)
}

pub fn item_counter(pocket: Pocket, i: usize, quantity: &u16) -> Container<'static, bag::Message> {
    let input = text_input(&quantity.to_string(), &quantity.to_string())
        .on_input(move |input| {
            bag::Message::ItemQuantityChanged(pocket, i, input, Operation::Change)
        })
        .on_paste(move |input| {
            bag::Message::ItemQuantityChanged(pocket, i, input, Operation::Change)
        })
        .line_height(text::LineHeight::Absolute(10.into()))
        .align_x(Horizontal::Center)
        .style(input_appearance)
        .width(30)
        .size(12);

    container(row![
        button(icon::minus().center())
            .on_press(bag::Message::ItemQuantityChanged(
                pocket,
                i,
                quantity.to_string(),
                Operation::Decrement
            ))
            .height(20)
            .width(20)
            .style(item_counter_button_apperance),
        input,
        button(icon::plus().center())
            .on_press(bag::Message::ItemQuantityChanged(
                pocket,
                i,
                quantity.to_string(),
                Operation::Increment
            ))
            .height(20)
            .width(20)
            .style(item_counter_button_apperance),
    ])
    .height(20)
    .center_x(80.0)
    .center_y(26.0)
    .style(level_appearance)
}
