use iced::advanced::widget::Id;
use iced::widget::container;
use iced::widget::{column, row};
use iced::Element;

use crate::menu_bar;
use crate::pokemon_info;
use crate::Message;
use crate::{widgets::party, widgets::pc_box};

use pk_edit::Pokemon;

pub fn party_box<'a>(
    selected: &Option<Id>,
    selected_tab: &Option<Id>,
    selected_pokemon: &Option<Pokemon>,
    party_list: &Vec<Pokemon>,
    pc_i: &usize,
    pc_list: &[Pokemon],
) -> Element<'a, Message> {
    row![
        column![
            menu_bar(selected_tab),
            row![
                iced::widget::Space::with_width(5.0),
                party(selected, party_list),
                pc_box(selected, pc_i, pc_list)
            ]
            .spacing(15),
        ]
        .spacing(15),
        if let Some(selected_pokemon) = selected_pokemon {
            pokemon_info(&selected_pokemon)
        } else {
            container("").into()
        },
    ]
    .spacing(15)
    .into()
}
