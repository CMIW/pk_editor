use iced::advanced::widget::Id;
use iced::widget::{column, container, row, text, vertical_rule};
use iced::{Border, Color, Element, Padding};

use crate::message::Message;
use crate::misc::PROJECT_DIR;
use crate::widgets::party_slot;

use pk_edit::{Pokemon, StorageType};

pub fn party_label<'a>() -> Element<'a, Message> {
    let handle = iced::widget::svg::Handle::from_memory(
        PROJECT_DIR
            .get_file("icons/Pokeball_icon.svg")
            .unwrap()
            .contents(),
    );

    let image = container(
        iced::widget::svg(handle)
            .width(30.0)
            .height(30.0)
            .style(|theme, _| iced::widget::svg::Style {
                color: iced::widget::text::base(theme).color,
            }),
    )
    .align_x(iced::alignment::Horizontal::Right)
    .align_y(iced::alignment::Vertical::Center);

    let row = row![
        image,
        iced::widget::Space::new(15, 40),
        vertical_rule(1),
        iced::widget::Space::new(35, 40),
        text("Current Party")
    ]
    .width(240.0)
    .height(40.0)
    .align_y(iced::alignment::Vertical::Center)
    .padding(Padding {
        top: 5.0,
        right: 5.0,
        bottom: 5.0,
        left: 20.0,
    }); // top, right, bottom, left

    container(row)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .style(default)
        .into()
}

fn default(theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        //text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(iced::color!(0x000000, 0.5))),
        border: Border {
            radius: 20.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: iced::Shadow {
            color: Color::BLACK,
            offset: iced::Vector::new(2.0, 2.0),
            blur_radius: 4.0,
        },
        ..iced::widget::container::rounded_box(theme)
    }
}

pub fn party<'a>(selected: &Option<Id>, party: &[Pokemon]) -> Element<'a, Message> {
    let mut col = iced::widget::Column::new().spacing(10);

    for pokemon in party {
        if pokemon.is_empty() {
            col = col.push(party_slot(None).on_press(Message::Selected(
                Some(Id::new(pokemon.offset().to_string())),
                Some(StorageType::Party),
                Some(*pokemon),
            )));
        } else {
            col = col.push(
                party_slot(Some(&pokemon))
                    .id(Id::new(pokemon.offset().to_string()))
                    .selected(selected)
                    .on_press(Message::Selected(
                        Some(Id::new(pokemon.offset().to_string())),
                        Some(StorageType::Party),
                        Some(*pokemon),
                    )),
            );
        }
    }

    column![party_label(), col].spacing(15).into()
}
