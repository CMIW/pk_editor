use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::Widget;
use iced::{color, mouse};
use iced::{Border, Color, Element, Length, Rectangle, Shadow, Size, Theme};

pub fn stat_bar<'a, Theme>(width: f32) -> StatBar<'a, Theme>
where
    Theme: Catalog + 'a,
{
    StatBar::new(width)
}

pub struct StatBar<'a, Theme>
where
    Theme: Catalog,
{
    size: iced::Size,
    class: Theme::Class<'a>,
}

impl<'a, Theme> StatBar<'a, Theme>
where
    Theme: Catalog + 'a,
{
    pub fn new(width: f32) -> Self {
        Self {
            size: Size::new(width, 15.0),
            class: <Theme as Catalog>::default(),
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for StatBar<'a, Theme>
where
    Theme: Catalog + 'a,
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.size.width.into(),
            height: self.size.height.into(),
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
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let style = theme.style(&self.class);

        let (color, border_color) = match self.size.width {
            val if val >= 150.0 => (style.phenomenal.background, style.phenomenal.border),
            val if val >= 120.0 => (style.very_good.background, style.very_good.border),
            val if val >= 90.0 => (style.good.background, style.good.border),
            val if val >= 60.0 => (style.mediocre.background, style.mediocre.border),
            val if val >= 30.0 => (style.bad.background, style.bad.border),
            _ => (style.very_bad.background, style.very_bad.border),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: border_color,
                    width: 2.0,
                    radius: 5.0.into(),
                },
                shadow: Shadow::default(),
            },
            color,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<StatBar<'a, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: iced::advanced::Renderer,
{
    fn from(widget: StatBar<'a, Theme>) -> Self {
        Self::new(widget)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub phenomenal: Pair,

    pub very_good: Pair,

    pub good: Pair,

    pub mediocre: Pair,

    pub bad: Pair,

    pub very_bad: Pair,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair {
    pub background: Color,
    pub border: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            phenomenal: Pair {
                background: color!(0x00c2b8),
                border: color!(0x00a59d),
            },
            very_good: Pair {
                background: color!(0x23cd5e),
                border: color!(0x1eaf50),
            },
            good: Pair {
                background: color!(0xa0e515),
                border: color!(0x88c312),
            },
            mediocre: Pair {
                background: color!(0xffdd57),
                border: color!(0xd9bc4a),
            },
            bad: Pair {
                background: color!(0xff7f0f),
                border: color!(0xd96c0d),
            },
            very_bad: Pair {
                background: color!(0xf34444),
                border: color!(0xcf3a3a),
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
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// A styling function for a [`Button`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_| Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}
