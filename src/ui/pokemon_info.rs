use iced::color;
use iced::widget::text_input;
use iced::widget::{column, container, image, mouse_area, pick_list, row, text};
use iced::{Alignment, Border, Element, Length, Shadow};

use crate::message::Message;
use crate::misc::{PROJECT_DIR, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::theme::{info_label_appearance, pokemon_info_appearance};
use crate::widgets::{gender, input_level};

use pk_edit::data_structure::pokemon::{
    items, species_list, transpose_item, Pokemon, Pokerus, Stats, NATURE,
};

use crate::slots::move_slot;
use iced::advanced::layout;
use iced::advanced::mouse;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::Layout;
use iced::advanced::Widget;
use iced::Rectangle;
use iced::Size;
use iced::Theme;

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

    let pokerus = mouse_area(pokerus).on_press(Message::ChangePokerusStatus);

    let name = text(pokemon.nickname());

    let row = row![
        pokeball,
        name,
        iced::widget::Space::with_width(Length::Fill),
        pokerus,
        iced::widget::Space::with_width(25),
        input_level(pokemon.level()),
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

fn stats(stats: Stats, level: u8) -> Element<'static, Message> {
    let (_, highest_stat) = stats.highest_stat(level);
    let scale = if highest_stat >= 400 { 0.23 } else { 0.45 };
    let hp = stats.hp(level);
    let attack = stats.attack(level);
    let defense = stats.defense(level);
    let sp_attack = stats.sp_attack(level);
    let sp_defense = stats.sp_defense(level);
    let speed = stats.speed(level);
    let labels_width = 70;
    let ev_iv_width = 30;

    let column = column![
        row![
            iced::widget::Space::with_width(Length::Fill),
            text("EV")
                .width(ev_iv_width)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            text("IV")
                .width(ev_iv_width)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            text("HP:").width(labels_width),
            stat_bar(hp as f32 * scale),
            text(hp),
            iced::widget::Space::with_width(Length::Fill),
            text_input(&stats.hp_ev.to_string(), &stats.hp_ev.to_string())
                .on_input(Message::HPEVChanged)
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(35)
                .size(12),
            text(stats.hp_iv)
                .width(ev_iv_width)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            text("Attack:").width(labels_width),
            stat_bar(attack as f32 * scale),
            text(attack),
            iced::widget::Space::with_width(Length::Fill),
            text_input(&stats.attack_ev.to_string(), &stats.attack_ev.to_string())
                .on_input(Message::AttackEVChanged)
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(35)
                .size(12),
            text(stats.attack_iv)
                .width(ev_iv_width)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            text("Defense:").width(labels_width),
            stat_bar(defense as f32 * scale),
            text(defense),
            iced::widget::Space::with_width(Length::Fill),
            text_input(&stats.defense_ev.to_string(), &stats.defense_ev.to_string())
                .on_input(Message::DefenseEVChanged)
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(35)
                .size(12),
            text(stats.defense_iv)
                .width(ev_iv_width)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            text("Sp. Atk:").width(labels_width),
            stat_bar(sp_attack as f32 * scale),
            text(sp_attack),
            iced::widget::Space::with_width(Length::Fill),
            text_input(&stats.sp_attack_ev.to_string(), &stats.sp_attack_ev.to_string())
                .on_input(Message::SpAtkEVChanged)
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(35)
                .size(12),
            text(stats.sp_attack_iv)
                .width(ev_iv_width)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            text("Sp. Def:").width(labels_width),
            stat_bar(sp_defense as f32 * scale),
            text(sp_defense),
            iced::widget::Space::with_width(Length::Fill),
            text_input(&stats.sp_defense_ev.to_string(), &stats.sp_defense_ev.to_string())
                .on_input(Message::SpDefEVChanged)
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(35)
                .size(12),
            text(stats.sp_defense_iv)
                .width(ev_iv_width)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            text("Speed:").width(labels_width),
            stat_bar(speed as f32 * scale),
            text(speed),
            iced::widget::Space::with_width(Length::Fill),
            text_input(&stats.speed_ev.to_string(), &stats.speed_ev.to_string())
                .on_input(Message::SpeedEVChanged)
                .line_height(text::LineHeight::Absolute(10.into()))
                .width(35)
                .size(12),
            text(stats.speed_iv)
                .width(ev_iv_width)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(5)
        .align_items(Alignment::Center),
    ]
    .padding([0, 10]) // top/bottom left/right
    .spacing(5);

    container(column).width(WINDOW_WIDTH * 0.33).into()
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

pub fn pokemon_info(pokemon: &Pokemon) -> Element<'static, Message> {
    let label = info_label(pokemon);

    let dex_species_lang = row![
        text(format!("No. {}", pokemon.nat_dex_number())),
        //text(pokemon.species()),
        pick_list(
            species_list(),
            Some(pokemon.species()),
            Message::SpeciesSelected
        )
        .width(130)
        .text_line_height(text::LineHeight::Absolute(10.into())),
        iced::widget::Space::with_width(Length::Fill),
        text(pokemon.language()),
        iced::widget::Space::with_width(45),
    ]
    .spacing(20)
    .align_items(Alignment::Center)
    .padding([5, 10, 5, 15]); // top, right, bottom, left

    let pid_friendship = container(row![
        row![
            text("PID").style(iced::theme::Text::Color(color!(0xffcc00))),
            iced::widget::Space::with_width(30),
            text(format!("{:X}", pokemon.personality_value()))
        ]
        .width((WINDOW_WIDTH * 0.33) / 2.0)
        .align_items(Alignment::Center),
        row![
            text("Friendship").style(iced::theme::Text::Color(color!(0xffcc00))),
            iced::widget::Space::with_width(Length::Fill),
            text_input(
                &pokemon.friendship().to_string(),
                &pokemon.friendship().to_string()
            )
            .on_input(Message::FriendshipChanged)
            .line_height(text::LineHeight::Absolute(10.into()))
            .size(12),
        ]
        .width((WINDOW_WIDTH * 0.33) / 2.0),
    ])
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15]);

    let nature_ability = container(row![
        row![
            text("Nature").style(iced::theme::Text::Color(color!(0xffcc00))),
            iced::widget::Space::with_width(10),
            //text(pokemon.nature()),
            pick_list(
                NATURE.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                Some(pokemon.nature()),
                Message::NatureSelected
            )
            .width(100)
            .text_line_height(text::LineHeight::Absolute(10.into())),
        ]
        .width((WINDOW_WIDTH * 0.33) / 2.0),
        row![
            text("Ability").style(iced::theme::Text::Color(color!(0xffcc00))),
            iced::widget::Space::with_width(15),
            text(pokemon.ability()),
        ]
        .width((WINDOW_WIDTH * 0.33) / 2.0),
    ])
    .width(WINDOW_WIDTH * 0.33)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .align_x(iced::alignment::Horizontal::Left)
    .padding([5, 15])
    .style(pokemon_info_appearance());

    let item_image = if let Some(item_index) = transpose_item(&pokemon.held_item()) {
        if let Some(item_image) =
            PROJECT_DIR.get_file(format!("Items/item_{:0width$}.png", item_index, width = 4))
        {
            let handle = image::Handle::from_memory(item_image.contents());
            image(handle).height(25)
        } else {
            image("").height(30)
        }
    } else {
        image("").height(30)
    };

    let item = container(
        row![
            text("Held Item").style(iced::theme::Text::Color(color!(0xffcc00))),
            iced::widget::Space::with_width(5),
            item_image,
            iced::widget::Space::with_width(5),
            pick_list(
                items(),
                Some(pokemon.held_item()),
                Message::HeldItemSelected
            )
            .width(150)
            .text_line_height(text::LineHeight::Absolute(10.into())),
            iced::widget::Space::with_width(Length::Fill),
        ]
        .align_items(Alignment::Center),
    )
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
        stats(pokemon.stats(), pokemon.level()),
        iced::widget::Space::with_height(Length::Fill),
        pid_friendship,
        nature_ability,
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

fn stat_bar(width: f32) -> StatBar {
    StatBar::new(width)
}

struct StatBar {
    size: iced::Size,
}

impl StatBar {
    pub fn new(width: f32) -> Self {
        Self {
            size: Size::new(width, 15.0),
        }
    }
}

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
        layout::Node::new(self.size)
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
        let color = match self.size.width {
            val if val >= 150.0 => color!(0x00c2b8),
            val if val >= 120.0 => color!(0x23cd5e),
            val if val >= 90.0 => color!(0xa0e515),
            val if val >= 60.0 => color!(0xffdd57),
            _ => color!(0xff7f0f),
        };

        let border_color = match self.size.width {
            val if val >= 150.0 => color!(0x00a59d),
            val if val >= 120.0 => color!(0x1eaf50),
            val if val >= 90.0 => color!(0x88c312),
            val if val >= 60.0 => color!(0xd9bc4a),
            _ => color!(0xd96c0d),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            color,
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
