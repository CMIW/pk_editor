use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::widget::Id;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::border::Border;
use iced::widget::image;
use iced::{color, event, mouse, touch};
use iced::{Background, Color, Element, Event, Length, Rectangle, Shadow, Size, Theme, Vector};

use pk_edit::Pokemon;

use crate::misc::PROJECT_DIR;
use crate::widgets::{gender, level};

pub fn party_slot<'a, Message: 'a + std::clone::Clone, Theme, Renderer>(
    pk_data: Option<&Pokemon>,
) -> Slot<'a, Message, Theme, Renderer>
where
    Theme: Catalog + gender::Catalog + level::Catalog + iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::image::Renderer + iced::advanced::text::Renderer + 'a,
    <Renderer as iced::advanced::image::Renderer>::Handle: From<iced::advanced::image::Handle>,
{
    Slot::new(pk_data)
}

pub struct Slot<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    id: Option<Id>,
    width: f32,
    height: f32,
    is_selected: bool,
    status: Option<Status>,
    class: Theme::Class<'a>,
    on_press: Option<OnPress<'a, Message>>,
    name: Element<'a, Message, Theme, Renderer>,
    level: Element<'a, Message, Theme, Renderer>,
    image: Element<'a, Message, Theme, Renderer>,
    gender: Element<'a, Message, Theme, Renderer>,
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<Message: Clone> OnPress<'_, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

impl<'a, Message: 'a + std::clone::Clone, Theme, Renderer> Slot<'a, Message, Theme, Renderer>
where
    Theme: Catalog + gender::Catalog + level::Catalog + iced::widget::text::Catalog + 'a,
    Renderer:
        renderer::Renderer + iced::advanced::text::Renderer + iced::advanced::image::Renderer + 'a,
{
    pub fn new(pk_data: Option<&Pokemon>) -> Self
    where
        <Renderer as iced::advanced::image::Renderer>::Handle: From<iced::advanced::image::Handle>,
    {
        let (name, level, image, gender) = if let Some(pokemon) = pk_data {
            let handle = iced::advanced::image::Handle::from_bytes(
                PROJECT_DIR
                    .get_file(format!(
                        "Pokemon/{:0width$}.png",
                        pokemon.nat_dex_number(),
                        width = 4
                    ))
                    .unwrap()
                    .contents(),
            );

            (
                iced::widget::text(pokemon.nickname()).into(),
                level(pokemon.level()).into(),
                image(handle).into(),
                gender(pokemon.gender()).into(),
            )
        } else {
            (
                iced::widget::Space::new(iced::Length::Shrink, iced::Length::Shrink).into(),
                iced::widget::Space::new(iced::Length::Shrink, iced::Length::Shrink).into(),
                iced::widget::Space::new(iced::Length::Shrink, iced::Length::Shrink).into(),
                iced::widget::Space::new(iced::Length::Shrink, iced::Length::Shrink).into(),
            )
        };

        Self {
            name,
            level,
            image,
            gender,
            id: None,
            width: 240.0,
            height: 80.0,
            on_press: None,
            is_selected: false,
            status: Some(Status::Idle),
            class: <Theme as Catalog>::default(),
        }
    }

    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    pub fn selected(mut self, id: &Option<Id>) -> Self {
        if self.id == *id && self.id.is_some() {
            self.is_selected = true;
            self.status = Some(Status::Selected);
        }
        self
    }

    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Slot<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Message: 'a + Clone,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![
            Tree::new(&self.image),
            Tree::new(&self.level),
            Tree::new(&self.name),
            Tree::new(&self.gender),
        ]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.image, &self.level, &self.name, &self.gender]);
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = Size::new(self.width, self.height);
        let mut image = self.image.as_widget().layout(
            &mut tree.children[0],
            renderer,
            &limits.width(self.height).height(self.height),
        );

        let mut level = self
            .level
            .as_widget()
            .layout(&mut tree.children[1], renderer, &limits);

        let mut name = self
            .name
            .as_widget()
            .layout(&mut tree.children[2], renderer, &limits);

        let mut gender = self
            .gender
            .as_widget()
            .layout(&mut tree.children[3], renderer, &limits);

        image = image.align(iced::Alignment::End, iced::Alignment::Center, size);

        let image_bounds = image.bounds();
        let image_position = image_bounds.position() + iced::Vector::new(-10.0, 0.0);

        image.move_to_mut(image_position);

        level = level.align(iced::Alignment::Start, iced::Alignment::Start, size);
        let level_size = level.size();
        let level_bounds = level.bounds();
        let level_position = level_bounds.position() + iced::Vector::new(20.0, 10.0);

        level.move_to_mut(level_position);

        name = name.align(iced::Alignment::Start, iced::Alignment::Start, size);

        let name_bounds = name.bounds();
        let name_position =
            name_bounds.position() + iced::Vector::new(20.0, 20.0 + level_size.height);

        name.move_to_mut(name_position);

        gender = gender.align(iced::Alignment::Start, iced::Alignment::Start, size);
        let gender_bounds = gender.bounds();
        let gender_position =
            gender_bounds.position() + iced::Vector::new(30.0 + level_size.width, 10.0);

        gender.move_to_mut(gender_position);

        layout::Node::with_children(size, vec![image, level, name, gender])
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let w_style = theme.style(&self.class, self.status.unwrap_or(Status::Idle));

        let mut children = layout.children();

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: w_style.border,
                shadow: w_style.shadow,
            },
            w_style.background.unwrap(),
        );

        self.image.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            children.next().unwrap(),
            cursor,
            viewport,
        );

        self.level.as_widget().draw(
            &tree.children[1],
            renderer,
            theme,
            style,
            children.next().unwrap(),
            cursor,
            viewport,
        );

        self.name.as_widget().draw(
            &tree.children[2],
            renderer,
            theme,
            style,
            children.next().unwrap(),
            cursor,
            viewport,
        );

        self.gender.as_widget().draw(
            &tree.children[3],
            renderer,
            theme,
            style,
            children.next().unwrap(),
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over && self.on_press.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: event::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::event::Status {
        use event::Status::*;

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let bounds = layout.bounds();

                if cursor.is_over(bounds) {
                    let state = tree.state.downcast_mut::<State>();

                    state.is_pressed = true;
                }

                Captured
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                if state.is_pressed {
                    state.is_pressed = false;

                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        if let Some(on_press) = &self.on_press {
                            shell.publish(on_press.get());
                        }
                    }
                }
                Captured
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_pressed = false;

                Captured
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let bounds = layout.bounds();
                if cursor.is_over(bounds) {
                    self.status = Some(Status::Hovered);
                } else if self.is_selected {
                    self.status = Some(Status::Selected);
                } else {
                    self.status = Some(Status::Idle);
                }

                Captured
            }
            _ => Ignored,
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Slot<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Message: 'a + Clone,
    Renderer: renderer::Renderer + 'a,
{
    fn from(slot: Slot<'a, Message, Theme, Renderer>) -> Self {
        Self::new(slot)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Idle,
    Hovered,
    Selected,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The text [`Color`] of the slot.
    pub text_color: Option<Color>,
    /// The [`Background`] of the slot.
    pub background: Option<Background>,
    /// The [`Border`] of the slot.
    pub border: Border,
    /// The [`Shadow`] of the slot.
    pub shadow: Shadow,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: Some(iced::Background::Color(color!(0x000000, 0.5))),
            text_color: None,
            border: Border {
                radius: 5.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(2.0, 2.0),
                blur_radius: 4.0,
            },
        }
    }
}

pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Button`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    match status {
        Status::Idle => Style { ..Style::default() },
        Status::Selected => Style {
            background: Some(Background::Color(palette.primary.strong.color)),
            ..Style::default()
        },
        Status::Hovered => Style {
            background: Some(Background::Color(palette.primary.base.color)),
            ..Style::default()
        },
    }
}
