//! Composite widget rendering a PC box.
//!
//! Provides two functions:
//! - [`pc_box`] — a 6 × 5 grid of [`crate::widgets::pc_slot`] widgets for the 30 slots of one box.
//! - `pc_label` (private) — a header row with left/right navigation buttons and the current box number.
//!
//! Box navigation emits [`crate::Message::Increment`] and [`crate::Message::Decrement`].

use iced::advanced::widget::Id;
use iced::widget::image;
use iced::widget::{button, column, row};
use iced::Element;

use itertools::Itertools;
use std::collections::HashMap;

use crate::DragState;
use crate::Message;
use crate::{icon, pc_slot};
use crate::{shadow_box, tab_bar_button_primary};

use pk_edit::StorageType;
use pk_edit::{AnyPokemon, PokemonTrait};

pub fn pc_label<'a>(
    pc_i: usize,
    images: &HashMap<String, image::Handle>,
    scale: f32,
) -> Element<'a, Message> {
    row![
        button(icon::left().size(25.0 * scale).center())
            .on_press(Message::Decrement)
            .height(40.0 * scale)
            .style(tab_bar_button_primary),
        iced::widget::container(
            row![
                image(images.get("pokebox_icon").unwrap_or({
                    let width = 0;
                    let height = 0;
                    let size = (width * height) as usize;
                    let pixels = vec![0u8; size * 4];
                    &image::Handle::from_rgba(width, height, pixels)
                }))
                .height(30.0 * scale),
                iced::widget::text(format!("Box {}", pc_i))
            ]
            .spacing(10.0 * scale)
            .align_y(iced::alignment::Vertical::Center)
        )
        .width(350.0 * scale)
        .height(40.0 * scale)
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Center)
        .style(shadow_box),
        button(icon::right().size(25.0 * scale).center())
            .on_press(Message::Increment)
            .height(40.0 * scale)
            .style(tab_bar_button_primary),
    ]
    .spacing(10.0 * scale)
    .align_y(iced::alignment::Vertical::Center)
    .into()
}

pub fn pc_box<'a>(
    selected: &Option<Id>,
    pc_i: &usize,
    pc_list: &'a [AnyPokemon],
    images: &HashMap<String, image::Handle>,
    drag: &Option<DragState>,
    scale: f32,
) -> Element<'a, Message> {
    let mut col = iced::widget::Column::new()
        .align_x(iced::Alignment::Center)
        .spacing(10.0 * scale);
    for chunk in pc_list.iter().enumerate().chunks(6).into_iter() {
        let mut pc_row = iced::widget::Row::new().spacing(10.0 * scale);
        for (index, pokemon) in chunk {
            let global_index = pc_i * pc_list.len() + index;
            let id = Id::from(format!("pc-{global_index}"));
            pc_row = pc_row.push(if !pokemon.is_empty() {
                pc_slot(Some(
                    images
                        .get(&format!("{:0width$}", pokemon.nat_dex_number(), width = 4))
                        .unwrap_or({
                            let width = 10;
                            let height = 10;
                            let size = (width * height) as usize;
                            let pixels = vec![0u8; size * 4];
                            &image::Handle::from_rgba(width, height, pixels)
                        })
                        .clone(),
                ))
                .scale(scale)
                .id(id.clone())
                .selected(selected)
                .in_drag_mode(drag.is_some())
                .is_drag_source(
                    drag.as_ref()
                        .is_some_and(|d| d.index == index && matches!(d.storage, StorageType::PC)),
                )
                .on_press(Message::Selected(
                    Some(id.clone()),
                    Some(StorageType::PC),
                    Some(*pokemon),
                ))
                .on_drag_start(move |origin| {
                    Message::DragStart(StorageType::PC, origin, pokemon.nat_dex_number(), index)
                })
                .on_drop(Message::DragDrop(StorageType::PC, index))
            } else {
                pc_slot(None)
                    .scale(scale)
                    .on_press(Message::Selected(
                        Some(id),
                        Some(StorageType::PC),
                        Some(*pokemon),
                    ))
                    .in_drag_mode(drag.is_some())
                    .on_drop(Message::DragDrop(StorageType::PC, index))
            });
        }
        col = col.push(pc_row);
    }

    column![pc_label(pc_i + 1, images, scale), col,]
        .align_x(iced::Alignment::Center)
        .spacing(15.0 * scale)
        .into()
}
