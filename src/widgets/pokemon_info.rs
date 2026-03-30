//! The Pokémon information and editing panel.
//!
//! Occupies the right third of the **Party & Boxes** screen when a Pokémon is selected.
//! The panel is divided into scrollable sections:
//!
//! 1. **Header** — Pokéball picker, nickname, Pokérus toggle, level input, gender badge.
//! 2. **Species** — National Dex number, species combo box, language.
//! 3. **Typing** — type icon(s) for the selected species.
//! 4. **Stats** — `HP` / `Atk` / `Def` / `SpA` / `SpD` / `Spe` with colour-coded bars and IV/EV inputs.
//! 5. **OT info** — original trainer name and ID (read-only).
//! 6. **PID / Friendship** — personality value (read-only display) and editable friendship.
//! 7. **Nature / Ability** — nature pick list and ability text.
//! 8. **Held Item** — item sprite and pick list.
//! 9. **Moves** — up to four [`crate::widgets::move_slot`] rows.
//!
//! Owns its own [`Message`] enum and [`update`] function which validate inputs
//! and mutate the selected [`pk_edit::pokemon::Pokemon`].

use iced::alignment::Horizontal;
use iced::color;
use iced::widget::{button, text_input};
use iced::widget::{
    column, combo_box, container, image, mouse_area, pick_list, row, scrollable, text,
};
use iced::{Alignment, Element, Length};
use pk_edit::error::PokemonError;
use pk_edit::misc::balls_id;
use widgets::generic_overlay::dropdown_root;

use std::collections::HashMap;

use crate::misc::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::theme::{info_label_appearance, pokeball_picker_apperance, pokemon_info_appearance};
use crate::widgets::input_level;

use pk_edit::misc::{held_items, item_id, NATURE};
use pk_edit::pokemon::gen_pokemon_from_species;
use pk_edit::pokemon::{Pokerus, Stats};
use pk_edit::Pokemon;

use crate::stat_bar;
use crate::widgets::gender;
use crate::widgets::move_slot;
use crate::{pick_list_default, text_input_default};

#[derive(Debug, Clone)]
pub enum Message {
    AddMove(usize),
    ChangePokerusStatus,
    ChangePokeball(u8),
    NatureSelected(String),
    SpeciesSelected(String),
    HeldItemSelected(String),
    FriendshipChanged(String),
    LevelInputChanged(String),
    IVChanged(String, String),
    EVChanged(String, String),
    MoveSelected(usize, String),
}

pub fn update(
    selected_pokemon: &mut Option<Pokemon>,
    ot_name: &[u8],
    ot_id: &[u8],
    message: Message,
) -> Result<(), PokemonError> {
    match message {
        Message::ChangePokeball(ball_id) => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                selected_pokemon.set_pokeball_caught(ball_id)?;
            }
            Ok(())
        }
        Message::ChangePokerusStatus => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                match selected_pokemon.pokerus_status() {
                    Pokerus::Infected => {
                        selected_pokemon.cure_pokerus();
                    }
                    Pokerus::Cured => {
                        selected_pokemon.remove_pokerus();
                    }
                    Pokerus::None => {
                        selected_pokemon.infect_pokerus();
                    }
                }
            }
            Ok(())
        }
        Message::LevelInputChanged(mut value) => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                value.retain(|c| c.is_numeric());
                if let Ok(number) = value.parse::<u64>() {
                    let lowest_level = selected_pokemon.lowest_level();
                    let value = if number > 100 {
                        100
                    } else if number < lowest_level as u64 {
                        lowest_level
                    } else {
                        number as u8
                    };
                    selected_pokemon.set_level(value);
                }
            }
            Ok(())
        }
        Message::SpeciesSelected(species) => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                if selected_pokemon.is_empty() {
                    *selected_pokemon =
                        gen_pokemon_from_species(*selected_pokemon, &species, ot_name, ot_id)?;
                } else {
                    selected_pokemon.set_species(&species)?;
                }
            }
            Ok(())
        }
        Message::IVChanged(iv, mut value) => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                value.retain(|c| c.is_numeric());
                if let Ok(number) = value.parse::<u16>() {
                    selected_pokemon.stats_mut().update_ivs(&iv, number);
                } else if value.is_empty() {
                    selected_pokemon.stats_mut().update_ivs(&iv, 0);
                }
            }
            Ok(())
        }
        Message::EVChanged(ev, mut value) => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                value.retain(|c| c.is_numeric());
                if let Ok(number) = value.parse::<u16>() {
                    selected_pokemon.stats_mut().update_evs(&ev, number);
                } else if value.is_empty() {
                    selected_pokemon.stats_mut().update_evs(&ev, 0);
                }
            }
            Ok(())
        }
        Message::FriendshipChanged(mut value) => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                value.retain(|c| c.is_numeric());
                if let Ok(number) = value.parse::<u64>() {
                    let value = if number > u8::MAX.into() {
                        u8::MAX
                    } else {
                        number as u8
                    };
                    selected_pokemon.set_friendship(value);
                }
            }
            Ok(())
        }
        Message::HeldItemSelected(item) => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                selected_pokemon.set_item(&item)?;
            }
            Ok(())
        }
        Message::MoveSelected(index, value) => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                selected_pokemon.set_move(index, &value)?;
            }
            Ok(())
        }
        Message::AddMove(index) => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                selected_pokemon.set_move(index, "Pound")?;
            }
            Ok(())
        }
        Message::NatureSelected(nature) => {
            if let Some(selected_pokemon) = selected_pokemon.as_mut() {
                selected_pokemon.set_nature(&nature);
            }
            Ok(())
        }
    }
}

fn info_label(
    pokemon: &Pokemon,
    images: &HashMap<String, image::Handle>,
) -> Element<'static, Message> {
    let pokeball = if pokemon.pokeball_caught() == 0 {
        dropdown_root("", "")
            .width(45)
            .height(45)
            .style(pokeball_picker_apperance)
    } else {
        dropdown_root(
            image(
                images
                    .get(&format!(
                        "item_{:0width$}",
                        pokemon.pokeball_caught(),
                        width = 4
                    ))
                    .unwrap_or({
                        let width = 10;
                        let height = 10;
                        let size = (width * height) as usize;
                        let pixels = vec![0u8; size * 4];
                        &image::Handle::from_rgba(width, height, pixels)
                    }),
            ),
            scrollable(column(balls_id().unwrap_or_default().iter().filter_map(
                |x| {
                    Some(
                        button(
                            image(images.get(&format!("item_{:0width$}", *x, width = 4))?)
                                .width(45)
                                .height(45),
                        )
                        .on_press(Message::ChangePokeball(*x as u8))
                        .style(button::subtle)
                        .into(),
                    )
                },
            )))
            .height(Length::Fixed(270.0)),
        )
        .overlay_width(Length::Shrink)
        .style(pokeball_picker_apperance)
    };

    let pokerus = match pokemon.pokerus_status() {
        Pokerus::None => image(images.get("PokérusIC_not_infected").unwrap_or({
            let width = 10;
            let height = 10;
            let size = (width * height) as usize;
            let pixels = vec![0u8; size * 4];
            &image::Handle::from_rgba(width, height, pixels)
        }))
        .width(30)
        .height(30),
        Pokerus::Infected => image(images.get("PokérusIC_infected").unwrap_or({
            let width = 10;
            let height = 10;
            let size = (width * height) as usize;
            let pixels = vec![0u8; size * 4];
            &image::Handle::from_rgba(width, height, pixels)
        }))
        .width(30)
        .height(30),
        Pokerus::Cured => image(images.get("PokérusIC_cured").unwrap_or({
            let width = 10;
            let height = 10;
            let size = (width * height) as usize;
            let pixels = vec![0u8; size * 4];
            &image::Handle::from_rgba(width, height, pixels)
        }))
        .width(30)
        .height(30),
    };

    let pokerus = mouse_area(pokerus)
        .interaction(iced::mouse::Interaction::Pointer)
        .on_press(Message::ChangePokerusStatus);

    let name = text(pokemon.nickname()).shaping(text::Shaping::Advanced);

    let row = row![
        pokeball,
        name,
        iced::widget::Space::new().width(Length::Fill),
        pokerus,
        iced::widget::Space::new().width(25),
        input_level(pokemon.level()),
        gender(pokemon.gender()),
    ]
    .align_y(Alignment::Center)
    .spacing(5)
    .padding([5, 20]); // top, right, bottom, left

    container(row)
        .width(WINDOW_WIDTH * 0.33)
        .height(50.0)
        .style(info_label_appearance)
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Left)
        .into()
}

fn stats(stats: Stats, level: u8) -> Element<'static, Message> {
    let (_, highest_stat) = stats.highest_stat(level);
    let scale = if highest_stat >= 400 { 0.23 } else { 0.45 };
    let hp = stats.hp(level);
    let attack = stats.attack(level);
    let defense = stats.defense(level);
    let sp_attack = stats.sp_attack(level);
    let sp_defense = stats.sp_defense(level);
    let speed = stats.speed(level);
    let labels_width = 70;
    let ev_iv_width = 30;

    let column = column![
        row![
            iced::widget::Space::new().width(Length::Fill),
            text("EV").width(ev_iv_width).center(),
            text("IV").width(ev_iv_width).center(),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        row![
            text("HP:").width(labels_width),
            stat_bar(hp as f32 * scale),
            text(hp),
            iced::widget::Space::new().width(Length::Fill),
            text_input(&stats.hp_ev.to_string(), &stats.hp_ev.to_string())
                .on_input(|input| Message::EVChanged(String::from("HP"), input))
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(35)
                .size(12),
            text_input(&stats.hp_iv.to_string(), &stats.hp_iv.to_string())
                .on_input(|input| Message::IVChanged(String::from("HP"), input))
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(30)
                .size(12),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        row![
            text("Attack:").width(labels_width),
            stat_bar(attack as f32 * scale),
            text(attack),
            iced::widget::Space::new().width(Length::Fill),
            text_input(&stats.attack_ev.to_string(), &stats.attack_ev.to_string())
                .on_input(|input| Message::EVChanged(String::from("Attack"), input))
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(35)
                .size(12),
            text_input(&stats.attack_iv.to_string(), &stats.attack_iv.to_string())
                .on_input(|input| Message::IVChanged(String::from("Attack"), input))
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(30)
                .size(12),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        row![
            text("Defense:").width(labels_width),
            stat_bar(defense as f32 * scale),
            text(defense),
            iced::widget::Space::new().width(Length::Fill),
            text_input(&stats.defense_ev.to_string(), &stats.defense_ev.to_string())
                .on_input(|input| Message::EVChanged(String::from("Defense"), input))
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(35)
                .size(12),
            text_input(&stats.defense_iv.to_string(), &stats.defense_iv.to_string())
                .on_input(|input| Message::IVChanged(String::from("Defense"), input))
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(30)
                .size(12),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        row![
            text("Sp. Atk:").width(labels_width),
            stat_bar(sp_attack as f32 * scale),
            text(sp_attack),
            iced::widget::Space::new().width(Length::Fill),
            text_input(
                &stats.sp_attack_ev.to_string(),
                &stats.sp_attack_ev.to_string()
            )
            .on_input(|input| Message::EVChanged(String::from("Sp. Atk"), input))
            .line_height(text::LineHeight::Absolute(10.into()))
            .width(35)
            .size(12),
            text_input(
                &stats.sp_attack_iv.to_string(),
                &stats.sp_attack_iv.to_string()
            )
            .on_input(|input| Message::IVChanged(String::from("Sp. Atk"), input))
            .line_height(text::LineHeight::Absolute(10.into()))
            .width(30)
            .size(12),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        row![
            text("Sp. Def:").width(labels_width),
            stat_bar(sp_defense as f32 * scale),
            text(sp_defense),
            iced::widget::Space::new().width(Length::Fill),
            text_input(
                &stats.sp_defense_ev.to_string(),
                &stats.sp_defense_ev.to_string()
            )
            .on_input(|input| Message::EVChanged(String::from("Sp. Def"), input))
            .line_height(text::LineHeight::Absolute(10.into()))
            .width(35)
            .size(12),
            text_input(
                &stats.sp_defense_iv.to_string(),
                &stats.sp_defense_iv.to_string()
            )
            .on_input(|input| Message::IVChanged(String::from("Sp. Def"), input))
            .line_height(text::LineHeight::Absolute(10.into()))
            .width(30)
            .size(12),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        row![
            text("Speed:").width(labels_width),
            stat_bar(speed as f32 * scale),
            text(speed),
            iced::widget::Space::new().width(Length::Fill),
            text_input(&stats.speed_ev.to_string(), &stats.speed_ev.to_string())
                .on_input(|input| Message::EVChanged(String::from("Speed"), input))
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(35)
                .size(12),
            text_input(&stats.speed_iv.to_string(), &stats.speed_iv.to_string())
                .on_input(|input| Message::IVChanged(String::from("Speed"), input))
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(30)
                .size(12),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
    ]
    .padding([0, 10]) // top/bottom left/right
    .spacing(5);

    container(column).width(WINDOW_WIDTH * 0.33).into()
}

fn pokemon_info_typing(
    typing: Option<(String, Option<String>)>,
    images: &HashMap<String, image::Handle>,
) -> Element<'static, Message> {
    match typing {
        None => container("")
            .width(WINDOW_WIDTH * 0.33)
            .height(40.0)
            .style(pokemon_info_appearance)
            .align_y(iced::alignment::Vertical::Center)
            .align_x(iced::alignment::Horizontal::Left)
            .padding([5, 15])
            .into(),
        Some(tuple) => match tuple {
            (type1, None) => {
                let type1 = image(images.get(&format!("{}IC_SV", type1)).unwrap_or({
                    let width = 10;
                    let height = 10;
                    let size = (width * height) as usize;
                    let pixels = vec![0u8; size * 4];
                    &image::Handle::from_rgba(width, height, pixels)
                }))
                .width((WINDOW_WIDTH * 0.33) * 0.33);

                container(row![type1].spacing(5))
                    .width(WINDOW_WIDTH * 0.33)
                    .height(40.0)
                    .style(pokemon_info_appearance)
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Left)
                    .padding([5, 15])
                    .into()
            }
            (type1, Some(type2)) => {
                let type1 = image(images.get(&format!("{}IC_SV", type1)).unwrap_or({
                    let width = 10;
                    let height = 10;
                    let size = (width * height) as usize;
                    let pixels = vec![0u8; size * 4];
                    &image::Handle::from_rgba(width, height, pixels)
                }))
                .width((WINDOW_WIDTH * 0.33) * 0.33);
                let type2 = image(images.get(&format!("{}IC_SV", type2)).unwrap_or({
                    let width = 10;
                    let height = 10;
                    let size = (width * height) as usize;
                    let pixels = vec![0u8; size * 4];
                    &image::Handle::from_rgba(width, height, pixels)
                }))
                .width((WINDOW_WIDTH * 0.33) * 0.33);

                container(row![type1, type2].spacing(5))
                    .width(WINDOW_WIDTH * 0.33)
                    .height(40.0)
                    .style(pokemon_info_appearance)
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Left)
                    .padding([5, 15])
                    .into()
            }
        },
    }
}

fn info_moves(
    moves: Vec<(String, String, u8, u8)>,
    images: &HashMap<String, image::Handle>,
) -> Element<'static, Message> {
    let mut column = column![];
    let len = moves.len();

    for (index, (typing, name, pp_used, pp)) in moves.into_iter().enumerate() {
        column = column.push(move_slot(index, &typing, &name, pp_used, pp, images));
    }

    for i in 0..4_u8.saturating_sub(len as u8) {
        let position = i as usize + len;
        let row = row![button(text("+").center())
            .on_press(Message::AddMove(position))
            .width(270),]
        .width(WINDOW_WIDTH * 0.33)
        .padding([5, 15])
        .align_y(Alignment::Center);
        column = column.push(row);
    }

    container(column)
        .width(WINDOW_WIDTH * 0.33)
        .height(160)
        .style(pokemon_info_appearance)
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Left)
        .into()
}

pub fn pokemon_info<'a>(
    state: &'a iced::widget::combo_box::State<String>,
    pokemon: &Pokemon,
    images: &HashMap<String, image::Handle>,
) -> Element<'a, Message> {
    let label = info_label(pokemon, images);

    let dex_species_lang = row![
        text(format!("No. {}", pokemon.nat_dex_number())),
        combo_box(
            state,
            "Select Pokemon species...",
            Some(&pokemon.species()),
            Message::SpeciesSelected
        )
        .width(130)
        .input_style(text_input_default),
        iced::widget::Space::new().width(Length::Fill),
        text(pokemon.language().to_string()),
        iced::widget::Space::new().width(45),
    ]
    .spacing(20)
    .align_y(Alignment::Center)
    .padding(iced::Padding {
        top: 5.0,
        right: 10.0,
        bottom: 5.0,
        left: 15.0,
    }); // top, right, bottom, left

    let ot = container(row![row![
        text("OT Name").color(color!(0xffcc00)),
        iced::widget::Space::new().width(10),
        text(pokemon.ot_name()),
    ]
    .width((WINDOW_WIDTH * 0.33) / 2.0),])
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15])
    .style(pokemon_info_appearance);

    let pid_friendship = container(row![
        row![
            text("PID").color(color!(0xffcc00)),
            iced::widget::Space::new().width(30),
            text(format!("{:X}", pokemon.personality_value))
        ]
        .width((WINDOW_WIDTH * 0.33) / 2.0)
        .align_y(Alignment::Center),
        row![
            text("Friendship").color(color!(0xffcc00)),
            iced::widget::Space::new().width(Length::Fill),
            text_input(
                &pokemon.friendship().to_string(),
                &pokemon.friendship().to_string()
            )
            .align_x(Horizontal::Center)
            .on_input(Message::FriendshipChanged)
            .line_height(text::LineHeight::Absolute(10.into()))
            .size(12),
        ]
        .width((WINDOW_WIDTH * 0.33) / 2.0),
    ])
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15]);

    let nature_ability = container(row![
        row![
            text("Nature").color(color!(0xffcc00)),
            iced::widget::Space::new().width(10),
            pick_list(
                NATURE.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                Some(pokemon.nature()),
                Message::NatureSelected
            )
            .width(100)
            .style(pick_list_default),
        ]
        .height(40.0)
        .align_y(iced::alignment::Vertical::Center)
        .width((WINDOW_WIDTH * 0.33) / 2.0),
        row![
            text("Ability").color(color!(0xffcc00)),
            iced::widget::Space::new().width(15),
            text(pokemon.ability()),
        ]
        .height(40.0)
        .align_y(iced::alignment::Vertical::Center)
        .width((WINDOW_WIDTH * 0.33) / 2.0),
    ])
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15])
    .style(pokemon_info_appearance);

    let item_id = item_id(&pokemon.item()).unwrap_or_default();

    let item_image =
        if let Some(handle) = images.get(&format!("item_{:0width$}", item_id, width = 4)) {
            image(handle).height(25)
        } else {
            image("").height(30)
        };

    let held_items = match held_items() {
        Ok(is) => is,
        Err(_) => {
            vec![String::from("")]
        }
    };

    let item = container(
        row![
            text("Held Item").color(color!(0xffcc00)),
            iced::widget::Space::new().width(5),
            item_image,
            iced::widget::Space::new().width(5),
            pick_list(held_items, Some(pokemon.item()), Message::HeldItemSelected)
                .width(150)
                .style(pick_list_default),
            iced::widget::Space::new().width(Length::Fill),
        ]
        .align_y(Alignment::Center),
    )
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15]);

    let moves = info_moves(pokemon.moves(), images);

    container(column![
        label,
        dex_species_lang,
        pokemon_info_typing(pokemon.typing(), images),
        iced::widget::Space::new().width(10),
        stats(pokemon.stats, pokemon.level()),
        iced::widget::Space::new().width(10),
        ot,
        pid_friendship,
        nature_ability,
        item,
        moves,
        row![].height(30)
    ])
    .width(WINDOW_WIDTH * 0.33)
    .height(WINDOW_HEIGHT)
    .style(pokemon_info_appearance)
    .align_y(iced::alignment::Vertical::Top)
    .align_x(iced::alignment::Horizontal::Center)
    .into()
}
