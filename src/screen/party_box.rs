//! The **Party & Boxes** screen.
//!
//! Renders the main editing view consisting of:
//! - A [`crate::menu_bar`] at the top for file operations and tab navigation.
//! - A party panel (left) showing the trainer's current party of up to 6 Pokémon.
//! - A PC box panel (centre) showing a 6 × 5 grid of slots for the active box.
//! - A Pokémon info panel (right) that appears when a slot is selected.

use iced::advanced::widget::Id;
use iced::widget::container;
use iced::widget::image;
use iced::widget::{column, row};
use iced::Element;

use std::collections::HashMap;

use crate::menu_bar;
use crate::pokemon_info;
use crate::DragState;
use crate::Message;
use crate::{widgets::party, widgets::pc_box};

use pk_edit::{AnyGameData, AnyPokemon};

const SPACING: f32 = 15.0;

pub fn party_box<'a>(
    cb_state: &'a iced::widget::combo_box::State<String>,
    selected: &Option<Id>,
    selected_tab: &Option<Id>,
    selected_pokemon: &Option<AnyPokemon>,
    game_data: &AnyGameData,
    party_list: &'a [AnyPokemon],
    pc_i: &usize,
    pc_list: &'a [AnyPokemon],
    images: &HashMap<String, image::Handle>,
    drag: &Option<DragState>,
    scale: f32,
) -> Element<'a, Message> {
    let info_w = 330.0 * scale;

    row![
        column![
            menu_bar::view(selected_tab, images).map(Message::MenuBar),
            row![
                iced::widget::Space::new().width(5.0 * scale),
                party(selected, party_list, images, drag),
                pc_box(selected, pc_i, pc_list, images, drag, scale)
            ]
            .spacing(SPACING * scale),
        ]
        .spacing(SPACING * scale),
        if let Some(selected_pokemon) = selected_pokemon {
            pokemon_info(cb_state, selected_pokemon, game_data, images, scale)
                .map(Message::PokemonInfo)
        } else {
            container("").width(info_w).into()
        },
    ]
    .spacing(SPACING * scale)
    .into()
}
