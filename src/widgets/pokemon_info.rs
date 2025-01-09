use iced::color;
use iced::widget::{button, text_input};
use iced::widget::{column, container, image, mouse_area, combo_box, pick_list, row, text};
use iced::{Alignment, Element, Length};

use crate::message::Message;
use crate::misc::{PROJECT_DIR, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::theme::{info_label_appearance, pokemon_info_appearance};
use crate::widgets::input_level;

use pk_edit::data_structure::pokemon::{Pokemon, Pokerus, Stats};
use pk_edit::misc::{held_items, item_id, species, NATURE};

use crate::{pick_list_default, text_input_default};
use crate::stat_bar;
use crate::widgets::gender;
use crate::widgets::move_slot;

fn info_label(pokemon: &Pokemon) -> Element<'static, Message> {
    let pokeball = if pokemon.pokeball_caught() == 0 {
        image("").width(45).height(45)
    } else {
        let handle = image::Handle::from_bytes(
            PROJECT_DIR
                .get_file(format!(
                    "Items/item_{:0width$}.png",
                    pokemon.pokeball_caught(),
                    width = 4
                ))
                .unwrap()
                .contents(),
        );

        image(handle).width(45).height(45)
    };

    let pokerus = match pokemon.pokerus_status() {
        Pokerus::None => {
            let pokerus_handle = image::Handle::from_bytes(
                PROJECT_DIR
                    .get_file("icons/PokérusIC_not_infected.png")
                    .unwrap()
                    .contents(),
            );
            image(pokerus_handle).width(30).height(30)
        }
        Pokerus::Infected => {
            let pokerus_handle = image::Handle::from_bytes(
                PROJECT_DIR
                    .get_file("icons/PokérusIC_infected.png")
                    .unwrap()
                    .contents(),
            );
            image(pokerus_handle).width(30).height(30)
        }
        Pokerus::Cured => {
            let pokerus_handle = image::Handle::from_bytes(
                PROJECT_DIR
                    .get_file("icons/PokérusIC_cured.png")
                    .unwrap()
                    .contents(),
            );
            image(pokerus_handle).width(30).height(30)
        }
    };

    let pokerus = mouse_area(pokerus)
        .interaction(iced::mouse::Interaction::Pointer)
        .on_press(Message::ChangePokerusStatus);

    let name = text(pokemon.nickname()).shaping(text::Shaping::Advanced);

    let row = row![
        pokeball,
        name,
        iced::widget::Space::with_width(Length::Fill),
        pokerus,
        iced::widget::Space::with_width(25),
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
            iced::widget::Space::with_width(Length::Fill),
            text("EV")
                .width(ev_iv_width)
                /*.horizontal_alignment(iced::alignment::Horizontal::Center)*/,
            text("IV")
                .width(ev_iv_width)
                /*.horizontal_alignment(iced::alignment::Horizontal::Center)*/,
        ]
        .spacing(5)
        .align_y(Alignment::Center),
        row![
            text("HP:").width(labels_width),
            stat_bar(hp as f32 * scale),
            text(hp),
            iced::widget::Space::with_width(Length::Fill),
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
            iced::widget::Space::with_width(Length::Fill),
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
            iced::widget::Space::with_width(Length::Fill),
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
            iced::widget::Space::with_width(Length::Fill),
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
            iced::widget::Space::with_width(Length::Fill),
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
            iced::widget::Space::with_width(Length::Fill),
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

fn pokemon_info_typing(typing: Option<(String, Option<String>)>) -> Element<'static, Message> {
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
                let handle1 = image::Handle::from_bytes(
                    PROJECT_DIR
                        .get_file(format!("Types/{}IC_SV.png", type1))
                        .unwrap()
                        .contents(),
                );

                let type1 = image(handle1).width((WINDOW_WIDTH * 0.33) * 0.33);

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
                let handle1 = image::Handle::from_bytes(
                    PROJECT_DIR
                        .get_file(format!("Types/{}IC_SV.png", type1))
                        .unwrap()
                        .contents(),
                );

                let handle2 = image::Handle::from_bytes(
                    PROJECT_DIR
                        .get_file(format!("Types/{}IC_SV.png", type2))
                        .unwrap()
                        .contents(),
                );

                let type1 = image(handle1).width((WINDOW_WIDTH * 0.33) * 0.33);
                let type2 = image(handle2).width((WINDOW_WIDTH * 0.33) * 0.33);

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

fn info_moves(moves: Vec<(String, String, u8, u8)>) -> Element<'static, Message> {
    let mut column = column![];
    let len = moves.len();

    for (index, (typing, name, pp_used, pp)) in moves.into_iter().enumerate() {
        column = column.push(move_slot(index, &typing, &name, pp_used, pp));
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

pub fn pokemon_info<'a>(state: &'a iced::widget::combo_box::State<String>, pokemon: &Pokemon) -> Element<'a, Message> {
    let label = info_label(pokemon);

    let dex_species_lang = row![
        text(format!("No. {}", pokemon.nat_dex_number())),
        combo_box(state, "Select Pokemon species...", Some(&pokemon.species()), Message::SpeciesSelected)
            .width(130)
            .input_style(text_input_default),
        iced::widget::Space::with_width(Length::Fill),
        text(pokemon.language().to_string()),
        iced::widget::Space::with_width(45),
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
        iced::widget::Space::with_width(10),
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
            iced::widget::Space::with_width(30),
            text(format!("{:X}", pokemon.personality_value()))
        ]
        .width((WINDOW_WIDTH * 0.33) / 2.0)
        .align_y(Alignment::Center),
        row![
            text("Friendship").color(color!(0xffcc00)),
            iced::widget::Space::with_width(Length::Fill),
            text_input(
                &pokemon.friendship().to_string(),
                &pokemon.friendship().to_string()
            )
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
            iced::widget::Space::with_width(10),
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
            iced::widget::Space::with_width(15),
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

    let item_id = match item_id(&pokemon.held_item()) {
        Ok(id) => id,
        Err(_) => 0,
    };

    let item_image = if let Some(item_image) =
        PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_id, width = 4))
    {
        let handle = image::Handle::from_bytes(item_image.contents());
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
            iced::widget::Space::with_width(5),
            item_image,
            iced::widget::Space::with_width(5),
            pick_list(
                held_items,
                Some(pokemon.held_item()),
                Message::HeldItemSelected
            )
            .width(150)
            .style(pick_list_default),
            iced::widget::Space::with_width(Length::Fill),
        ]
        .align_y(Alignment::Center),
    )
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15]);

    let moves = info_moves(pokemon.moves());

    container(column![
        label,
        dex_species_lang,
        pokemon_info_typing(pokemon.typing()),
        iced::widget::Space::with_height(10),
        stats(pokemon.stats(), pokemon.level()),
        iced::widget::Space::with_height(10),
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
