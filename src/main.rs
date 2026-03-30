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
use pk_edit::pokemon::Pokemon;
use pk_edit::save::storage::Pocket;
use pk_edit::save::storage::StorageType;
use pk_edit::SaveFile;
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
    images: HashMap<String, image::Handle>,
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
                save_file: SaveFile::default(),
                selected_tab: Some(Id::new("1")),
                selected_bag: Some(Id::new("1")),
                screen: Some(Screen::PartyBoxes),
                //screen: None,
                party: vec![Pokemon::default(); 6],
                current_pc: vec![Pokemon::default(); 30],
                selected_pokemon_storage: StorageType::None,
                cb_state: iced::widget::combo_box::State::new(species),
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
                if let Err(error) = bag::update(
                    &mut self.tm_bag,
                    &mut self.key_bag,
                    &mut self.item_bag,
                    &mut self.ball_bag,
                    &mut self.berry_bag,
                    &mut self.save_file,
                    &mut self.selected_bag,
                    message,
                ) {
                    let error_msg = error.to_string();
                    return Task::perform(save_error_dialog(error_msg), |_| Message::HideModal);
                }
                self.update(Message::UpdateChanges)
            }
            Message::PokemonInfo(message) => {
                if let Err(error) = pokemon_info::update(
                    &mut self.selected_pokemon,
                    &self.save_file.ot_name(),
                    &self.save_file.ot_id(),
                    message,
                ) {
                    let error_msg = error.to_string();
                    return Task::perform(save_error_dialog(error_msg), |_| Message::HideModal);
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
                if let Some(mut selected_pokemon) = &self.selected_pokemon {
                    selected_pokemon.update_checksum();
                    if let Err(error) = self
                        .save_file
                        .save_pokemon(self.selected_pokemon_storage, selected_pokemon)
                    {
                        let error_msg = error.to_string();
                        return Task::perform(save_error_dialog(error_msg), |_| Message::HideModal);
                    }
                }

                self.party = self.save_file.get_party().expect("REASON");
                self.current_pc = self
                    .save_file
                    .pc_box(self.current_pc_index)
                    .expect("REASON");
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
            Message::DragStart(offset, storage, origin, nat_dex_number, i) => {
                tracing::debug!(?storage, offset, i, "DragStart");
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
            Message::DragDrop(_, to_storage, to_index) => {
                tracing::debug!(?to_storage, to_index, "DragDrop");
                if let Some(from) = self.drag.take() {
                    let from_pokemon = match from.storage {
                        StorageType::PC => self.current_pc[from.index],
                        StorageType::Party => self.party[from.index],
                        StorageType::None => return Task::none(),
                    };
                    let to_pokemon = match to_storage {
                        StorageType::PC => self.current_pc[to_index],
                        StorageType::Party => self.party[to_index],
                        StorageType::None => return Task::none(),
                    };
                    self.save_file
                        .swap_pokemon(from_pokemon, from.storage, to_pokemon, to_storage)
                        .expect("REASON");

                    return self.update(Message::UpdateChanges);
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
                &self.images,
                &self.drag,
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
            //let offset = drag.cursor - drag.origin;
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
