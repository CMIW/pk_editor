#![windows_subsystem = "windows"]
use iced::advanced::widget::Id;
use iced::widget::container;
use iced::widget::opaque;
use iced::{Element, Task, Theme};

use std::path::PathBuf;
use std::sync::Arc;

use pk_editor::error::Error;
use pk_editor::message::Message;
use pk_editor::misc::{WINDOW_HEIGHT, WINDOW_WIDTH};
use pk_editor::{bag, icon, party_box};

use pk_edit::data_structure::pokemon::{gen_pokemon_from_species, Pokemon, Pokerus};
use pk_edit::misc::extract_db;
use pk_edit::{Pocket, SaveFile, StorageType};
use pk_editor::menu_bar;
use pk_editor::pokemon_info;

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
    show_modal: bool,
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
    cb_state: iced::widget::combo_box::State<String>,
}

#[derive(Debug)]
pub enum Screen {
    PartyBoxes,
    BagTrainer,
}

impl State {
    fn new() -> (Self, Task<Message>) {
        use pk_edit::misc::species;

        let species = match species() {
            Ok(sps) => sps,
            Err(_) => {
                vec![String::from("")]
            }
        };

        (
            State {
                error: None,
                tm_bag: vec![],
                selected: None,
                key_bag: vec![],
                item_bag: vec![],
                ball_bag: vec![],
                berry_bag: vec![],
                show_modal: false,
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
                cb_state: iced::widget::combo_box::State::new(species),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MenuBar(message) => match message {
                menu_bar::Message::OpenFile => {
                    self.show_modal = true;
                    Task::perform(pick_file(), Message::FileOpened)
                }
                menu_bar::Message::SaveFile => {
                    self.show_modal = true;
                    Task::perform(save_file(), Message::FileSaved)
                }
                menu_bar::Message::SelectedTab(id) => {
                    self.selected_tab = Some(id);

                    if self.selected_tab == Some(Id::new("1")) {
                        self.screen = Some(Screen::PartyBoxes);
                    } else if self.selected_tab == Some(Id::new("2")) {
                        self.screen = Some(Screen::BagTrainer);
                    }

                    Task::none()
                }
            },
            Message::Bag(message) => {
                bag::update(
                    &mut self.tm_bag,
                    &mut self.key_bag,
                    &mut self.item_bag,
                    &mut self.ball_bag,
                    &mut self.berry_bag,
                    &mut self.save_file,
                    &mut self.selected_bag,
                    message,
                );
                self.update(Message::UpdateChanges)
            }
            Message::PokemonInfo(message) => {
                pokemon_info::update(
                    &mut self.selected_pokemon,
                    &self.save_file.ot_name(),
                    &self.save_file.ot_id(),
                    message,
                );
                self.update(Message::UpdateChanges)
            }
            Message::FileOpened(Ok(path)) => Task::perform(load_file(path), Message::LoadFile),
            Message::FileOpened(Err(error)) => {
                match error {
                    Error::DialogClosed => self.show_modal = false,
                    _ => self.error = Some(error),
                }
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
                match error {
                    Error::DialogClosed => self.show_modal = false,
                    _ => self.error = Some(error),
                }
                Task::none()
            }
            Message::LoadFile(Ok(results)) => {
                self.show_modal = false;
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
                self.show_modal = false;
                self.error = Some(error);
                Task::none()
            }
            Message::WriteFile(Ok(_)) => Task::perform(save_success_dialog(), |_| Message::HideModal),
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
                self.item_bag = self.save_file.pocket(Pocket::Items).expect("REASON");
                self.ball_bag = self.save_file.pocket(Pocket::Pokeballs).expect("REASON");
                self.berry_bag = self.save_file.pocket(Pocket::Berries).expect("REASON");
                self.tm_bag = self.save_file.pocket(Pocket::Tms).expect("REASON");
                self.key_bag = self.save_file.pocket(Pocket::Key).expect("REASON");

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
            Message::Loaded(_) => todo!(), //_ => Task::none(),
            Message::HideModal => {
                self.show_modal = false;
                Task::none()
            }
            Message::ShowModal => todo!(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = container(match self.screen {
            Some(Screen::PartyBoxes) => party_box(
                &self.cb_state,
                &self.selected,
                &self.selected_tab,
                &self.selected_pokemon,
                &self.party,
                &self.current_pc_index,
                &self.current_pc,
            ),
            Some(Screen::BagTrainer) => bag(
                &self.selected_bag,
                &self.selected_tab,
                &self.item_bag,
                &self.ball_bag,
                &self.berry_bag,
                &self.tm_bag,
                &self.key_bag,
            ),
            _ => container("").into(),
        });

        if self.show_modal {
            iced::widget::stack![
                content,
                opaque(
                    container("")
                        .width(WINDOW_WIDTH + 50.0)
                        .height(WINDOW_HEIGHT)
                        .style(|_theme| {
                            container::Style {
                                background: Some(
                                    iced::Color {
                                        a: 0.8,
                                        ..iced::Color::BLACK
                                    }
                                    .into(),
                                ),
                                ..container::Style::default()
                            }
                        })
                )
            ]
            .width(WINDOW_WIDTH + 50.0)
            .height(WINDOW_HEIGHT)
            .into()
        } else {
            content.into()
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

async fn save_success_dialog() {
    rfd::AsyncMessageDialog::new()
        .set_title("Saved")
        .set_level(rfd::MessageLevel::Info)
        .set_description("Backup succesful!")
        .set_buttons(rfd::MessageButtons::Ok)
        .show()
        .await;
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

async fn save_file() -> Result<PathBuf, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a file...")
        .add_filter("Save File", &["sav"])
        .save_file()
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
