use iced::widget::container;
use iced::{Element, Task, Theme};

use iced::advanced::widget::Id;

use std::path::PathBuf;
use std::sync::Arc;

use pk_editor::error::Error;
use pk_editor::message::Message;
use pk_editor::misc::{WINDOW_HEIGHT, WINDOW_WIDTH};
use pk_editor::{bag, icon, party_box};

use pk_edit::data_structure::pokemon::{gen_pokemon_from_species, Pokemon, Pokerus};
use pk_edit::misc::extract_db;
use pk_edit::{SaveFile, StorageType};

fn main() -> iced::Result {
    if extract_db().is_ok() {};

    iced::application("PK_Editor", State::update, State::view)
        .centered()
        .font(icon::FONT)
        .theme(State::theme)
        .window_size([WINDOW_WIDTH + 50.0, WINDOW_HEIGHT])
        .run_with(State::new)
}

#[derive(Default, Debug)]
pub struct State {
    theme: Theme,
    save_file: SaveFile,
    party: Vec<Pokemon>,
    selected: Option<Id>,
    error: Option<Error>,
    screen: Option<Screen>,
    current_pc_index: usize,
    selected_tab: Option<Id>,
    selected_bag: Option<Id>,
    current_pc: Vec<Pokemon>,
    selected_pokemon: Option<Pokemon>,
    tm_bag: Vec<(String, u16)>,
    key_bag: Vec<(String, u16)>,
    item_bag: Vec<(String, u16)>,
    ball_bag: Vec<(String, u16)>,
    berry_bag: Vec<(String, u16)>,
    selected_pokemon_storage: StorageType,
}

#[derive(Debug)]
pub enum Screen {
    PartyBoxes,
    Bag,
    Trainer,
}

impl State {
    fn new() -> (Self, Task<Message>) {
        (
            State {
                error: None,
                tm_bag: vec![],
                selected: None,
                key_bag: vec![],
                item_bag: vec![],
                ball_bag: vec![],
                berry_bag: vec![],
                current_pc_index: 0,
                selected_pokemon: None,
                theme: iced::Theme::Dracula,
                save_file: SaveFile::default(),
                selected_tab: Some(Id::new("1")),
                selected_bag: Some(Id::new("1")),
                screen: Some(Screen::PartyBoxes),
                party: vec![Pokemon::default(); 6],
                current_pc: vec![Pokemon::default(); 30],
                selected_pokemon_storage: StorageType::None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFile => Task::perform(pick_file(), Message::FileOpened),
            Message::SaveFile => Task::perform(pick_file(), Message::FileSaved),
            Message::FileOpened(Ok(path)) => Task::perform(load_file(path), Message::LoadFile),
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Task::none()
            }
            Message::FileSaved(Ok(path)) => {
                if !self.save_file.is_empty() {
                    Task::perform(
                        write_file(path, Some(Arc::new(self.save_file.raw_data()))),
                        Message::WriteFile,
                    )
                } else {
                    Task::none()
                }
            }
            Message::FileSaved(Err(error)) => {
                self.error = Some(error);
                Task::none()
            }
            Message::LoadFile(Ok(results)) => {
                self.selected = None;
                self.tm_bag = vec![];
                self.key_bag = vec![];
                self.item_bag = vec![];
                self.ball_bag = vec![];
                self.berry_bag = vec![];
                self.current_pc_index = 0;
                self.selected_pokemon = None;
                self.save_file = SaveFile::new(&results);
                self.selected_pokemon_storage = StorageType::None;

                self.update(Message::UpdateChanges)
            }
            Message::LoadFile(Err(error)) => {
                self.error = Some(error);
                Task::none()
            }
            Message::WriteFile(Ok(_)) => Task::none(),
            Message::WriteFile(Err(error)) => {
                self.error = Some(error);
                Task::none()
            }
            Message::UpdateChanges => {
                if let Some(mut selected_pokemon) = &self.selected_pokemon {
                    selected_pokemon.update_checksum();
                    self.save_file
                        .save_pokemon(self.selected_pokemon_storage, selected_pokemon);
                }

                self.party = self.save_file.get_party().expect("REASON");
                self.current_pc = self.save_file.pc_box(self.current_pc_index);
                self.item_bag = self.save_file.item_pocket().expect("REASON");
                self.ball_bag = self.save_file.ball_pocket().expect("REASON");
                self.berry_bag = self.save_file.berry_pocket().expect("REASON");
                self.tm_bag = self.save_file.tm_pocket().expect("REASON");
                self.key_bag = self.save_file.key_pocket().expect("REASON");

                Task::none()
            }
            Message::Selected(id, storage, pokemon) => {
                self.selected = id;
                if let Some(storage) = storage {
                    self.selected_pokemon_storage = storage;
                }
                self.selected_pokemon = pokemon;
                Task::none()
            }
            Message::SelectedTab(id) => {
                self.selected_tab = Some(id);

                if self.selected_tab == Some(Id::new("1")) {
                    self.screen = Some(Screen::PartyBoxes);
                } else if self.selected_tab == Some(Id::new("2")) {
                    self.screen = Some(Screen::Bag);
                } else if self.selected_tab == Some(Id::new("3")) {
                    self.screen = Some(Screen::Trainer);
                }

                Task::none()
            }
            Message::SelectedBag(id) => {
                self.selected_bag = Some(id);

                Task::none()
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
                    Task::none()
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
                    Task::none()
                }
            }
            Message::ChangePokerusStatus => {
                if let Some(selected_pokemon) = self.selected_pokemon.as_mut() {
                    match selected_pokemon.pokerus_status() {
                        Pokerus::Infected => {
                            let _ = selected_pokemon.cure_pokerus();
                        }
                        Pokerus::Cured => {
                            let _ = selected_pokemon.remove_pokerus();
                        }
                        Pokerus::None => {
                            let _ = selected_pokemon.infect_pokerus();
                        }
                    }
                }

                self.update(Message::UpdateChanges)
            }
            Message::LevelInputChanged(mut value) => {
                if let Some(selected_pokemon) = self.selected_pokemon.as_mut() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u64>() {
                        let lowest_level = selected_pokemon.lowest_level();
                        let value = if number > 100 {
                            100
                        } else if number < lowest_level as u64 {
                            lowest_level
                        } else {
                            number as u8
                        };
                        selected_pokemon.set_level(value);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Task::none()
                }
            }
            Message::SpeciesSelected(species) => {
                if let Some(selected_pokemon) = self.selected_pokemon.as_mut() {
                    if selected_pokemon.is_empty() {
                        *selected_pokemon = gen_pokemon_from_species(
                            selected_pokemon.offset(),
                            &species,
                            &self.save_file.ot_name(),
                            &self.save_file.ot_id(),
                        );

                        self.update(Message::UpdateChanges)
                    } else {
                        selected_pokemon.set_species(&species);
                        self.update(Message::UpdateChanges)
                    }
                } else {
                    Task::none()
                }
            }
            Message::IVChanged(iv, mut value) => {
                if let Some(selected_pokemon) = self.selected_pokemon.as_mut() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u16>() {
                        selected_pokemon.stats_mut().update_ivs(&iv, number);
                    } else if value.is_empty() {
                        selected_pokemon.stats_mut().update_ivs(&iv, 0);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Task::none()
                }
            }
            Message::EVChanged(ev, mut value) => {
                if let Some(selected_pokemon) = self.selected_pokemon.as_mut() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u16>() {
                        selected_pokemon.stats_mut().update_evs(&ev, number);
                    } else if value.is_empty() {
                        selected_pokemon.stats_mut().update_evs(&ev, 0);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Task::none()
                }
            }
            Message::FriendshipChanged(mut value) => {
                if let Some(selected_pokemon) = self.selected_pokemon.as_mut() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u64>() {
                        let value = if number > u8::MAX.into() {
                            u8::MAX
                        } else {
                            number as u8
                        };
                        selected_pokemon.set_friendship(value);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Task::none()
                }
            }
            Message::HeldItemSelected(item) => {
                if let Some(selected_pokemon) = self.selected_pokemon.as_mut() {
                    selected_pokemon.give_item(&item);
                    self.update(Message::UpdateChanges)
                } else {
                    Task::none()
                }
            }
            Message::MoveSelected(index, value) => {
                if let Some(selected_pokemon) = self.selected_pokemon.as_mut() {
                    selected_pokemon.set_move(index, &value);
                    self.update(Message::UpdateChanges)
                } else {
                    Task::none()
                }
            }
            Message::AddMove(index) => {
                if let Some(selected_pokemon) = self.selected_pokemon.as_mut() {
                    selected_pokemon.set_move(index, "Pound");
                    self.update(Message::UpdateChanges)
                } else {
                    Task::none()
                }
            }
            /*Message::NatureSelected(_nature) => {
                //self.selected_pokemon.set_nature(&nature);
                self.update(Message::UpdateChanges)
            }*/
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
                    Task::none()
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
                    Task::none()
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
                    Task::none()
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
                    Task::none()
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
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(match self.screen {
            Some(Screen::PartyBoxes) => party_box(
                &self.selected,
                &self.selected_tab,
                &self.selected_pokemon,
                &self.party,
                &self.current_pc_index,
                &self.current_pc,
            ),
            Some(Screen::Bag) => bag(
                &self.selected_bag,
                &self.selected_tab,
                &self.item_bag,
                &self.ball_bag,
                &self.berry_bag,
                &self.tm_bag,
                &self.key_bag,
            ),
            _ => container("").into(),
        })
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
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
