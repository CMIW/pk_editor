use iced::widget::Container;
use iced::widget::{column, container, image, row, text};
use iced::Padding;

use crate::message::Message;
use crate::misc::PROJECT_DIR;
use crate::slots::party_slot;
use crate::theme::party_label_appearance;

use pk_edit::data_structure::pokemon::Pokemon;

fn party_label() -> Container<'static, Message> {
    let handle = image::Handle::from_memory(
        PROJECT_DIR
            .get_file("Pokeball_icon_white.png")
            .unwrap()
            .contents(),
    );

    let image = container(image(handle))
        .width(20.0)
        .height(20.0)
        .align_x(iced::alignment::Horizontal::Right)
        .align_y(iced::alignment::Vertical::Center);

    let row = row![image, text("Current Party")]
        .spacing(50)
        .width(240.0)
        .height(40.0)
        .padding(Padding::from([9, 10, 9, 20])); // top, right, bottom, left

    container(row)
        .width(240.0)
        .height(40.0)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .style(party_label_appearance())
}

pub fn party(id: &usize, party: &Vec<Pokemon>) -> Container<'static, Message> {
    let label = party_label();
    let mut col = iced::widget::Column::new().spacing(5);

    for pokemon in party {
        col = col.push(party_slot(id, pokemon));
    }

    container(column![label, col].spacing(10))
}
