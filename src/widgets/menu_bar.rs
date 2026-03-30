//! The application menu bar.
//!
//! Renders a horizontal bar containing:
//! - An **Open** button (folder icon) that triggers the file-open dialog.
//! - A **Save** button (floppy disk icon) that triggers the file-save dialog.
//! - Tab buttons for switching between the **Party & Boxes** and **Bag & Trainer** screens.

use iced::advanced::widget::Id;
use iced::widget::button;
use iced::widget::container;
use iced::widget::image;
use iced::widget::row;
use iced::widget::text;
use iced::Element;

use std::collections::HashMap;

use crate::icon;
use crate::menu_bar_default;
use crate::tab;
use crate::tab_bar_button_primary;
use crate::tab_bar_tab;

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile,
    SaveFile,
    SelectedTab(Id),
}

pub fn view<'a>(
    selected_tab: &Option<Id>,
    images: &HashMap<String, image::Handle>,
) -> Element<'a, Message> {
    container(row![
        button(icon::open().center())
            .on_press(Message::OpenFile)
            .style(tab_bar_button_primary),
        button(icon::save().center())
            .on_press(Message::SaveFile)
            .style(tab_bar_button_primary),
        tab(row![
            image(images.get("pokebox_icon").unwrap_or({
                let width = 10;
                let height = 10;
                let size = (width * height) as usize;
                let pixels = vec![0u8; size * 4];
                &image::Handle::from_rgba(width, height, pixels)
            }))
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
            image(images.get("bag_icon").unwrap_or({
                let width = 0;
                let height = 0;
                let size = (width * height) as usize;
                let pixels = vec![0u8; size * 4];
                &image::Handle::from_rgba(width, height, pixels)
            }))
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
