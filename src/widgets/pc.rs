use iced::advanced::widget::Id;
use iced::widget::image;
use iced::widget::{button, column, row};
use iced::Element;

use crate::misc::PROJECT_DIR;
use crate::Message;
use crate::{icon, pc_slot};
use crate::{shadow_box, tab_bar_button_primary};

use pk_edit::{Pokemon, StorageType};

pub fn pc_label<'a>(pc_i: usize) -> Element<'a, Message> {
    row![
        button(icon::left().size(25).center())
            .on_press(Message::Decrement)
            .height(40.0)
            .style(tab_bar_button_primary),
        iced::widget::container(
            row![
                image(image::Handle::from_bytes(
                    PROJECT_DIR
                        .get_file("icons/pokebox_icon.png")
                        .unwrap()
                        .contents()
                ))
                .height(30.0),
                iced::widget::text(format!("Box {}", pc_i))
            ]
            .spacing(10.0)
            .align_y(iced::alignment::Vertical::Center)
        )
        .width(350)
        .height(40.0)
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Center)
        .style(shadow_box),
        button(icon::right().size(25).center())
            .on_press(Message::Increment)
            .height(40.0)
            .style(tab_bar_button_primary),
    ]
    .spacing(10)
    .align_y(iced::alignment::Vertical::Center)
    .into()
}

pub fn pc_box<'a>(
    selected: &Option<Id>,
    pc_i: &usize,
    pc_list: &[Pokemon],
) -> Element<'a, Message> {
    let mut col = iced::widget::Column::new()
        .align_x(iced::Alignment::Center)
        .spacing(10);

    for row in pc_list.chunks(6) {
        let mut pc_row = iced::widget::Row::new().spacing(10);
        for pokemon in row {
            pc_row = pc_row.push(if pokemon.is_empty() {
                pc_slot(None).on_press(Message::Selected(
                    Some(Id::new(pokemon.offset().to_string())),
                    Some(StorageType::Party),
                    Some(*pokemon),
                ))
            } else {
                pc_slot(Some(pokemon.nat_dex_number()))
                    .id(Id::new(pokemon.offset().to_string()))
                    .selected(selected)
                    .on_press(Message::Selected(
                        Some(Id::new(pokemon.offset().to_string())),
                        Some(StorageType::PC),
                        Some(*pokemon),
                    ))
            });
        }
        col = col.push(pc_row);
    }

    column![pc_label(pc_i + 1), col,]
        .align_x(iced::Alignment::Center)
        .spacing(15)
        .into()
}
