#![windows_subsystem = "windows"]
use iced::advanced::widget::Id;
use iced::event;
use iced::mouse;
use iced::widget::container;
use iced::widget::image;
use iced::widget::opaque;
use iced::Subscription;
use iced::Vector;
use iced::{Element, Task, Theme};

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tracing_subscriber;

use pk_editor::error::Error;
use pk_editor::message::Message;
use pk_editor::misc::{PROJECT_DIR, WINDOW_HEIGHT, WINDOW_WIDTH};
use pk_editor::DragState;
use pk_editor::{bag, icon, party_box};

use pk_edit::misc::extract_db;
use pk_edit::{AnyPokemon, GameData, Gen3Pocket as Pocket, OpenSave, PokemonTrait, StorageType};
use pk_editor::menu_bar;
use pk_editor::pokemon_info;

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    if extract_db().is_ok() {};

    iced::application(State::new, State::update, State::view)
        .subscription(State::subscription)
        .title("PK_Editor")
        .centered()
        .font(icon::FONT)
        .theme(State::theme)
        .window_size([WINDOW_WIDTH + 50.0, WINDOW_HEIGHT])
        .run()
}

#[derive(Debug)]
pub struct State {
    theme: Theme,
    show_modal: bool,
    drag: Option<DragState>,
    save_file: Option<OpenSave>,
    party: Vec<AnyPokemon>,
    selected: Option<Id>,
    error: Option<Error>,
    screen: Option<Screen>,
    current_pc_index: usize,
    selected_tab: Option<Id>,
    selected_bag: Option<Id>,
    current_pc: Vec<AnyPokemon>,
    selected_pokemon: Option<AnyPokemon>,
    tm_bag: Vec<(String, u16)>,
    key_bag: Vec<(String, u16)>,
    item_bag: Vec<(String, u16)>,
    ball_bag: Vec<(String, u16)>,
    berry_bag: Vec<(String, u16)>,
    selected_pokemon_storage: StorageType,
    cb_state: iced::widget::combo_box::State<String>,
    images: HashMap<String, image::Handle>,
}

#[derive(Debug)]
pub enum Screen {
    PartyBoxes,
    BagTrainer,
}

impl State {
    fn new() -> (Self, Task<Message>) {
        (
            State {
                error: None,
                tm_bag: vec![],
                drag: None,
                selected: None,
                key_bag: vec![],
                item_bag: vec![],
                ball_bag: vec![],
                berry_bag: vec![],
                show_modal: false,
                current_pc_index: 0,
                selected_pokemon: None,
                theme: iced::Theme::Dracula,
                save_file: None,
                selected_tab: Some(Id::new("1")),
                selected_bag: Some(Id::new("1")),
                screen: Some(Screen::PartyBoxes),
                party: vec![],
                current_pc: vec![],
                selected_pokemon_storage: StorageType::None,
                cb_state: iced::widget::combo_box::State::new(vec![]),
                images: HashMap::new(),
            },
            Task::perform(load_images(), Message::ImagesListed),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ImagesListed(Ok(images)) => {
                self.images = images;

                Task::none()
            }
            Message::ImagesListed(Err(error)) => {
                match error {
                    Error::DialogClosed => self.show_modal = false,
                    _ => {
                        let error_msg = error.to_string();
                        return Task::perform(save_error_dialog(error_msg), |_| Message::HideModal);
                    }
                }
                Task::none()
            }
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
                if let Some(OpenSave::Gen3(ref mut gen3)) = self.save_file {
                    if let Err(error) = bag::update(
                        &mut self.tm_bag,
                        &mut self.key_bag,
                        &mut self.item_bag,
                        &mut self.ball_bag,
                        &mut self.berry_bag,
                        gen3,
                        &mut self.selected_bag,
                        message,
                    ) {
                        let error_msg = error.to_string();
                        return Task::perform(save_error_dialog(error_msg), |_| Message::HideModal);
                    }
                }
                self.update(Message::UpdateChanges)
            }
            Message::PokemonInfo(message) => {
                if let Some(ref save_file) = self.save_file {
                    let factory = save_file.pokemon_factory();
                    let game_data = save_file.game_data();
                    let ot_name = save_file.trainer_name();
                    let ot_id = save_file.trainer_id();
                    if let Err(error) = pokemon_info::update(
                        &mut self.selected_pokemon,
                        &factory,
                        &game_data,
                        &ot_name,
                        ot_id,
                        message,
                    ) {
                        let error_msg = error.to_string();
                        return Task::perform(save_error_dialog(error_msg), |_| Message::HideModal);
                    }
                }
                self.update(Message::UpdateChanges)
            }
            Message::FileOpened(Ok(path)) => Task::perform(load_file(path), Message::LoadFile),
            Message::FileOpened(Err(error)) => {
                match error {
                    Error::DialogClosed => self.show_modal = false,
                    _ => {
                        let error_msg = error.to_string();
                        return Task::perform(save_error_dialog(error_msg), |_| Message::HideModal);
                    }
                }
                Task::none()
            }
            Message::FileSaved(Ok(path)) => {
                if let Some(ref save_file) = self.save_file {
                    Task::perform(
                        write_file(path, Some(Arc::new(save_file.raw_data()))),
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
                self.selected_pokemon_storage = StorageType::None;

                match pk_edit::open(&results) {
                    Ok(save_file) => {
                        let species = save_file.game_data().species().unwrap_or_default();
                        self.cb_state = iced::widget::combo_box::State::new(species);
                        self.save_file = Some(save_file);
                    }
                    Err(error) => {
                        let error_msg = error.to_string();
                        return Task::perform(save_error_dialog(error_msg), |_| Message::HideModal);
                    }
                }

                self.update(Message::UpdateChanges)
            }
            Message::LoadFile(Err(error)) => {
                let error_msg = error.to_string();
                Task::perform(save_error_dialog(error_msg), |_| Message::HideModal)
            }
            Message::WriteFile(Ok(_)) => {
                Task::perform(save_success_dialog(), |_| Message::HideModal)
            }
            Message::WriteFile(Err(error)) => {
                let error_msg = error.to_string();
                Task::perform(save_error_dialog(error_msg), |_| Message::HideModal)
            }
            Message::UpdateChanges => {
                let Some(ref mut save_file) = self.save_file else {
                    return Task::none();
                };

                if let Some(mut selected_pokemon) = self.selected_pokemon {
                    selected_pokemon.update_checksum();
                    if let Err(error) = save_file
                        .save_pokemon(self.selected_pokemon_storage, selected_pokemon)
                    {
                        let error_msg = error.to_string();
                        return Task::perform(save_error_dialog(error_msg), |_| Message::HideModal);
                    }
                }

                self.party = save_file.party().unwrap_or_default();
                self.current_pc = save_file.pc_box(self.current_pc_index).unwrap_or_default();
                self.item_bag = save_file.pocket(Pocket::Items).unwrap_or_default();
                self.ball_bag = save_file.pocket(Pocket::Pokeballs).unwrap_or_default();
                self.berry_bag = save_file.pocket(Pocket::Berries).unwrap_or_default();
                self.tm_bag = save_file.pocket(Pocket::Tms).unwrap_or_default();
                self.key_bag = save_file.pocket(Pocket::Key).unwrap_or_default();

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
                if self.save_file.as_ref().is_some_and(|s| !s.is_pc_empty()) {
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
                if self.save_file.as_ref().is_some_and(|s| !s.is_pc_empty()) {
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
            Message::DragStart(storage, origin, nat_dex_number, i) => {
                tracing::debug!(?storage, i, "DragStart");
                let handle = self
                    .images
                    .get(&format!("{:0width$}", nat_dex_number, width = 4))
                    .unwrap_or({
                        let width = 10;
                        let height = 10;
                        let size = (width * height) as usize;
                        let pixels = vec![0u8; size * 4];
                        &image::Handle::from_rgba(width, height, pixels)
                    })
                    .clone();

                self.drag = Some(DragState {
                    storage,
                    cursor: origin,
                    handle,
                    index: i,
                });
                Task::none()
            }
            Message::DragMoved(pos) => {
                if let Some(d) = &mut self.drag {
                    d.cursor = pos;
                }
                Task::none()
            }
            Message::DragDrop(to_storage, to_index) => {
                tracing::debug!(?to_storage, to_index, "DragDrop");
                if let Some(from) = self.drag.take() {
                    let from_pokemon = match from.storage {
                        StorageType::PC => self.current_pc.get(from.index).copied(),
                        StorageType::Party => self.party.get(from.index).copied(),
                        StorageType::None => None,
                    };
                    let to_pokemon = match to_storage {
                        StorageType::PC => self.current_pc.get(to_index).copied(),
                        StorageType::Party => self.party.get(to_index).copied(),
                        StorageType::None => None,
                    };
                    if let (Some(from_pokemon), Some(to_pokemon), Some(ref mut save_file)) =
                        (from_pokemon, to_pokemon, self.save_file.as_mut())
                    {
                        if let Err(error) = save_file.swap_pokemon(
                            from_pokemon,
                            from.storage,
                            to_pokemon,
                            to_storage,
                        ) {
                            let error_msg = error.to_string();
                            return Task::perform(
                                save_error_dialog(error_msg),
                                |_| Message::HideModal,
                            );
                        }
                        return self.update(Message::UpdateChanges);
                    }
                }
                Task::none()
            }
            Message::DragReleased => {
                if self.drag.is_some() {
                    tracing::debug!("DragReleased — cancelling drag");
                    self.drag = None;
                } else {
                    tracing::debug!("DragReleased — already handled by DragDrop");
                }
                Task::none()
            }
            Message::Loaded(_) => todo!(),
            Message::HideModal => {
                self.show_modal = false;
                Task::none()
            }
            Message::ShowModal => todo!(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let game_data = self
            .save_file
            .as_ref()
            .map(|s| s.game_data())
            .unwrap_or_default();

        let scale = (WINDOW_WIDTH + 50.0) / 1130.0;
        let scale = scale.max(1.0);

        let content = container(match self.screen {
            Some(Screen::PartyBoxes) => party_box(
                &self.cb_state,
                &self.selected,
                &self.selected_tab,
                &self.selected_pokemon,
                &game_data,
                &self.party,
                &self.current_pc_index,
                &self.current_pc,
                &self.images,
                &self.drag,
                scale,
            ),
            Some(Screen::BagTrainer) => bag(
                &self.selected_bag,
                &self.selected_tab,
                &self.item_bag,
                &self.ball_bag,
                &self.berry_bag,
                &self.tm_bag,
                &self.key_bag,
                &self.images,
            ),
            _ => container("").into(),
        });

        if self.show_modal {
            let layers = iced::widget::Stack::new().push(content);

            layers
                .push(opaque(
                    container("")
                        .width(WINDOW_WIDTH + 50.0)
                        .height(WINDOW_HEIGHT)
                        .style(|_theme| container::Style {
                            background: Some(
                                iced::Color {
                                    a: 0.8,
                                    ..iced::Color::BLACK
                                }
                                .into(),
                            ),
                            ..container::Style::default()
                        }),
                ))
                .width(WINDOW_WIDTH + 50.0)
                .height(WINDOW_HEIGHT)
                .into()
        } else if let Some(drag) = &self.drag {
            iced::widget::stack![
                content,
                iced::widget::float(image(drag.handle.clone()).width(80).height(80)).translate(
                    move |b, _vp| {
                        Vector::new(drag.cursor.x - b.x - 40.0, drag.cursor.y - b.y - 40.0)
                    },
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

    fn subscription(&self) -> Subscription<Message> {
        if self.drag.is_some() {
            event::listen_with(|event, _, _| match event {
                event::Event::Mouse(mouse::Event::CursorMoved { position, .. }) => {
                    Some(Message::DragMoved(position))
                }
                event::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    Some(Message::DragReleased)
                }
                _ => None,
            })
        } else {
            Subscription::none()
        }
    }
}

async fn load_images() -> Result<HashMap<String, image::Handle>, Error> {
    let folders = ["icons", "Items", "Pokemon", "Types"];
    let mut images: HashMap<String, image::Handle> = HashMap::new();
    for folder in folders {
        let dir = PROJECT_DIR
            .get_dir(folder)
            .ok_or_else(|| Error::MissingDirectory(folder.to_string()))?;

        images.extend(dir.files().filter_map(|f| {
            let name = f.path().file_stem()?.to_str()?.to_owned();
            Some((name, image::Handle::from_bytes(f.contents())))
        }));
    }

    Ok(images)
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

async fn save_error_dialog(message: String) {
    rfd::AsyncMessageDialog::new()
        .set_title("Error")
        .set_level(rfd::MessageLevel::Error)
        .set_description(&message)
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
