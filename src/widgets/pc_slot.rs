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

use crate::misc::PROJECT_DIR;

pub fn pc_slot<'a, Message: 'a, Theme, Renderer>(
    dex_num: Option<u16>,
) -> Slot<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + iced::advanced::image::Renderer + 'a,
    <Renderer as iced::advanced::image::Renderer>::Handle: From<iced::advanced::image::Handle>,
{
    Slot::new(dex_num)
}

pub struct Slot<'a, Message, Theme, Renderer>
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
    content: Element<'a, Message, Theme, Renderer>,
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

impl<'a, Message: 'a, Theme, Renderer> Slot<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + iced::advanced::image::Renderer + 'a,
{
    pub fn new(dex_num: Option<u16>) -> Self
    where
        <Renderer as iced::advanced::image::Renderer>::Handle: From<iced::advanced::image::Handle>,
    {
        let content = if let Some(num) = dex_num {
            let handle = iced::advanced::image::Handle::from_bytes(
                PROJECT_DIR
                    .get_file(format!("Pokemon/{:0width$}.png", num, width = 4))
                    .unwrap()
                    .contents(),
            );

            image(handle).into()
        } else {
            iced::widget::Space::new(iced::Length::Shrink, iced::Length::Shrink).into()
        };

        Self {
            content,
            id: None,
            width: 80.0,
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
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = Size::new(self.width, self.height);
        layout::Node::with_children(
            size,
            vec![self
                .content
                .as_widget()
                .layout(
                    &mut tree.children[0],
                    renderer,
                    &limits.width(self.width - 10.0).height(self.height - 10.0),
                )
                .align(iced::Alignment::Center, iced::Alignment::Center, size)],
        )
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
        let bounds = layout.bounds();
        let is_mouse_over = cursor.is_over(bounds);

        let status = if self.on_press.is_none() {
            Status::Idle
        } else if is_mouse_over {
            let state = tree.state.downcast_ref::<State>();

            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else {
            Status::Idle
        };

        let w_style = theme.style(&self.class, status);

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: w_style.border,
                shadow: w_style.shadow,
            },
            w_style.background.unwrap(),
        );

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
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
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> iced::event::Status {
        if let iced::event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            return iced::event::Status::Captured;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        let state = tree.state.downcast_mut::<State>();

                        state.is_pressed = true;

                        return iced::event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = self.on_press.as_ref().map(OnPress::get) {
                    let state = tree.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press);
                        }

                        return iced::event::Status::Captured;
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            _ => {}
        }

        iced::event::Status::Ignored
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
    Pressed,
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
        Status::Idle | Status::Pressed => Style { ..Style::default() },
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
