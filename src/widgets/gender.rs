use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::tree::Tree;
use iced::advanced::widget::{self, Widget};
use iced::border::Border;
use iced::widget::text;
use iced::{color, mouse};
use iced::{Background, Color, Element, Length, Rectangle, Shadow, Size, Theme};

use pk_edit::data_structure::pokemon;

pub fn gender<'a, Message: 'a, Theme, Renderer>(
    gender: pokemon::Gender,
) -> Gender<'a, Message, Theme, Renderer>
where
    Theme: Catalog + iced::widget::text::Catalog + 'a,
    Renderer: renderer::Renderer + iced::advanced::text::Renderer + 'a,
{
    Gender::new(gender)
}

pub struct Gender<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    width: f32,
    height: f32,
    gender: pokemon::Gender,
    class: Theme::Class<'a>,
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message: 'a, Theme, Renderer> Gender<'a, Message, Theme, Renderer>
where
    Theme: Catalog + iced::widget::text::Catalog + 'a,
    Renderer: renderer::Renderer + iced::advanced::text::Renderer + 'a,
{
    pub fn new(gender: pokemon::Gender) -> Self {
        let content = match gender {
            pokemon::Gender::None => text("").into(),
            pokemon::Gender::F => text("♀").shaping(text::Shaping::Advanced).into(),
            pokemon::Gender::M => text("♂").shaping(text::Shaping::Advanced).into(),
        };

        Self {
            gender,
            content,
            width: 26.0,
            height: 26.0,
            class: <Theme as Catalog>::default(),
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Gender<'a, Message, Theme, Renderer>
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
                .layout(&mut tree.children[0], renderer, &limits)
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
        let w_style = theme.style(&self.class);

        let background = match self.gender {
            pokemon::Gender::None => Some(Background::Color(Color::TRANSPARENT)),
            pokemon::Gender::F => w_style.female,
            pokemon::Gender::M => w_style.male,
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: w_style.border,
                shadow: w_style.shadow,
            },
            background.unwrap(),
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
}

impl<'a, Message, Theme, Renderer> From<Gender<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Message: 'a + Clone,
    Renderer: renderer::Renderer + 'a,
{
    fn from(gender: Gender<'a, Message, Theme, Renderer>) -> Self {
        Self::new(gender)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The text [`Color`] of the slot.
    pub text_color: Color,
    /// The [`Background`] of the slot.
    pub male: Option<Background>,
    pub female: Option<Background>,
    /// The [`Border`] of the slot.
    pub border: Border,
    /// The [`Shadow`] of the slot.
    pub shadow: Shadow,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            male: Some(iced::Background::Color(color!(0x4186d7))),
            female: Some(iced::Background::Color(color!(0xd65c63))),
            text_color: iced::Color::WHITE,
            border: iced::Border {
                radius: 20.0.into(),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            shadow: iced::Shadow::default(),
        }
    }
}

pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// A styling function for a [`Gender`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(_theme: &Theme) -> Style {
    Style::default()
}
