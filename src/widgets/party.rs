//! Composite widget rendering the trainer's current party.
//!
//! Displays a labelled column of up to six [`crate::widgets::party_slot`] widgets,
//! each showing the Pokémon's sprite, nickname, level badge, and gender badge.
//! An empty slot renders a blank placeholder at the same size (240 × 80).

use iced::advanced::image;
use iced::advanced::widget::Id;
use iced::widget::{column, container, row, text};
use iced::{Border, Color, Element, Padding};

use std::collections::HashMap;

use crate::message::Message;
use crate::misc::PROJECT_DIR;
use crate::widgets::party_slot;
use crate::DragState;

use pk_edit::{AnyPokemon, PokemonTrait, StorageType};

pub fn party_label<'a>() -> Element<'a, Message> {
    let handle = iced::widget::svg::Handle::from_memory(
        PROJECT_DIR
            .get_file("icons/Pokeball_icon.svg")
            .unwrap()
            .contents(),
    );

    let image = container(
        iced::widget::svg(handle)
            .width(30.0)
            .height(30.0)
            .style(|theme, _| iced::widget::svg::Style {
                color: iced::widget::text::base(theme).color,
            }),
    )
    .align_x(iced::alignment::Horizontal::Right)
    .align_y(iced::alignment::Vertical::Center);

    let row = row![
        image,
        iced::widget::Space::new().width(15).height(40),
        iced::widget::rule::vertical(1),
        iced::widget::Space::new().width(35).height(40),
        text("Current Party")
    ]
    .width(240.0)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .padding(Padding {
        top: 5.0,
        right: 5.0,
        bottom: 5.0,
        left: 20.0,
    }); // top, right, bottom, left

    container(row)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .style(default)
        .into()
}

fn default(theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        //text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(iced::color!(0x000000, 0.5))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: iced::Shadow {
            color: Color::BLACK,
            offset: iced::Vector::new(2.0, 2.0),
            blur_radius: 4.0,
        },
        ..iced::widget::container::rounded_box(theme)
    }
}

pub fn party<'a>(
    selected: &Option<Id>,
    party: &'a [AnyPokemon],
    images: &HashMap<String, image::Handle>,
    drag: &Option<DragState>,
) -> Element<'a, Message> {
    let mut col = iced::widget::Column::new().spacing(10);

    for i in 0..6 {
        let id = Id::from(format!("party_{}", i));
        match party.get(i) {
            Some(pokemon) if !pokemon.is_empty() => {
                col = col.push(
                    party_slot(
                        Some(pokemon),
                        Some(
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
                        ),
                    )
                    .id(id.clone())
                    .selected(selected)
                    .on_press(Message::Selected(
                        Some(id.clone()),
                        Some(StorageType::Party),
                        Some(*pokemon),
                    ))
                    .in_drag_mode(drag.is_some())
                    .is_drag_source(
                        drag.as_ref().is_some_and(|d| {
                            d.index == i && matches!(d.storage, StorageType::Party)
                        }),
                    )
                    .on_drag_start(move |origin| {
                        Message::DragStart(StorageType::Party, origin, pokemon.nat_dex_number(), i)
                    })
                    .on_drop(Message::DragDrop(StorageType::Party, i)),
                );
            }
            Some(pokemon) => {
                col = col.push(
                    party_slot(None, None)
                        .on_press(Message::Selected(
                            Some(id),
                            Some(StorageType::Party),
                            Some(*pokemon),
                        ))
                        .in_drag_mode(drag.is_some())
                        .on_drop(Message::DragDrop(StorageType::Party, i)),
                );
            }
            None => {
                col = col.push(party_slot(None, None).in_drag_mode(drag.is_some()));
            }
        }
    }

    column![party_label(), col].spacing(15).into()
}
