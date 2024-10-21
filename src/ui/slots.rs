use iced::widget::{column, container, image, mouse_area, pick_list, row, text};
use iced::widget::{Container, MouseArea};
use iced::{Alignment, Element, Length, Padding};

use crate::message::Message;
use crate::misc::{PROJECT_DIR, WINDOW_WIDTH};
use crate::theme::{slot_appearance, slot_selected_appearance};
use crate::widgets::{gender, level};

use pk_edit::data_structure::pokemon::Pokemon;
use pk_edit::misc::moves;
use pk_edit::StorageType;

pub fn move_slot(
    index: usize,
    move_type: &str,
    move_name: &str,
    pp_used: u8,
    pp_total: u8,
) -> Element<'static, Message> {
    let handle = image::Handle::from_memory(
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
            .width(140)
            .text_line_height(text::LineHeight::Absolute(10.into())),
            iced::widget::Space::with_width(Length::Fill),
            pp(pp_used, pp_total),
            iced::widget::Space::with_width(100)
        ]
        .align_items(Alignment::Center)
        .spacing(10),
    )
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15])
    .into()
}

fn pp(pp_used: u8, pp_total: u8) -> Container<'static, Message> {
    container(text(format!("{}/{}", pp_used, pp_total)))
        .width(60)
        .height(30.0)
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Center)
        .style(slot_appearance())
}

pub fn pc_slot(id: &usize, pokemon: &Pokemon) -> MouseArea<'static, Message> {
    let apperance = if id == &pokemon.offset() && !pokemon.is_empty() {
        slot_selected_appearance()
    } else {
        slot_appearance()
    };
    match pokemon.is_empty() {
        false => {
            let handle = image::Handle::from_memory(
                PROJECT_DIR
                    .get_file(format!(
                        "Pokemon/{:0width$}.png",
                        pokemon.nat_dex_number(),
                        width = 4
                    ))
                    .unwrap()
                    .contents(),
            );

            let image = container(image(handle))
                .width(70.0)
                .height(70.0)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Bottom);

            let container = container(image)
                .width(80.0)
                .height(80.0)
                .style(apperance)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Bottom);

            mouse_area(container).on_press(Message::SelectedPokemon((StorageType::PC, *pokemon)))
        }
        true => {
            let container = container("")
                .width(80.0)
                .height(80.0)
                .style(apperance)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Bottom);

            mouse_area(container).on_press(Message::SelectedPokemon((StorageType::PC, *pokemon)))
        }
    }
}

pub fn party_slot(id: &usize, pokemon: &Pokemon) -> MouseArea<'static, Message> {
    let apperance = if id == &pokemon.offset() && !pokemon.is_empty() {
        slot_selected_appearance()
    } else {
        slot_appearance()
    };
    match pokemon.is_empty() {
        false => {
            let handle = image::Handle::from_memory(
                PROJECT_DIR
                    .get_file(format!(
                        "Pokemon/{:0width$}.png",
                        pokemon.nat_dex_number(),
                        width = 4
                    ))
                    .unwrap()
                    .contents(),
            );

            let level = level(pokemon.level());

            let name = text(pokemon.nickname());

            let image = container(image(handle))
                .width(80.0)
                .height(80.0)
                .align_x(iced::alignment::Horizontal::Right)
                .align_y(iced::alignment::Vertical::Bottom);

            let gender = gender(pokemon.gender());

            let level_gender = row![level, gender].spacing(5);

            let column = column![level_gender, name]
                .width(140.0)
                .height(80.0)
                .spacing(10)
                .padding(Padding::from([10, 10]));

            let container = container(row![column, image])
                .width(240.0)
                .height(80.0)
                .padding(Padding::from([0, 10])) // top/bottom, left/right
                .style(apperance);

            mouse_area(container).on_press(Message::SelectedPokemon((StorageType::Party, *pokemon)))
        }
        true => {
            let container = container("")
                .width(240.0)
                .height(80.0)
                .padding(Padding::from([0, 10])) // top/bottom, left/right
                .style(apperance);

            mouse_area(container).on_press(Message::SelectedPokemon((StorageType::Party, *pokemon)))
        }
    }
}
