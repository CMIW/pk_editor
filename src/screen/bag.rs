use iced::advanced::widget::Id;
use iced::widget::container;
use iced::widget::image;
use iced::widget::{column, pick_list, row, scrollable, text, text_input};
use iced::Length;
use iced::{Alignment, Element};

use crate::message;
use crate::misc::PROJECT_DIR;
use crate::pick_list_default;
use crate::tab_bar_tab;

use pk_edit::misc::{balls, berries, item_id, items, key_items, tms};
use pk_edit::Pocket;
use pk_edit::SaveFile;

use crate::menu_bar;
use crate::menu_bar_default;
use crate::tab;

#[derive(Debug, Clone)]
pub enum Message {
    SelectedBag(Id),
    ItemChanged(usize, String),
    ItemQuantityChanged(usize, String),
    BallChanged(usize, String),
    BallQuantityChanged(usize, String),
    BerryChanged(usize, String),
    BerryQuantityChanged(usize, String),
    TmChanged(usize, String),
    TmQuantityChanged(usize, String),
    KeyChanged(usize, String),
}

pub fn update(
    tm_bag: &mut Vec<(String, u16)>,
    key_bag: &mut Vec<(String, u16)>,
    item_bag: &mut Vec<(String, u16)>,
    ball_bag: &mut Vec<(String, u16)>,
    berry_bag: &mut Vec<(String, u16)>,
    save_file: &mut SaveFile,
    selected_bag: &mut Option<Id>,
    message: Message,
) {
    match message {
        Message::SelectedBag(id) => {
            selected_bag.replace(id);
        }
        Message::ItemChanged(i, selected) => {
            item_bag[i].0 = selected;
            if item_bag[i].1 == 0 {
                item_bag[i].1 = 1;
            }
            save_file.save_pocket(Pocket::Items, item_bag.clone());
        }
        Message::ItemQuantityChanged(i, mut value) => {
            value.retain(|c| c.is_numeric());
            if let Ok(number) = value.parse::<u16>() {
                if number >= 99 {
                    item_bag[i].1 = 99;
                } else if number == 0 {
                    item_bag[i].1 = 0;
                    item_bag[i].0 = "Nothing".to_string();
                } else {
                    item_bag[i].1 = number;
                }
                save_file.save_pocket(Pocket::Items, item_bag.clone());
            }
        }
        Message::BallChanged(i, selected) => {
            ball_bag[i].0 = selected;
            if ball_bag[i].0 == "Nothing" {
                ball_bag[i].1 = 0;
            } else if ball_bag[i].1 == 0 {
                ball_bag[i].1 = 1;
            }
            save_file.save_pocket(Pocket::Pokeballs, ball_bag.clone());
        }
        Message::BallQuantityChanged(i, mut value) => {
            value.retain(|c| c.is_numeric());
            if let Ok(number) = value.parse::<u16>() {
                if number >= 99 {
                    ball_bag[i].1 = 99;
                } else if number == 0 {
                    ball_bag[i].1 = 0;
                    ball_bag[i].0 = "Nothing".to_string();
                } else {
                    ball_bag[i].1 = number;
                }
                save_file.save_pocket(Pocket::Pokeballs, ball_bag.clone());
            }
        }
        Message::BerryChanged(i, selected) => {
            berry_bag[i].0 = selected;
            if berry_bag[i].0 == "Nothing" {
                berry_bag[i].1 = 0;
            } else if berry_bag[i].1 == 0 {
                berry_bag[i].1 = 1;
            }
            save_file.save_pocket(Pocket::Berries, berry_bag.clone());
        }
        Message::BerryQuantityChanged(i, mut value) => {
            value.retain(|c| c.is_numeric());
            if let Ok(number) = value.parse::<u16>() {
                if number >= 99 {
                    berry_bag[i].1 = 99;
                } else if number == 0 {
                    berry_bag[i].1 = 0;
                    berry_bag[i].0 = "Nothing".to_string();
                } else {
                    berry_bag[i].1 = number;
                }
                save_file.save_pocket(Pocket::Berries, berry_bag.clone());
            }
        }
        Message::TmChanged(i, selected) => {
            tm_bag[i].0 = selected;
            if tm_bag[i].0 == "Nothing" {
                tm_bag[i].1 = 0;
            } else if tm_bag[i].1 == 0 {
                tm_bag[i].1 = 1;
            }
            save_file.save_pocket(Pocket::Tms, tm_bag.clone());
        }
        Message::TmQuantityChanged(i, mut value) => {
            value.retain(|c| c.is_numeric());
            if let Ok(number) = value.parse::<u16>() {
                if number >= 99 {
                    tm_bag[i].1 = 99;
                } else if number == 0 {
                    tm_bag[i].1 = 0;
                    tm_bag[i].0 = "Nothing".to_string();
                } else {
                    tm_bag[i].1 = number;
                }
                save_file.save_pocket(Pocket::Tms, tm_bag.clone());
            }
        }
        Message::KeyChanged(i, selected) => {
            key_bag[i].0 = selected;
            if key_bag[i].0 == "Nothing" {
                key_bag[i].1 = 0;
            } else if key_bag[i].1 == 0 {
                key_bag[i].1 = 1;
            }
            save_file.save_pocket(Pocket::Key, key_bag.clone());
        }
    };
}

pub fn bag<'a>(
    selected_bag: &Option<Id>,
    selected_tab: &Option<Id>,
    items: &'a [(String, u16)],
    balls: &'a [(String, u16)],
    berries: &'a [(String, u16)],
    tms: &'a [(String, u16)],
    key_items: &'a [(String, u16)],
) -> Element<'a, message::Message> {
    column![
        menu_bar::view(selected_tab).map(|m| message::Message::MenuBar(m)),
        row![
            iced::widget::Space::with_width(Length::Fill),
            bag_tab_bar(selected_bag).map(|m| message::Message::Bag(m)),
            iced::widget::Space::with_width(Length::Fill)
        ],
        row![
            iced::widget::Space::with_width(Length::Fill),
            if Some(Id::new("1")) == *selected_bag {
                items_bag(items).map(|m| message::Message::Bag(m))
            } else if Some(Id::new("2")) == *selected_bag {
                balls_bag(balls).map(|m| message::Message::Bag(m))
            } else if Some(Id::new("3")) == *selected_bag {
                berries_bag(berries).map(|m| message::Message::Bag(m))
            } else if Some(Id::new("4")) == *selected_bag {
                tms_bag(tms).map(|m| message::Message::Bag(m))
            } else if Some(Id::new("5")) == *selected_bag {
                keys_bag(key_items).map(|m| message::Message::Bag(m))
            } else {
                text("").into()
            },
            iced::widget::Space::with_width(Length::Fill),
        ]
        .spacing(15),
    ]
    .spacing(15)
    .into()
}

fn bag_tab_bar<'a>(selected: &Option<Id>) -> Element<'a, Message> {
    let handle = image::Handle::from_bytes(
        PROJECT_DIR
            .get_file("icons/battle_items_icon.png")
            .unwrap()
            .contents(),
    );
    let battle = row![image(handle), text("Items")]
        .spacing(10.0)
        .align_y(Alignment::Center);

    let handle = image::Handle::from_bytes(
        PROJECT_DIR
            .get_file("icons/poke_balls_icon.png")
            .unwrap()
            .contents(),
    );
    let poke_ball = row![image(handle), text("Pokeballs")]
        .spacing(10.0)
        .align_y(iced::alignment::Vertical::Center);

    let handle = image::Handle::from_bytes(
        PROJECT_DIR
            .get_file("icons/berries_icon.png")
            .unwrap()
            .contents(),
    );
    let berry = row![image(handle), text("Berries")]
        .spacing(10.0)
        .align_y(iced::alignment::Vertical::Center);

    let handle = image::Handle::from_bytes(
        PROJECT_DIR
            .get_file("icons/tms_icon.png")
            .unwrap()
            .contents(),
    );
    let tm = row![image(handle), text("TMs")]
        .spacing(10.0)
        .align_y(iced::alignment::Vertical::Center);

    let handle = image::Handle::from_bytes(
        PROJECT_DIR
            .get_file("icons/key_items_icon.png")
            .unwrap()
            .contents(),
    );
    let key = row![image(handle), text("Key Items")]
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

fn items_bag<'a>(bag: &'a [(String, u16)]) -> Element<'a, Message> {
    let mut column = column![];

    let items = match items() {
        Ok(is) => is,
        Err(_) => {
            vec![String::from("")]
        }
    };

    for (i, (item, quantity)) in bag.into_iter().enumerate() {
        let item_id = match item_id(&item) {
            Ok(id) => id,
            Err(_) => 0,
        };

        let item_image = if let Some(item_image) =
            PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_id, width = 4))
        {
            let handle = image::Handle::from_bytes(item_image.contents());
            image(handle).height(40)
        } else {
            image("").width(40)
        };

        column = column.push(
            row![
                item_image,
                pick_list(items.clone(), Some(item), move |selected| {
                    Message::ItemChanged(i, selected)
                })
                .width(150)
                .style(pick_list_default),
                text_input(&quantity.to_string(), &quantity.to_string())
                    .on_input(move |input| Message::ItemQuantityChanged(i, input))
                    .line_height(text::LineHeight::Absolute(10.into()))
                    .width(30)
                    .size(12),
            ]
            .align_y(Alignment::Center)
            .spacing(25)
            .width(320)
            .height(40.0),
        )
    }
    scrollable(column).into()
}

fn balls_bag<'a>(bag: &'a [(String, u16)]) -> Element<'a, Message> {
    let mut column = column![];

    let balls = match balls() {
        Ok(ms) => ms,
        Err(_) => {
            vec![String::from("")]
        }
    };

    for (i, (item, quantity)) in bag.into_iter().enumerate() {
        let item_id = match item_id(&item) {
            Ok(id) => id,
            Err(_) => 0,
        };

        let item_image = if let Some(item_image) =
            PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_id, width = 4))
        {
            let handle = image::Handle::from_bytes(item_image.contents());
            image(handle).height(40)
        } else {
            image("").width(40)
        };

        column = column.push(
            row![
                item_image,
                pick_list(balls.clone(), Some(item), move |selected| {
                    Message::BallChanged(i, selected)
                })
                .width(150)
                .style(pick_list_default),
                text_input(&quantity.to_string(), &quantity.to_string())
                    .on_input(move |input| Message::BallQuantityChanged(i, input))
                    .line_height(text::LineHeight::Absolute(10.into()))
                    .width(30)
                    .size(12),
            ]
            .align_y(Alignment::Center)
            .spacing(25)
            .width(320)
            .height(40.0),
        )
    }
    scrollable(column).into()
}

fn berries_bag<'a>(bag: &'a [(String, u16)]) -> Element<'a, Message> {
    let mut column = column![];

    let berries = match berries() {
        Ok(ms) => ms,
        Err(_) => {
            vec![String::from("")]
        }
    };

    for (i, (item, quantity)) in bag.into_iter().enumerate() {
        let item_id = match item_id(&item) {
            Ok(id) => id,
            Err(_) => 0,
        };

        let item_image = if let Some(item_image) =
            PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_id, width = 4))
        {
            let handle = image::Handle::from_bytes(item_image.contents());
            image(handle).height(40)
        } else {
            image("").width(40)
        };

        column = column.push(
            row![
                item_image,
                pick_list(berries.clone(), Some(item), move |selected| {
                    Message::BerryChanged(i, selected)
                })
                .width(150)
                .style(pick_list_default),
                text_input(&quantity.to_string(), &quantity.to_string())
                    .on_input(move |input| Message::BerryQuantityChanged(i, input))
                    .line_height(text::LineHeight::Absolute(10.into()))
                    .width(30)
                    .size(12),
            ]
            .align_y(Alignment::Center)
            .spacing(25)
            .width(320)
            .height(40.0),
        )
    }
    scrollable(column).into()
}

fn tms_bag<'a>(bag: &'a [(String, u16)]) -> Element<'a, Message> {
    let mut column = column![];

    let tms = match tms() {
        Ok(ms) => ms,
        Err(_) => {
            vec![String::from("")]
        }
    };

    for (i, (item, quantity)) in bag.into_iter().enumerate() {
        let item_id = match item_id(&item) {
            Ok(id) => id,
            Err(_) => 0,
        };

        let item_image = if let Some(item_image) =
            PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_id, width = 4))
        {
            let handle = image::Handle::from_bytes(item_image.contents());
            image(handle).height(40)
        } else {
            image("").width(40)
        };

        column = column.push(
            row![
                item_image,
                pick_list(tms.clone(), Some(item), move |selected| {
                    Message::TmChanged(i, selected)
                })
                .width(150)
                .style(pick_list_default),
                text_input(&quantity.to_string(), &quantity.to_string())
                    .on_input(move |input| Message::TmQuantityChanged(i, input))
                    .line_height(text::LineHeight::Absolute(10.into()))
                    .width(30)
                    .size(12),
            ]
            .align_y(Alignment::Center)
            .spacing(25)
            .width(320)
            .height(40.0),
        )
    }
    scrollable(column).into()
}

fn keys_bag<'a>(bag: &'a [(String, u16)]) -> Element<'a, Message> {
    let mut column = column![];

    let key_items = match key_items() {
        Ok(ms) => ms,
        Err(_) => {
            vec![String::from("")]
        }
    };

    for (i, (item, _)) in bag.into_iter().enumerate() {
        let item_id = match item_id(&item) {
            Ok(id) => id,
            Err(_) => 0,
        };

        let item_image = if let Some(item_image) =
            PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_id, width = 4))
        {
            let handle = image::Handle::from_bytes(item_image.contents());
            image(handle).height(40)
        } else {
            let handle = image::Handle::from_bytes(
                PROJECT_DIR
                    .get_file("icons/other_items_icon.png")
                    .unwrap()
                    .contents(),
            );
            image(handle).width(40)
        };

        column = column.push(
            row![
                item_image,
                pick_list(key_items.clone(), Some(item), move |selected| {
                    Message::KeyChanged(i, selected)
                })
                .width(150)
                .style(pick_list_default),
            ]
            .align_y(Alignment::Center)
            .spacing(25)
            .width(320)
            .height(40.0),
        )
    }
    scrollable(column).into()
}
