use iced::application::Application;
use iced::executor;
use iced::font;
use iced::widget::{button, column, container, row};
use iced::Settings;
use iced::{Command, Element, Size, Theme};

use std::path::PathBuf;
use std::sync::Arc;

use pk_editor::error::Error;
use pk_editor::message::Message;
use pk_editor::misc::{WINDOW_HEIGHT, WINDOW_WIDTH};
use pk_editor::party::party;
use pk_editor::pc::pc_box;
use pk_editor::pokemon_info::pokemon_info;

use pk_edit::data_structure::pokemon::{Pokemon, Pokerus};
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
            Message::FileSaved(Ok(path)) => Command::perform(
                write_file(path, Some(Arc::new(self.save_file.raw_data()))),
                Message::WriteFile,
            ),
            Message::FileSaved(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::LoadFile(Ok(results)) => {
                self.save_file = SaveFile::new(&results);
                self.current_pc_index = 0;
                self.selected_pokemon = Pokemon::default();
                self.selected_pokemon_storage = StorageType::None;

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
                }
                self.update(Message::UpdateChanges)
            }
            Message::Decrement => {
                if self.current_pc_index > 0 {
                    self.current_pc_index -= 1;
                } else {
                    self.current_pc_index = 13;
                }
                self.update(Message::UpdateChanges)
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
                println!("{species}");
                Command::none()
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
            Message::NatureSelected(nature) => Command::none(),
            Message::HPEVChanged(mut value) => {
                if !self.selected_pokemon.is_empty() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u16>(){
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("HP", number);
                    } else if value.is_empty() {
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("HP", 0);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::AttackEVChanged(mut value) => {
                if !self.selected_pokemon.is_empty() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u16>(){
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("Attack", number);
                    }else if value.is_empty() {
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("Attack", 0);
                    }

                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::DefenseEVChanged(mut value) => {
                if !self.selected_pokemon.is_empty() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u16>(){
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("Defense", number);
                    }else if value.is_empty() {
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("Defense", 0);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::SpAtkEVChanged(mut value) => {
                if !self.selected_pokemon.is_empty() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u16>(){
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("Sp. Atk", number);
                    }else if value.is_empty() {
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("Sp. Atk", 0);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::SpDefEVChanged(mut value) => {
                if !self.selected_pokemon.is_empty() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u16>(){
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("Sp. Def", number);
                    }else if value.is_empty() {
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("Sp. Def", 0);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            Message::SpeedEVChanged(mut value) => {
                if !self.selected_pokemon.is_empty() {
                    value.retain(|c| c.is_numeric());
                    if let Ok(number) = value.parse::<u16>(){
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("Speed", number);
                    }else if value.is_empty() {
                        self.selected_pokemon
                            .stats_mut()
                            .update_evs("Speed", 0);
                    }
                    self.update(Message::UpdateChanges)
                } else {
                    Command::none()
                }
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            button("Open File").on_press(Message::OpenFile),
            button("Save File").on_press(Message::SaveFile)
        ];

        container(
            row![
                column![
                    controls,
                    row![
                        party(&self.selected_pokemon.ofsset(), &self.party),
                        pc_box(
                            &self.selected_pokemon.ofsset(),
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
            .spacing(10),
        )
        .into()
    }
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
