use iced::color;
use iced::widget::{button, column, container, image, mouse_area, row, text};
use iced::widget::{Container, MouseArea};
use iced::{Alignment, Border, Color, Element, Length, Padding, Shadow};

use crate::message::Message;
use crate::misc::{PROJECT_DIR, WINDOW_WIDTH};
use crate::slots::pc_slot;
use crate::theme::slot_appearance;
use crate::widgets::{gender, level};

use pk_edit::data_structure::pokemon::Pokemon;
use pk_edit::StorageType;

fn pc_box_label(box_number: usize) -> Element<'static, Message> {
    container(text(format!("Box {}", box_number)))
        .width(335)
        .height(40.0)
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Center)
        .style(slot_appearance())
        .into()
}

pub fn pc_box(id: &usize, num: &usize, pc_box: &Vec<Pokemon>) -> Container<'static, Message> {
    let label = pc_box_label(num + 1);
    let mut col = iced::widget::Column::new()
        .align_items(Alignment::Center)
        .spacing(5);

    for row in pc_box.chunks(6) {
        let mut pc_row = iced::widget::Row::new().spacing(5);
        for slot in row {
            pc_row = pc_row.push(pc_slot(id, slot));
        }
        col = col.push(pc_row);
    }

    let left_handle =
        image::Handle::from_memory(PROJECT_DIR.get_file("left-arrow.png").unwrap().contents());

    let right_handle =
        image::Handle::from_memory(PROJECT_DIR.get_file("right-arrow.png").unwrap().contents());

    container(
        column![
            row![
                button(image(left_handle))
                    .on_press(Message::Decrement)
                    .width(40)
                    .height(40),
                label,
                button(image(right_handle))
                    .on_press(Message::Increment)
                    .width(40)
                    .height(40)
            ]
            .spacing(5),
            col
        ]
        .align_items(Alignment::Center)
        .spacing(10),
    )
}
