use iced::advanced::widget::Id;
use iced::widget::button;
use iced::widget::container;
use iced::widget::image;
use iced::widget::row;
use iced::widget::text;
use iced::Element;

use crate::icon;
use crate::menu_bar_default;
use crate::misc::PROJECT_DIR;
use crate::tab;
use crate::tab_bar_button_primary;
use crate::tab_bar_tab;
use crate::Message;

pub fn menu_bar<'a>(selected_tab: &Option<Id>) -> Element<'a, Message> {
    container(row![
        button(icon::open().center())
            .on_press(Message::OpenFile)
            .style(tab_bar_button_primary),
        button(icon::save().center())
            .on_press(Message::SaveFile)
            .style(tab_bar_button_primary),
        tab(row![
            image(image::Handle::from_bytes(
                PROJECT_DIR
                    .get_file("icons/pokebox_icon.png")
                    .unwrap()
                    .contents()
            ))
            .height(20.0),
            text("Party & Boxes")
        ]
        .spacing(5.0)
        .align_y(iced::alignment::Vertical::Center))
        .id(Id::new("1"))
        .style(tab_bar_tab)
        .selected(selected_tab)
        .on_press(Message::SelectedTab(Id::new("1"))),
        tab(row![
            image(image::Handle::from_bytes(
                PROJECT_DIR
                    .get_file("icons/bag_icon.png")
                    .unwrap()
                    .contents()
            ))
            .height(20.0),
            text("Bag & Trainer")
        ]
        .spacing(5.0)
        .align_y(iced::alignment::Vertical::Center))
        .id(Id::new("2"))
        .style(tab_bar_tab)
        .selected(selected_tab)
        .on_press(Message::SelectedTab(Id::new("2"))),
    ])
    .style(menu_bar_default)
    .into()
}
