//! The **Bag & Trainer** screen.
//!
//! Renders five scrollable bag pockets — Items, Pokéballs, Berries, TMs, and
//! Key Items — each row showing an item sprite, a pick-list for the item name,
//! and an [`crate::widgets::item_counter`] for the quantity.
//!
//! Owns its own [`Message`] enum and [`update`] function which validate
//! quantities (0–99) and write changes back into the [`pk_edit::SaveFile`].

use iced::advanced::widget::Id;
use iced::widget::container;
use iced::widget::image;
use iced::widget::{column, pick_list, row, scrollable, text};
use iced::{Alignment, Element, Length, Padding};
use pk_edit::error::SaveDataError;

use std::collections::HashMap;

use crate::message;
use crate::pick_list_default;
use crate::widgets::item_counter;
use crate::{shadow_box_light, tab_bar_tab};

use pk_edit::misc::{balls, berries, item_id, items, key_items, tms};
use pk_edit::save::storage::Pocket;
use pk_edit::SaveFile;

use crate::menu_bar;
use crate::menu_bar_default;
use crate::tab;

#[derive(Debug, Clone)]
pub enum Message {
    SelectedBag(Id),
    ItemChanged(Pocket, usize, String),
    ItemQuantityChanged(Pocket, usize, String, Operation),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Change,
    Increment,
    Decrement,
}

pub fn update(
    tm_bag: &mut [(String, u16)],
    key_bag: &mut [(String, u16)],
    item_bag: &mut [(String, u16)],
    ball_bag: &mut [(String, u16)],
    berry_bag: &mut [(String, u16)],
    save_file: &mut SaveFile,
    selected_bag: &mut Option<Id>,
    message: Message,
) -> Result<(), SaveDataError> {
    match message {
        Message::SelectedBag(id) => {
            selected_bag.replace(id);
            Ok(())
        }
        Message::ItemChanged(pocket, i, selected) => {
            let bag = match pocket {
                Pocket::Berries => berry_bag,
                Pocket::Items => item_bag,
                Pocket::Key => key_bag,
                Pocket::Pokeballs => ball_bag,
                Pocket::Tms => tm_bag,
            };

            let item = bag.get_mut(i).ok_or(SaveDataError::InvalidIndex(i))?;
            if "Nothing".eq(&selected) && pocket != Pocket::Key {
                item.1 = 0;
            } else if item.1 == 0 && pocket != Pocket::Key {
                item.1 = 1;
            }
            item.0 = selected;
            save_file.save_pocket(pocket, bag.to_owned())
        }
        Message::ItemQuantityChanged(pocket, i, mut quantity, operation) => {
            quantity.retain(|c| c.is_numeric());
            let bag = match pocket {
                Pocket::Berries => berry_bag,
                Pocket::Items => item_bag,
                Pocket::Key => key_bag,
                Pocket::Pokeballs => ball_bag,
                Pocket::Tms => tm_bag,
            };
            let item = bag.get_mut(i).ok_or(SaveDataError::InvalidIndex(i))?;
            if item.0 == "Nothing" {
                return Ok(());
            }

            if let Ok(mut number) = quantity.parse::<u16>() {
                match operation {
                    Operation::Increment => {
                        number += 1;
                    }
                    Operation::Decrement => {
                        number -= 1;
                    }
                    Operation::Change => {}
                }
                if number >= 99 {
                    item.1 = 99;
                } else if number == 0 {
                    item.1 = 0;
                    item.0 = "Nothing".to_string();
                } else {
                    item.1 = number;
                }
                return save_file.save_pocket(pocket, bag.to_owned());
            }
            Ok(())
        }
    }
}

pub fn bag<'a>(
    selected_bag: &Option<Id>,
    selected_tab: &Option<Id>,
    items: &'a [(String, u16)],
    balls: &'a [(String, u16)],
    berries: &'a [(String, u16)],
    tms: &'a [(String, u16)],
    key_items: &'a [(String, u16)],
    images: &HashMap<String, image::Handle>,
) -> Element<'a, message::Message> {
    column![
        menu_bar::view(selected_tab, images).map(message::Message::MenuBar),
        row![
            iced::widget::Space::new().width(Length::Fill),
            bag_tab_bar(selected_bag, images).map(message::Message::Bag),
            iced::widget::Space::new().width(Length::Fill)
        ],
        row![
            iced::widget::Space::new().width(Length::Fill),
            if Some(Id::new("1")) == *selected_bag {
                items_bag(items, images).map(message::Message::Bag)
            } else if Some(Id::new("2")) == *selected_bag {
                balls_bag(balls, images).map(message::Message::Bag)
            } else if Some(Id::new("3")) == *selected_bag {
                berries_bag(berries, images).map(message::Message::Bag)
            } else if Some(Id::new("4")) == *selected_bag {
                tms_bag(tms, images).map(message::Message::Bag)
            } else if Some(Id::new("5")) == *selected_bag {
                keys_bag(key_items, images).map(message::Message::Bag)
            } else {
                text("").into()
            },
            iced::widget::Space::new().width(Length::Fill),
        ]
        .spacing(15),
    ]
    .spacing(15)
    .into()
}

fn bag_tab_bar<'a>(
    selected: &Option<Id>,
    images: &HashMap<String, image::Handle>,
) -> Element<'a, Message> {
    let battle = row![
        image(images.get("battle_items_icon").unwrap_or({
            let width = 10;
            let height = 10;
            let size = (width * height) as usize;
            let pixels = vec![0u8; size * 4];
            &image::Handle::from_rgba(width, height, pixels)
        })),
        text("Items")
    ]
    .spacing(10.0)
    .align_y(Alignment::Center);

    let poke_ball = row![
        image(images.get("poke_balls_icon").unwrap_or({
            let width = 10;
            let height = 10;
            let size = (width * height) as usize;
            let pixels = vec![0u8; size * 4];
            &image::Handle::from_rgba(width, height, pixels)
        })),
        text("Pokeballs")
    ]
    .spacing(10.0)
    .align_y(iced::alignment::Vertical::Center);

    let berry = row![
        image(images.get("berries_icon").unwrap_or({
            let width = 10;
            let height = 10;
            let size = (width * height) as usize;
            let pixels = vec![0u8; size * 4];
            &image::Handle::from_rgba(width, height, pixels)
        })),
        text("Berries")
    ]
    .spacing(10.0)
    .align_y(iced::alignment::Vertical::Center);

    let tm = row![
        image(images.get("tms_icon").unwrap_or({
            let width = 10;
            let height = 10;
            let size = (width * height) as usize;
            let pixels = vec![0u8; size * 4];
            &image::Handle::from_rgba(width, height, pixels)
        })),
        text("TMs")
    ]
    .spacing(10.0)
    .align_y(iced::alignment::Vertical::Center);

    let key = row![
        image(images.get("key_items_icon").unwrap_or({
            let width = 10;
            let height = 10;
            let size = (width * height) as usize;
            let pixels = vec![0u8; size * 4];
            &image::Handle::from_rgba(width, height, pixels)
        })),
        text("Key Items")
    ]
    .spacing(10.0)
    .align_y(iced::alignment::Vertical::Center);

    container(row![
        tab(battle)
            .id(Id::new("1"))
            .style(tab_bar_tab)
            .selected(selected)
            .on_press(Message::SelectedBag(Id::new("1"))),
        tab(poke_ball)
            .id(Id::new("2"))
            .style(tab_bar_tab)
            .selected(selected)
            .on_press(Message::SelectedBag(Id::new("2"))),
        tab(berry)
            .id(Id::new("3"))
            .style(tab_bar_tab)
            .selected(selected)
            .on_press(Message::SelectedBag(Id::new("3"))),
        tab(tm)
            .id(Id::new("4"))
            .style(tab_bar_tab)
            .selected(selected)
            .on_press(Message::SelectedBag(Id::new("4"))),
        tab(key)
            .id(Id::new("5"))
            .style(tab_bar_tab)
            .selected(selected)
            .on_press(Message::SelectedBag(Id::new("5"))),
    ])
    .height(40.0)
    .style(menu_bar_default)
    .into()
}

fn items_bag<'a>(
    bag: &'a [(String, u16)],
    images: &HashMap<String, image::Handle>,
) -> Element<'a, Message> {
    let mut column = column![].spacing(10);

    let items = match items() {
        Ok(is) => is,
        Err(_) => {
            vec![String::from("")]
        }
    };

    for (i, (item, quantity)) in bag.iter().enumerate() {
        let item_id = item_id(item).unwrap_or_default();
        let item_image =
            if let Some(handle) = images.get(&format!("item_{:0width$}", item_id, width = 4)) {
                image(handle).height(40)
            } else {
                image(images.get("other_items_icon").unwrap_or({
                    let width = 10;
                    let height = 10;
                    let size = (width * height) as usize;
                    let pixels = vec![0u8; size * 4];
                    &image::Handle::from_rgba(width, height, pixels)
                }))
                .width(40)
            };

        column = column.push(
            container(
                row![
                    item_image,
                    pick_list(items.clone(), Some(item), move |selected| {
                        Message::ItemChanged(Pocket::Items, i, selected)
                    })
                    .width(150)
                    .style(pick_list_default),
                    item_counter(Pocket::Items, i, quantity),
                ]
                .padding(Padding::from([0, 10]))
                .align_y(Alignment::Center)
                .spacing(25),
            )
            .style(shadow_box_light)
            .width(350)
            .height(40.0),
        )
    }
    scrollable(column).into()
}

fn balls_bag<'a>(
    bag: &'a [(String, u16)],
    images: &HashMap<String, image::Handle>,
) -> Element<'a, Message> {
    let mut column = column![].spacing(10);

    let balls = match balls() {
        Ok(ms) => ms,
        Err(_) => {
            vec![String::from("")]
        }
    };

    for (i, (item, quantity)) in bag.iter().enumerate() {
        let item_id = item_id(item).unwrap_or_default();

        let item_image =
            if let Some(handle) = images.get(&format!("item_{:0width$}", item_id, width = 4)) {
                image(handle).height(40)
            } else {
                image(images.get("poke_balls_icon").unwrap_or({
                    let width = 10;
                    let height = 10;
                    let size = (width * height) as usize;
                    let pixels = vec![0u8; size * 4];
                    &image::Handle::from_rgba(width, height, pixels)
                }))
                .width(40)
            };

        column = column.push(
            container(
                row![
                    item_image,
                    pick_list(balls.clone(), Some(item), move |selected| {
                        Message::ItemChanged(Pocket::Pokeballs, i, selected)
                    })
                    .width(150)
                    .style(pick_list_default),
                    item_counter(Pocket::Pokeballs, i, quantity),
                ]
                .padding(Padding::from([0, 10]))
                .align_y(Alignment::Center)
                .spacing(25),
            )
            .style(shadow_box_light)
            .width(350)
            .height(40.0),
        )
    }
    scrollable(column).into()
}

fn berries_bag<'a>(
    bag: &'a [(String, u16)],
    images: &HashMap<String, image::Handle>,
) -> Element<'a, Message> {
    let mut column = column![].spacing(10);

    let berries = match berries() {
        Ok(ms) => ms,
        Err(_) => {
            vec![String::from("")]
        }
    };

    for (i, (item, quantity)) in bag.iter().enumerate() {
        let item_id = item_id(item).unwrap_or_default();

        let item_image =
            if let Some(handle) = images.get(&format!("item_{:0width$}", item_id, width = 4)) {
                image(handle).height(40)
            } else {
                image(images.get("berries_icon").unwrap_or({
                    let width = 10;
                    let height = 10;
                    let size = (width * height) as usize;
                    let pixels = vec![0u8; size * 4];
                    &image::Handle::from_rgba(width, height, pixels)
                }))
                .width(40)
            };

        column = column.push(
            container(
                row![
                    item_image,
                    pick_list(berries.clone(), Some(item), move |selected| {
                        Message::ItemChanged(Pocket::Berries, i, selected)
                    })
                    .width(150)
                    .style(pick_list_default),
                    item_counter(Pocket::Berries, i, quantity),
                ]
                .padding(Padding::from([0, 10]))
                .align_y(Alignment::Center)
                .spacing(25),
            )
            .style(shadow_box_light)
            .width(350)
            .height(40.0),
        )
    }
    scrollable(column).into()
}

fn tms_bag<'a>(
    bag: &'a [(String, u16)],
    images: &HashMap<String, image::Handle>,
) -> Element<'a, Message> {
    let mut column = column![].spacing(10);

    let tms = match tms() {
        Ok(ms) => ms,
        Err(_) => {
            vec![String::from("")]
        }
    };

    for (i, (item, quantity)) in bag.iter().enumerate() {
        let item_id = item_id(item).unwrap_or_default();

        let item_image =
            if let Some(handle) = images.get(&format!("item_{:0width$}", item_id, width = 4)) {
                image(handle).height(40)
            } else {
                image(images.get("tms_icon").unwrap_or({
                    let width = 10;
                    let height = 10;
                    let size = (width * height) as usize;
                    let pixels = vec![0u8; size * 4];
                    &image::Handle::from_rgba(width, height, pixels)
                }))
                .width(40)
            };

        column = column.push(
            container(
                row![
                    item_image,
                    pick_list(tms.clone(), Some(item), move |selected| {
                        Message::ItemChanged(Pocket::Tms, i, selected)
                    })
                    .width(150)
                    .style(pick_list_default),
                    item_counter(Pocket::Tms, i, quantity),
                ]
                .padding(Padding::from([0, 10]))
                .align_y(Alignment::Center)
                .spacing(25),
            )
            .style(shadow_box_light)
            .width(350)
            .height(40.0),
        )
    }
    scrollable(column).into()
}

fn keys_bag<'a>(
    bag: &'a [(String, u16)],
    images: &HashMap<String, image::Handle>,
) -> Element<'a, Message> {
    let mut column = column![].spacing(10);

    let key_items = match key_items() {
        Ok(ms) => ms,
        Err(_) => {
            vec![String::from("")]
        }
    };

    for (i, (item, _)) in bag.iter().enumerate() {
        let item_id = item_id(item).unwrap_or_default();

        let item_image =
            if let Some(handle) = images.get(&format!("item_{:0width$}", item_id, width = 4)) {
                image(handle).height(40)
            } else {
                image(images.get("key_items_icon").unwrap_or({
                    let width = 10;
                    let height = 10;
                    let size = (width * height) as usize;
                    let pixels = vec![0u8; size * 4];
                    &image::Handle::from_rgba(width, height, pixels)
                }))
                .width(40)
            };

        column = column.push(
            container(
                row![
                    item_image,
                    pick_list(key_items.clone(), Some(item), move |selected| {
                        Message::ItemChanged(Pocket::Key, i, selected)
                    })
                    .width(150)
                    .style(pick_list_default),
                ]
                .padding(Padding::from([0, 10]))
                .align_y(Alignment::Center)
                .spacing(25),
            )
            .style(shadow_box_light)
            .width(350)
            .height(40.0),
        )
    }
    scrollable(column).into()
}
