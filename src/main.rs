use iced::application::Application;
use iced::executor;
use iced::font;
use iced::widget::image;
use iced::widget::Column;
use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input};
use iced::Settings;
use iced::{Alignment, Command, Element, Size, Theme};

use iced::widget::scrollable::Direction;
use iced::widget::scrollable::Properties;
use iced::widget::Scrollable;

use std::path::PathBuf;
use std::sync::Arc;

use pk_editor::error::Error;
use pk_editor::message::Message;
use pk_editor::misc::{PROJECT_DIR, WINDOW_HEIGHT, WINDOW_WIDTH};
use pk_editor::party::party;
use pk_editor::pc::pc_box;
use pk_editor::pokemon_info::pokemon_info;

use pk_edit::data_structure::pokemon::{gen_pokemon_from_species, Pokemon, Pokerus};
use pk_edit::misc::{balls_list, berries_list, items_list, key_list, tm_list, transpose_item};
use pk_edit::{SaveFile, StorageType};

fn main() -> iced::Result {
    State::run(Settings {
        window: iced::window::Settings {
            size: Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: false,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default, Debug)]
pub struct State {
    error: Option<Error>,
    save_file: SaveFile,
    party: Vec<Pokemon>,
    current_pc: Vec<Pokemon>,
    current_pc_index: usize,
    selected_pokemon: Pokemon,
    selected_pokemon_storage: StorageType,
    item_bag: Vec<(String, u16)>,
    ball_bag: Vec<(String, u16)>,
    berry_bag: Vec<(String, u16)>,
    tm_bag: Vec<(String, u16)>,
    key_bag: Vec<(String, u16)>,
    pokemon_view: bool,
    bag_view: bool,
}

impl Application for State {
    type Flags = ();
    type Theme = Theme;
    type Message = Message;
    type Executor = executor::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            State {
                error: None,
                save_file: SaveFile::default(),
                party: vec![Pokemon::default(); 6],
                current_pc: vec![Pokemon::default(); 30],
                current_pc_index: 0,
                selected_pokemon: Pokemon::default(),
                selected_pokemon_storage: StorageType::None,
                item_bag: vec![],
                ball_bag: vec![],
                berry_bag: vec![],
                tm_bag: vec![],
                key_bag: vec![],
                pokemon_view: true,
                bag_view: false,
            },
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("PK_Edit")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OpenFile => Command::perform(pick_file(), Message::FileOpened),
            Message::SaveFile => Command::perform(pick_file(), Message::FileSaved),
            Message::FileOpened(Ok(path)) => Command::perform(load_file(path), Message::LoadFile),
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::FileSaved(Ok(path)) => {
                if !self.save_file.is_empty() {
                    Command::perform(
                        write_file(path, Some(Arc::new(self.save_file.raw_data()))),
                        Message::WriteFile,
                    )
                } else {
                    Command::none()
                }
            }
            Message::FileSaved(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::LoadFile(Ok(results)) => {
                self.save_file = SaveFile::new(&results);
                self.current_pc_index = 0;
                self.selected_pokemon = Pokemon::default();
                self.selected_pokemon_storage = StorageType::None;
                self.item_bag = vec![];
                self.ball_bag = vec![];
                self.berry_bag = vec![];
                self.tm_bag = vec![];
                self.key_bag = vec![];

                self.update(Message::UpdateChanges)
            }
            Message::LoadFile(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::WriteFile(Ok(_)) => Command::none(),
            Message::WriteFile(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::UpdateChanges => {
                if !&self.selected_pokemon.is_empty() {
                    let _ = &self.selected_pokemon.update_checksum();
                    let _ = &self
                        .save_file
                        .save_pokemon(self.selected_pokemon_storage, self.selected_pokemon);
                }

                self.party = self.save_file.get_party();
                self.current_pc = self.save_file.pc_box(self.current_pc_index);
                self.item_bag = self.save_file.item_pocket();
                self.ball_bag = self.save_file.ball_pocket();
                self.berry_bag = self.save_file.berry_pocket();
                self.tm_bag = self.save_file.tm_pocket();
                self.key_bag = self.save_file.key_pocket();

                Command::none()
            }
            Message::SelectedPokemon((storage, pokemon)) => {
                self.selected_pokemon_storage = storage;
                self.selected_pokemon = pokemon;
                Command::none()
            }
            Message::Increment => {
                if !self.save_file.is_pc_empty() {
                    if self.current_pc_index < 13 {
                        self.current_pc_index += 1;
                    } else {
                        self.current_pc_index = 0;
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::Decrement => {
                if !self.save_file.is_pc_empty() {
                    if self.current_pc_index > 0 {
                        self.current_pc_index -= 1;
                    } else {
                        self.current_pc_index = 13;
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::ChangePokerusStatus => {
                match &self.selected_pokemon.pokerus_status() {
                    Pokerus::Infected => {
                        let _ = &self.selected_pokemon.cure_pokerus();
                    }
                    Pokerus::Cured => {
                        let _ = &self.selected_pokemon.remove_pokerus();
                    }
                    Pokerus::None => {
                        let _ = &self.selected_pokemon.infect_pokerus();
                    }
                }

                self.update(Message::UpdateChanges)
            }
            Message::HeldItemSelected(item) => {
                if !&self.selected_pokemon.is_empty() {
                    self.selected_pokemon.give_item(&item);
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::SpeciesSelected(species) => {
                if self.selected_pokemon.is_empty() && !self.save_file.is_empty() {
                    self.selected_pokemon = gen_pokemon_from_species(
                        &mut self.selected_pokemon,
                        &species,
                        &self.save_file.ot_name(),
                        &self.save_file.ot_id(),
                    );

                    self.update(Message::UpdateChanges)
                } else {
                    self.selected_pokemon.set_species(&species);
                    self.update(Message::UpdateChanges)
                }
            }
            Message::MoveSelected(index, value) => {
                self.selected_pokemon.set_move(index, &value);
                self.update(Message::UpdateChanges)
            }
            Message::AddMove(index) => {
                if !self.selected_pokemon.is_empty() {
                    self.selected_pokemon.set_move(index, "Pound");
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::FriendshipChanged(mut value) => {
                value.retain(|c| c.is_numeric());
                if let Ok(number) = value.parse::<u64>() {
                    let value = if number > u8::MAX.into() {
                        u8::MAX
                    } else {
                        number as u8
                    };
                    self.selected_pokemon.set_friendship(value);
                }
                self.update(Message::UpdateChanges)
            }
            Message::LevelInputChanged(mut value) => {
                if !self.selected_pokemon.is_empty() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u64>() {
                        let lowest_level = self.selected_pokemon.lowest_level();
                        let value = if number > 100 {
                            100
                        } else if number < lowest_level as u64 {
                            lowest_level
                        } else {
                            number as u8
                        };
                        self.selected_pokemon.set_level(value);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::NatureSelected(nature) => {
                //self.selected_pokemon.set_nature(&nature);
                self.update(Message::UpdateChanges)
            }
            Message::IVChanged(iv, mut value) => {
                if !self.selected_pokemon.is_empty() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u16>() {
                        self.selected_pokemon.stats_mut().update_ivs(&iv, number);
                    } else if value.is_empty() {
                        self.selected_pokemon.stats_mut().update_ivs(&iv, 0);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::EVChanged(ev, mut value) => {
                if !self.selected_pokemon.is_empty() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u16>() {
                        self.selected_pokemon.stats_mut().update_evs(&ev, number);
                    } else if value.is_empty() {
                        self.selected_pokemon.stats_mut().update_evs(&ev, 0);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::PokemonView => {
                self.pokemon_view = true;
                self.bag_view = false;
                Command::none()
            }
            Message::BagView => {
                self.pokemon_view = false;
                self.bag_view = true;
                Command::none()
            }
            Message::ItemChanged(i, selected) => {
                self.item_bag[i].0 = selected;
                if self.item_bag[i].1 == 0 {
                    self.item_bag[i].1 = 1;
                }
                self.save_file.save_item_pocket(self.item_bag.clone());
                self.update(Message::UpdateChanges)
            }
            Message::ItemQuantityChanged(i, mut value) => {
                value.retain(|c| c.is_numeric());
                if let Ok(number) = value.parse::<u16>() {
                    if number >= 99 {
                        self.item_bag[i].1 = 99;
                    } else if number == 0 {
                        self.item_bag[i].1 = 0;
                        self.item_bag[i].0 = "Nothing".to_string();
                    } else {
                        self.item_bag[i].1 = number;
                    }
                    self.save_file.save_item_pocket(self.item_bag.clone());
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::BallChanged(i, selected) => {
                self.ball_bag[i].0 = selected;
                if self.ball_bag[i].0 == "Nothing" {
                    self.ball_bag[i].1 = 0;
                } else if self.ball_bag[i].1 == 0 {
                    self.ball_bag[i].1 = 1;
                }
                self.save_file.save_ball_pocket(self.ball_bag.clone());
                self.update(Message::UpdateChanges)
            }
            Message::BallQuantityChanged(i, mut value) => {
                value.retain(|c| c.is_numeric());
                if let Ok(number) = value.parse::<u16>() {
                    if number >= 99 {
                        self.ball_bag[i].1 = 99;
                    } else if number == 0 {
                        self.ball_bag[i].1 = 0;
                        self.ball_bag[i].0 = "Nothing".to_string();
                    } else {
                        self.ball_bag[i].1 = number;
                    }
                    self.save_file.save_ball_pocket(self.ball_bag.clone());
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::BerryChanged(i, selected) => {
                self.berry_bag[i].0 = selected;
                if self.berry_bag[i].0 == "Nothing" {
                    self.berry_bag[i].1 = 0;
                } else if self.berry_bag[i].1 == 0 {
                    self.berry_bag[i].1 = 1;
                }
                self.save_file.save_berry_pocket(self.berry_bag.clone());
                self.update(Message::UpdateChanges)
            }
            Message::BerryQuantityChanged(i, mut value) => {
                value.retain(|c| c.is_numeric());
                if let Ok(number) = value.parse::<u16>() {
                    if number >= 99 {
                        self.berry_bag[i].1 = 99;
                    } else if number == 0 {
                        self.berry_bag[i].1 = 0;
                        self.berry_bag[i].0 = "Nothing".to_string();
                    } else {
                        self.berry_bag[i].1 = number;
                    }
                    self.save_file.save_berry_pocket(self.berry_bag.clone());
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::TmChanged(i, selected) => {
                self.tm_bag[i].0 = selected;
                if self.tm_bag[i].0 == "Nothing" {
                    self.tm_bag[i].1 = 0;
                } else if self.tm_bag[i].1 == 0 {
                    self.tm_bag[i].1 = 1;
                }
                self.save_file.save_tm_pocket(self.tm_bag.clone());
                self.update(Message::UpdateChanges)
            }
            Message::TmQuantityChanged(i, mut value) => {
                value.retain(|c| c.is_numeric());
                if let Ok(number) = value.parse::<u16>() {
                    if number >= 99 {
                        self.tm_bag[i].1 = 99;
                    } else if number == 0 {
                        self.tm_bag[i].1 = 0;
                        self.tm_bag[i].0 = "Nothing".to_string();
                    } else {
                        self.tm_bag[i].1 = number;
                    }
                    self.save_file.save_tm_pocket(self.tm_bag.clone());
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::KeyChanged(i, selected) => {
                self.key_bag[i].0 = selected;
                if self.key_bag[i].0 == "Nothing" {
                    self.key_bag[i].1 = 0;
                } else if self.key_bag[i].1 == 0 {
                    self.key_bag[i].1 = 1;
                }
                self.save_file.save_key_pocket(self.key_bag.clone());
                self.update(Message::UpdateChanges)
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            button("Open File").on_press(Message::OpenFile),
            button("Save File").on_press(Message::SaveFile),
            button("Pokemon").on_press(Message::PokemonView),
            button("Bag").on_press(Message::BagView),
        ];

        let pokemon_view = row![
            column![
                controls,
                row![
                    party(&self.selected_pokemon.offset(), &self.party),
                    pc_box(
                        &self.selected_pokemon.offset(),
                        &self.current_pc_index,
                        &self.current_pc
                    )
                ]
                .padding([0, 0, 0, 10])
                .spacing(10)
            ]
            .spacing(20),
            pokemon_info(&self.selected_pokemon)
        ]
        .spacing(10);

        let controls = row![
            button("Open File").on_press(Message::OpenFile),
            button("Save File").on_press(Message::SaveFile),
            button("Pokemon").on_press(Message::PokemonView),
            button("Bag").on_press(Message::BagView),
        ];

        let bag_view = row![column![
            controls,
            if !self.save_file.is_empty() {
                row![Scrollable::new(row![
                    items_bag(self.item_bag.clone()),
                    balls_bag(self.ball_bag.clone()),
                    tms_bag(self.tm_bag.clone()),
                    berries_bag(self.berry_bag.clone()),
                    keys_bag(self.key_bag.clone()),
                ])
                .direction(Direction::Horizontal(Properties::new()))]
            } else {
                row![]
            }
        ]
        .spacing(20),]
        .spacing(10);

        let view = if self.pokemon_view {
            pokemon_view
        } else {
            bag_view
        };

        container(view).into()
    }
}

fn items_bag(bag: Vec<(String, u16)>) -> Column<'static, Message> {
    let mut column = column![];
    for (i, (item, quantity)) in bag.into_iter().enumerate() {
        let item_image = if let Some(item_index) = transpose_item(&item) {
            if let Some(item_image) =
                PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_index, width = 4))
            {
                let handle = image::Handle::from_memory(item_image.contents());
                image(handle).height(40)
            } else {
                image("").width(40)
            }
        } else {
            image("").width(40)
        };

        column = column.push(
            row![
                item_image,
                pick_list(items_list(), Some(item), move |selected| {
                    Message::ItemChanged(i, selected)
                })
                .width(150)
                .text_line_height(text::LineHeight::Absolute(10.into())),
                text_input(&quantity.to_string(), &quantity.to_string())
                    .on_input(move |input| Message::ItemQuantityChanged(i, input))
                    .line_height(text::LineHeight::Absolute(10.into()))
                    .width(30)
                    .size(12),
            ]
            .align_items(Alignment::Center)
            .spacing(5)
            .width(250)
            .height(40.0),
        )
    }
    column![text("Items"), scrollable(column)].align_items(Alignment::Center)
}

fn balls_bag(bag: Vec<(String, u16)>) -> Column<'static, Message> {
    let mut column = column![];
    for (i, (item, quantity)) in bag.into_iter().enumerate() {
        let item_image = if let Some(item_index) = transpose_item(&item) {
            if let Some(item_image) =
                PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_index, width = 4))
            {
                let handle = image::Handle::from_memory(item_image.contents());
                image(handle).height(40)
            } else {
                image("").width(40)
            }
        } else {
            image("").width(40)
        };

        column = column.push(
            row![
                item_image,
                pick_list(balls_list(), Some(item), move |selected| {
                    Message::BallChanged(i, selected)
                })
                .width(150)
                .text_line_height(text::LineHeight::Absolute(10.into())),
                text_input(&quantity.to_string(), &quantity.to_string())
                    .on_input(move |input| Message::BallQuantityChanged(i, input))
                    .line_height(text::LineHeight::Absolute(10.into()))
                    .width(30)
                    .size(12),
            ]
            .align_items(Alignment::Center)
            .spacing(5)
            .width(250)
            .height(40.0),
        )
    }
    column![text("Pokeballs"), scrollable(column)].align_items(Alignment::Center)
}

fn berries_bag(bag: Vec<(String, u16)>) -> Column<'static, Message> {
    let mut column = column![];
    for (i, (item, quantity)) in bag.into_iter().enumerate() {
        let item_image = if let Some(item_index) = transpose_item(&item) {
            if let Some(item_image) =
                PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_index, width = 4))
            {
                let handle = image::Handle::from_memory(item_image.contents());
                image(handle).height(40)
            } else {
                image("").width(40)
            }
        } else {
            image("").width(40)
        };

        column = column.push(
            row![
                item_image,
                pick_list(berries_list(), Some(item), move |selected| {
                    Message::BerryChanged(i, selected)
                })
                .width(150)
                .text_line_height(text::LineHeight::Absolute(10.into())),
                text_input(&quantity.to_string(), &quantity.to_string())
                    .on_input(move |input| Message::BerryQuantityChanged(i, input))
                    .line_height(text::LineHeight::Absolute(10.into()))
                    .width(30)
                    .size(12),
            ]
            .align_items(Alignment::Center)
            .spacing(5)
            .width(250)
            .height(40.0),
        )
    }
    column![text("Berries"), scrollable(column)].align_items(Alignment::Center)
}

fn tms_bag(bag: Vec<(String, u16)>) -> Column<'static, Message> {
    let mut column = column![];
    for (i, (item, quantity)) in bag.into_iter().enumerate() {
        let item_image = if let Some(item_index) = transpose_item(&item) {
            if let Some(item_image) =
                PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_index, width = 4))
            {
                let handle = image::Handle::from_memory(item_image.contents());
                image(handle).height(40)
            } else {
                image("").width(40)
            }
        } else {
            image("").width(40)
        };

        column = column.push(
            row![
                item_image,
                pick_list(tm_list(), Some(item), move |selected| {
                    Message::TmChanged(i, selected)
                })
                .width(150)
                .text_line_height(text::LineHeight::Absolute(10.into())),
                text_input(&quantity.to_string(), &quantity.to_string())
                    .on_input(move |input| Message::TmQuantityChanged(i, input))
                    .line_height(text::LineHeight::Absolute(10.into()))
                    .width(30)
                    .size(12),
            ]
            .align_items(Alignment::Center)
            .spacing(5)
            .width(250)
            .height(40.0),
        )
    }
    column![text("Machines"), scrollable(column)].align_items(Alignment::Center)
}

fn keys_bag(bag: Vec<(String, u16)>) -> Column<'static, Message> {
    let mut column = column![];
    for (i, (item, quantity)) in bag.into_iter().enumerate() {
        let item_image = if let Some(item_index) = transpose_item(&item) {
            if let Some(item_image) =
                PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_index, width = 4))
            {
                let handle = image::Handle::from_memory(item_image.contents());
                image(handle).height(40)
            } else {
                image("").width(40)
            }
        } else {
            image("").width(40)
        };

        column = column.push(
            row![
                item_image,
                pick_list(key_list(), Some(item), move |selected| {
                    Message::KeyChanged(i, selected)
                })
                .width(150)
                .text_line_height(text::LineHeight::Absolute(10.into())),
            ]
            .align_items(Alignment::Center)
            .spacing(5)
            .width(250)
            .height(40.0),
        )
    }
    column![text("Key Items"), scrollable(column)].align_items(Alignment::Center)
}

async fn load() -> Result<(), String> {
    Ok(())
}

async fn pick_file() -> Result<PathBuf, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a file...")
        .add_filter("Save File", &["sav"])
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    Ok(handle.path().to_owned())
}

async fn load_file(path: PathBuf) -> Result<Arc<Vec<u8>>, Error> {
    let contents = tokio::fs::read(&path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)?;

    Ok(contents)
}

async fn write_file(path: PathBuf, contents: Option<Arc<Vec<u8>>>) -> Result<(), Error> {
    match contents {
        Some(content) => tokio::fs::write(path, content.as_ref())
            .await
            .map_err(|error| error.kind())
            .map_err(Error::IO)?,
        None => {
            return Err(Error::NoFileOpened);
        }
    }
    Ok(())
}
