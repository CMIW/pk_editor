use iced::advanced::layout;
use iced::advanced::mouse;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::Layout;
use iced::advanced::Widget;
use iced::application::Application;
use iced::executor;
use iced::widget::{button, column, container, image, mouse_area, row, text, text_input};
use iced::{color, font};
use iced::{
    Alignment, Border, Color, Command, Element, Length, Padding, Rectangle, Shadow, Size, Theme,
};

use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

use crate::misc::PROJECT_DIR;
use pk_edit::data_structure::pokemon::{transpose_item, Gender, Pokemon, Pokerus};
use pk_edit::SaveFile;

const SCALE: f32 = 0.6;
pub const WINDOW_WIDTH: f32 = 1920.0 * SCALE;
pub const WINDOW_HEIGHT: f32 = 1080.0 * SCALE;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("File select dialog closed")]
    DialogClosed,
    #[error("IO Error")]
    IO(io::ErrorKind),
    #[error("No file has been opened, first open a file the you can save it")]
    NoFileOpened,
}

pub struct State {
    error: Option<Error>,
    file_path: Option<PathBuf>,
    save_data: SaveFile,
    party: Vec<Pokemon>,
    current_pc: (usize, Vec<Pokemon>),
    selected_pokemon: Pokemon,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile,
    SaveFile,
    LoadFile(Result<(PathBuf, Arc<Vec<u8>>), Error>),
    WriteFile(Result<(), Error>),
    FileOpened(Result<PathBuf, Error>),
    FileSaved(Result<PathBuf, Error>),
    FontLoaded(Result<(), font::Error>),
    SelectedPokemon(Pokemon),
    Increment,
    Decrement,
}

impl Application for State {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            State {
                file_path: None,
                error: None,
                save_data: SaveFile::default(),
                party: vec![Pokemon::default(); 6],
                current_pc: (0, vec![Pokemon::default(); 30]),
                selected_pokemon: Pokemon::default(),
            },
            Command::batch(vec![font::load(
                PROJECT_DIR
                    .get_file("Font/muktavaani/MuktaVaani-Medium.ttf")
                    .unwrap()
                    .contents(),
            )
            .map(Message::FontLoaded)]),
        )
    }

    fn title(&self) -> String {
        String::from("PK_Edit")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OpenFile => Command::perform(pick_file(), Message::FileOpened),
            Message::SaveFile => Command::perform(pick_file(), Message::FileSaved),
            Message::LoadFile(Ok((path, results))) => {
                self.file_path = Some(path);
                self.save_data = SaveFile::new(&results);
                self.party = self.save_data.get_party();
                self.current_pc = (0, self.save_data.get_box(0));

                Command::none()
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
            Message::FileOpened(Ok(path)) => Command::perform(load_file(path), Message::LoadFile),
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::FileSaved(Ok(path)) => Command::none(),
            Message::FileSaved(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::SelectedPokemon(pokemon) => {
                self.selected_pokemon = pokemon;
                Command::none()
            }
            Message::Increment => {
                let (index, _) = self.current_pc;
                if !self.save_data.is_pc_empty() {
                    if index < 13 {
                        self.current_pc = (index + 1, self.save_data.pc_box(index + 1));
                    }
                }
                Command::none()
            }
            Message::Decrement => {
                let (index, _) = self.current_pc;
                if index > 0 {
                    self.current_pc = (index - 1, self.save_data.pc_box(index - 1));
                }
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            button("Open File").on_press(Message::OpenFile),
            button("Save File").on_press(Message::SaveFile)
        ];

        let party = party(&self.selected_pokemon.ofsset(), &self.party);

        let (box_num, box_vec) = &self.current_pc;

        container(
            row![
                column![
                    controls,
                    row![
                        party,
                        pc_box(&self.selected_pokemon.ofsset(), box_num, box_vec)
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

fn gender_f_apperance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0xd65c63))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn gender_n_apperance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::TRANSPARENT),
        background: Some(iced::Background::Color(Color::TRANSPARENT)),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn gender_m_apperance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x4186d7))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn slot_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.5))),
        border: Border {
            radius: 5.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn slot_selected_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0xffcc00))),
        border: Border {
            radius: 5.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn level_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.7))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn party_label_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.5))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn info_label_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::BLACK),
        background: Some(iced::Background::Color(color!(0xffcc00))),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn pokemon_info_appearance() -> iced::widget::container::Appearance {
    iced::widget::container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(color!(0x000000, 0.5))),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}

fn party_label() -> Element<'static, Message> {
    let handle = image::Handle::from_memory(
        PROJECT_DIR
            .get_file("Pokeball_icon_white.png")
            .unwrap()
            .contents(),
    );

    let image = container(image(handle))
        .width(20.0)
        .height(20.0)
        .align_x(iced::alignment::Horizontal::Right)
        .align_y(iced::alignment::Vertical::Center);

    let row = row![image, text("Current Party")]
        .spacing(50)
        .width(240.0)
        .height(40.0)
        .padding(Padding::from([9, 10, 9, 20])); // top, right, bottom, left

    container(row)
        .width(240.0)
        .height(40.0)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .style(party_label_appearance())
        .into()
}

fn info_label(pokemon: &Pokemon) -> Element<'static, Message> {
    let pokeball = if pokemon.pokeball_caught() == 0 {
        image("").width(45).height(45)
    } else {
        let handle = image::Handle::from_memory(
            PROJECT_DIR
                .get_file(format!(
                    "Items/item_{:0width$}.png",
                    pokemon.pokeball_caught(),
                    width = 4
                ))
                .unwrap()
                .contents(),
        );

        image(handle).width(45).height(45)
    };

    let pokerus = match pokemon.pokerus_status() {
        Pokerus::None => {
            let pokerus_handle = image::Handle::from_memory(
                PROJECT_DIR
                    .get_file("PokérusIC_not_infected.png")
                    .unwrap()
                    .contents(),
            );
            image(pokerus_handle).width(30).height(30)
        }
        Pokerus::Infected => {
            let pokerus_handle = image::Handle::from_memory(
                PROJECT_DIR
                    .get_file("PokérusIC_infected.png")
                    .unwrap()
                    .contents(),
            );
            image(pokerus_handle).width(30).height(30)
        }
        Pokerus::Cured => {
            let pokerus_handle = image::Handle::from_memory(
                PROJECT_DIR
                    .get_file("PokérusIC_cured.png")
                    .unwrap()
                    .contents(),
            );
            image(pokerus_handle).width(30).height(30)
        }
    };

    let name = text(pokemon.nickname());

    let row = row![
        pokeball,
        name,
        iced::widget::Space::with_width(Length::Fill),
        pokerus,
        iced::widget::Space::with_width(25),
        level(pokemon.level()),
        gender(pokemon.gender()),
    ]
    .align_items(Alignment::Center)
    .spacing(5)
    .padding([5, 20, 5, 20]); // top, right, bottom, left

    container(row)
        .width(WINDOW_WIDTH * 0.33)
        .height(50.0)
        .style(info_label_appearance())
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Left)
        .into()
}

fn pc_box_label(box_number: usize) -> Element<'static, Message> {
    container(text(format!("Box {}", box_number)))
        .width(335)
        .height(40.0)
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Center)
        .style(slot_appearance())
        .into()
}

fn gender(gender: Gender) -> Element<'static, Message> {
    let (_text, style) = match gender {
        Gender::F => ("", gender_f_apperance()),
        Gender::M => ("", gender_m_apperance()),
        Gender::None => ("", gender_n_apperance()),
    };

    container("").width(26.0).height(26.0).style(style).into()
}

fn level(level: u8) -> Element<'static, Message> {
    container(text(format!("Lv. {}", level)))
        .width(80.0)
        .height(26.0)
        .center_x()
        .center_y()
        .style(level_appearance())
        .into()
}

fn input_level(level: u8) -> Element<'static, Message> {
    container(row![text("Lv. "), text_input("", &level.to_string())])
        .width(80.0)
        .height(26.0)
        .center_x()
        .center_y()
        .style(level_appearance())
        .into()
}

fn pokemon_info(pokemon: &Pokemon) -> Element<'static, Message> {
    let label = info_label(pokemon);

    let dex_species_lang = row![
        text(format!("No. {}", pokemon.nat_dex_number())),
        text(pokemon.species()),
        iced::widget::Space::with_width(Length::Fill),
        text(pokemon.language()),
        iced::widget::Space::with_width(45),
    ]
    .spacing(20)
    .padding([5, 10, 5, 15]); // top, right, bottom, left

    let nature = container(row![
        text("Nature").style(iced::theme::Text::Color(color!(0xffcc00))),
        iced::widget::Space::with_width(50),
        text(pokemon.nature())
    ])
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15]);

    let ability = container(row![
        text("Ability").style(iced::theme::Text::Color(color!(0xffcc00))),
        iced::widget::Space::with_width(60),
        text(pokemon.ability())
    ])
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15])
    .style(pokemon_info_appearance());

    let item_image = if let Some(item_index) = transpose_item(&pokemon.held_item()) {
        let handle = image::Handle::from_memory(
            PROJECT_DIR
                .get_file(format!("Items/item_{:0width$}.png", item_index, width = 4))
                .unwrap()
                .contents(),
        );
        image(handle).height(30)
    } else {
        image("").height(30)
    };

    let item = container(row![
        text("Held Item").style(iced::theme::Text::Color(color!(0xffcc00))),
        iced::widget::Space::with_width(30),
        item_image,
        iced::widget::Space::with_width(5),
        text(pokemon.held_item())
    ])
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15]);

    let moves = info_moves(pokemon.moves());

    container(column![
        label,
        dex_species_lang,
        pokemon_info_typing(pokemon.typing()),
        StatBar {},
        iced::widget::Space::with_height(Length::Fill),
        nature,
        ability,
        item,
        moves,
        row![].height(40)
    ])
    .width(WINDOW_WIDTH * 0.33)
    .height(WINDOW_HEIGHT)
    .style(pokemon_info_appearance())
    .align_y(iced::alignment::Vertical::Top)
    .align_x(iced::alignment::Horizontal::Center)
    .into()
}

fn info_moves(moves: Vec<(String, String, u8, u8)>) -> Element<'static, Message> {
    let mut column = column![];

    for (typing, name, pp_used, pp) in moves {
        column = column.push(move_slot(&typing, &name, pp_used, pp));
    }

    container(column)
        .width(WINDOW_WIDTH * 0.33)
        .height(160)
        .style(pokemon_info_appearance())
        .align_y(iced::alignment::Vertical::Top)
        .align_x(iced::alignment::Horizontal::Center)
        .into()
}

fn move_slot(
    move_type: &str,
    move_name: &str,
    pp_used: u8,
    pp_total: u8,
) -> Element<'static, Message> {
    let handle = image::Handle::from_memory(
        PROJECT_DIR
            .get_file(format!("Types/{}_icon_SV.png", move_type))
            .unwrap()
            .contents(),
    );

    let move_icon = image(handle)/*.width(45).height(45)*/;

    container(
        row![
            move_icon,
            text(move_name),
            iced::widget::Space::with_width(Length::Fill),
            pp(pp_used, pp_total),
            iced::widget::Space::with_width(100)
        ]
        .align_items(Alignment::Center)
        .spacing(10),
    )
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15])
    .into()
}

fn pp(pp_used: u8, pp_total: u8) -> Element<'static, Message> {
    container(text(format!("{}/{}", pp_used, pp_total)))
        .width(60)
        .height(30.0)
        .align_y(iced::alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Center)
        .style(slot_appearance())
        .into()
}

fn pokemon_info_typing(typing: Option<(String, Option<String>)>) -> Element<'static, Message> {
    match typing {
        None => container("")
            .width(WINDOW_WIDTH * 0.33)
            .height(40.0)
            .style(pokemon_info_appearance())
            .align_y(iced::alignment::Vertical::Center)
            .align_x(iced::alignment::Horizontal::Left)
            .padding([5, 15])
            .into(),
        Some(tuple) => match tuple {
            (type1, None) => {
                let handle1 = image::Handle::from_memory(
                    PROJECT_DIR
                        .get_file(format!("Types/{}IC_SV.png", type1))
                        .unwrap()
                        .contents(),
                );

                let type1 = image(handle1).width((WINDOW_WIDTH * 0.33) * 0.33);

                container(row![type1].spacing(5))
                    .width(WINDOW_WIDTH * 0.33)
                    .height(40.0)
                    .style(pokemon_info_appearance())
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Left)
                    .padding([5, 15])
                    .into()
            }
            (type1, Some(type2)) => {
                let handle1 = image::Handle::from_memory(
                    PROJECT_DIR
                        .get_file(format!("Types/{}IC_SV.png", type1))
                        .unwrap()
                        .contents(),
                );

                let handle2 = image::Handle::from_memory(
                    PROJECT_DIR
                        .get_file(format!("Types/{}IC_SV.png", type2))
                        .unwrap()
                        .contents(),
                );

                let type1 = image(handle1).width((WINDOW_WIDTH * 0.33) * 0.33);
                let type2 = image(handle2).width((WINDOW_WIDTH * 0.33) * 0.33);

                container(row![type1, type2].spacing(5))
                    .width(WINDOW_WIDTH * 0.33)
                    .height(40.0)
                    .style(pokemon_info_appearance())
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Left)
                    .padding([5, 15])
                    .into()
            }
        },
    }
}

fn pc_slot(id: &usize, pokemon: &Pokemon) -> Element<'static, Message> {
    match pokemon.is_empty() {
        false => {
            let apperance = if id == &pokemon.ofsset() {
                slot_selected_appearance()
            } else {
                slot_appearance()
            };

            let handle = image::Handle::from_memory(
                PROJECT_DIR
                    .get_file(format!(
                        "Pokemon/{:0width$}.png",
                        pokemon.nat_dex_number(),
                        width = 4
                    ))
                    .unwrap()
                    .contents(),
            );

            let image = container(image(handle))
                .width(70.0)
                .height(70.0)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Bottom);

            let container = container(image)
                .width(80.0)
                .height(80.0)
                .style(apperance)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Bottom);

            mouse_area(container)
                .on_press(Message::SelectedPokemon(pokemon.clone()))
                .into()
        }
        true => {
            let container = container("")
                .width(80.0)
                .height(80.0)
                .style(slot_appearance())
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Bottom);

            mouse_area(container)
                .on_press(Message::SelectedPokemon(pokemon.clone()))
                .into()
        }
    }
}

fn party_slot(id: &usize, pokemon: &Pokemon) -> Element<'static, Message> {
    match pokemon.is_empty() {
        false => {
            let apperance = if id == &pokemon.ofsset() {
                slot_selected_appearance()
            } else {
                slot_appearance()
            };

            let handle = image::Handle::from_memory(
                PROJECT_DIR
                    .get_file(format!(
                        "Pokemon/{:0width$}.png",
                        pokemon.nat_dex_number(),
                        width = 4
                    ))
                    .unwrap()
                    .contents(),
            );

            let level = level(pokemon.level());

            let name = text(pokemon.nickname());

            let image = container(image(handle))
                .width(80.0)
                .height(80.0)
                .align_x(iced::alignment::Horizontal::Right)
                .align_y(iced::alignment::Vertical::Bottom);

            let gender = gender(pokemon.gender());

            let level_gender = row![level, gender].spacing(5);

            let column = column![level_gender, name]
                .width(140.0)
                .height(80.0)
                .spacing(10)
                .padding(Padding::from([10, 10]));

            let container = container(row![column, image])
                .width(240.0)
                .height(80.0)
                .padding(Padding::from([0, 10])) // top/bottom, left/right
                .style(apperance);

            mouse_area(container)
                .on_press(Message::SelectedPokemon(pokemon.clone()))
                .into()
        }
        true => {
            let container = container("")
                .width(240.0)
                .height(80.0)
                .padding(Padding::from([0, 10])) // top/bottom, left/right
                .style(slot_appearance());

            mouse_area(container)
                .on_press(Message::SelectedPokemon(pokemon.clone()))
                .into()
        }
    }
}

fn party(id: &usize, party: &Vec<Pokemon>) -> Element<'static, Message> {
    let label = party_label();
    let mut col = iced::widget::Column::new().spacing(5);

    for pokemon in party {
        col = col.push(party_slot(id, pokemon));
    }

    container(column![label, col].spacing(10)).into()
}

fn pc_box(id: &usize, num: &usize, pc_box: &Vec<Pokemon>) -> Element<'static, Message> {
    let label = pc_box_label(num + 1);
    let mut col = iced::widget::Column::new()
        .align_items(Alignment::Center)
        .spacing(5);

    for row in pc_box.chunks(6) {
        let mut pc_row = iced::widget::Row::new().spacing(5);
        for slot in row {
            pc_row = pc_row.push(pc_slot(id, slot));
        }
        col = col.push(pc_row);
    }

    let left_handle =
        image::Handle::from_memory(PROJECT_DIR.get_file("left-arrow.png").unwrap().contents());

    let right_handle =
        image::Handle::from_memory(PROJECT_DIR.get_file("right-arrow.png").unwrap().contents());

    container(
        column![
            row![
                button(image(left_handle))
                    .on_press(Message::Decrement)
                    .width(40)
                    .height(40),
                label,
                button(image(right_handle))
                    .on_press(Message::Increment)
                    .width(40)
                    .height(40)
            ]
            .spacing(5),
            col
        ]
        .align_items(Alignment::Center)
        .spacing(10),
    )
    .into()
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

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<Vec<u8>>), Error> {
    let contents = tokio::fs::read(&path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)?;

    Ok((path, contents))
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

fn stat_bar() -> StatBar {
    StatBar {}
}

struct StatBar {
    /*size: f32,
    radius: [f32; 4],
    border_width: f32,
    shadow: Shadow,*/
}

/*impl StatBar {
    pub fn new(size: f32, radius: [f32; 4], border_width: f32, shadow: Shadow) -> Self {
        Self {
            size,
            radius,
            border_width,
            shadow,
        }
    }
}*/

impl<Message, Renderer> Widget<Message, Theme, Renderer> for StatBar
where
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(150.0, 15.0))
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::from_rgb(0.0, 0.2, 0.4),
        );
    }
}

impl<'a, Message, Renderer> From<StatBar> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn from(widget: StatBar) -> Self {
        Self::new(widget)
    }
}
