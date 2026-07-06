//! Unified slot widget for party and PC box Pokémon.
//!
//! Provides two constructor functions that share one [`Slot`] implementation:
//! - [`pc_slot`] — 80 × 80, displays the Pokémon sprite only.
//! - [`party_slot`] — 240 × 80, displays sprite, level badge, nickname, and gender badge.
//!
//! Interaction (hover, press, select, drag) and styling are implemented once.
//! Layout and draw branch on [`SlotKind`].

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::widget::Id;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::border::Border;
use iced::widget::image;
use iced::window;
use iced::Point;
use iced::{color, event, mouse, touch};
use iced::{Background, Color, Element, Event, Length, Rectangle, Shadow, Size, Theme, Vector};

use pk_edit::{AnyPokemon, PokemonTrait};

use crate::widgets::{gender, level};

// ── Kind ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotKind {
    Pc,
    Party,
}

// ── Constructors ─────────────────────────────────────────────────────────────

/// Creates an 80 × 80 PC box slot widget.
pub fn pc_slot<'a, Message: 'a + Clone, Theme, Renderer>(
    handle: Option<iced::advanced::image::Handle>,
) -> Slot<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + iced::advanced::image::Renderer + 'a,
    <Renderer as iced::advanced::image::Renderer>::Handle: From<iced::advanced::image::Handle>,
{
    let content: Element<'a, Message, Theme, Renderer> = if let Some(h) = handle {
        image(h).into()
    } else {
        iced::widget::Space::new().into()
    };

    Slot::new(SlotKind::Pc, vec![content], 80.0, 80.0)
}

/// Creates a 240 × 80 party slot widget.
pub fn party_slot<'a, Message: 'a + Clone, Theme, Renderer>(
    pk_data: Option<&AnyPokemon>,
    handle: Option<iced::advanced::image::Handle>,
) -> Slot<'a, Message, Theme, Renderer>
where
    Theme: Catalog + gender::Catalog + level::Catalog + iced::widget::text::Catalog + 'a,
    Renderer:
        renderer::Renderer + iced::advanced::image::Renderer + iced::advanced::text::Renderer + 'a,
    <Renderer as iced::advanced::image::Renderer>::Handle: From<iced::advanced::image::Handle>,
{
    let children: Vec<Element<'a, Message, Theme, Renderer>> = if let Some(pokemon) = pk_data {
        let img: Element<'a, Message, Theme, Renderer> = if let Some(h) = handle {
            image(h).into()
        } else {
            iced::widget::Space::new().into()
        };
        vec![
            img,
            level::level(pokemon.level()).into(),
            iced::widget::text(pokemon.nickname()).into(),
            gender::gender(pokemon.gender()).into(),
        ]
    } else {
        vec![
            iced::widget::Space::new().into(),
            iced::widget::Space::new().into(),
            iced::widget::Space::new().into(),
            iced::widget::Space::new().into(),
        ]
    };

    Slot::new(SlotKind::Party, children, 240.0, 80.0)
}

// ── Widget ───────────────────────────────────────────────────────────────────

/// A slot widget for a Pokémon, used in both the party panel and PC box grid.
pub struct Slot<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    kind: SlotKind,
    id: Option<Id>,
    width: f32,
    height: f32,
    scale: f32,
    is_selected: bool,
    status: Option<Status>,
    class: Theme::Class<'a>,
    on_press: Option<OnPress<'a, Message>>,
    on_drop: Option<Message>,
    on_drag_start: Option<Box<dyn Fn(Point) -> Message + 'a>>,
    in_drag_mode: bool,
    is_drag_source: bool,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
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

impl<'a, Message: Clone + 'a, Theme, Renderer> Slot<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn new(
        kind: SlotKind,
        children: Vec<Element<'a, Message, Theme, Renderer>>,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            kind,
            children,
            id: None,
            width,
            height,
            scale: 1.0,
            on_press: None,
            on_drop: None,
            on_drag_start: None,
            in_drag_mode: false,
            is_drag_source: false,
            is_selected: false,
            status: Some(Status::Idle),
            class: <Theme as Catalog>::default(),
        }
    }

    /// Set a uniform scale for this slot (1.0 = default size)
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale.max(0.0);
        self
    }

    /// Sets the widget ID, used for selection tracking.
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    /// Marks this slot as selected if its ID matches the provided ID.
    pub fn selected(mut self, id: &Option<Id>) -> Self {
        if self.id == *id && self.id.is_some() {
            self.is_selected = true;
            self.status = Some(Status::Selected);
        }
        self
    }

    /// Sets a message to publish when the slot is clicked.
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    /// Sets a closure to call when the slot is clicked.
    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }

    /// Tells the slot that a drag is currently in progress globally.
    /// Gates [`Status::DropTarget`] highlighting on hover.
    pub fn in_drag_mode(mut self, v: bool) -> Self {
        self.in_drag_mode = v;
        self
    }

    /// Marks this slot as the drag source, rendering it dimmed.
    pub fn is_drag_source(mut self, v: bool) -> Self {
        self.is_drag_source = v;
        self
    }

    /// Sets a closure that receives the screen-space origin [`Point`] and
    /// returns the message to publish when the drag threshold is crossed.
    pub fn on_drag_start(mut self, f: impl Fn(Point) -> Message + 'a) -> Self {
        self.on_drag_start = Some(Box::new(f));
        self
    }

    /// Sets the message to publish when another slot is dropped onto this one.
    pub fn on_drop(mut self, msg: Message) -> Self {
        self.on_drop = Some(msg);
        self
    }
}

// ── State ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct State {
    is_pressed: bool,
    press_pos: Option<Point>,
    drag_fired: bool,
}

// ── Widget impl ──────────────────────────────────────────────────────────────

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Slot<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Message: 'a + Clone,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: (self.width * self.scale).into(),
            height: (self.height * self.scale).into(),
        }
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // current base size times scale
        let w = self.width * self.scale;
        let h = self.height * self.scale;
        let size = Size::new(w, h);

        match self.kind {
            SlotKind::Pc => layout::Node::with_children(
                size,
                vec![self.children[0]
                    .as_widget_mut()
                    .layout(
                        &mut tree.children[0],
                        renderer,
                        &limits
                            .width(w - 10.0 * self.scale)
                            .height(h - 10.0 * self.scale),
                    )
                    .align(iced::Alignment::Center, iced::Alignment::Center, size)],
            ),
            SlotKind::Party => {
                let img_size = h;
                let mut img = self.children[0].as_widget_mut().layout(
                    &mut tree.children[0],
                    renderer,
                    &limits.width(img_size).height(img_size),
                );
                let mut lv = self.children[1].as_widget_mut().layout(
                    &mut tree.children[1],
                    renderer,
                    limits,
                );
                let mut name = self.children[2].as_widget_mut().layout(
                    &mut tree.children[2],
                    renderer,
                    limits,
                );
                let mut gnd = self.children[3].as_widget_mut().layout(
                    &mut tree.children[3],
                    renderer,
                    limits,
                );

                img = img.align(iced::Alignment::End, iced::Alignment::Center, size);
                img.move_to_mut(img.bounds().position() + Vector::new(-10.0 * self.scale, 0.0));

                lv = lv.align(iced::Alignment::Start, iced::Alignment::Start, size);
                let lv_size = lv.size();
                lv.move_to_mut(
                    lv.bounds().position() + Vector::new(20.0 * self.scale, 10.0 * self.scale),
                );

                name = name.align(iced::Alignment::Start, iced::Alignment::Start, size);
                name.move_to_mut(
                    name.bounds().position()
                        + Vector::new(20.0 * self.scale, 20.0 * self.scale + lv_size.height),
                );

                gnd = gnd.align(iced::Alignment::Start, iced::Alignment::Start, size);
                gnd.move_to_mut(
                    gnd.bounds().position()
                        + Vector::new(30.0 * self.scale + lv_size.width, 10.0 * self.scale),
                );

                layout::Node::with_children(size, vec![img, lv, name, gnd])
            }
        }
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
        let w_style = theme.style(&self.class, self.status.unwrap_or(Status::Idle));

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: w_style.border,
                shadow: w_style.shadow,
                ..Default::default()
            },
            w_style.background.unwrap(),
        );

        let child_count = match self.kind {
            SlotKind::Pc => 1,
            SlotKind::Party => 4,
        };

        let mut child_layouts = layout.children();
        for i in 0..child_count {
            if let Some(child) = self.children.get(i) {
                if let Some(tree_child) = tree.children.get(i) {
                    child.as_widget().draw(
                        tree_child,
                        renderer,
                        theme,
                        style,
                        child_layouts.next().unwrap(),
                        cursor,
                        viewport,
                    );
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) && self.on_press.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &event::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() && cursor.is_over(layout.bounds()) {
                    let state = tree.state.downcast_mut::<State>();
                    state.is_pressed = true;
                    state.press_pos = cursor.position();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                let state = tree.state.downcast_mut::<State>();
                if self.in_drag_mode {
                    if cursor.is_over(layout.bounds()) {
                        if let Some(msg) = self.on_drop.clone() {
                            shell.publish(msg);
                        }
                    }
                    state.is_pressed = false;
                    state.press_pos = None;
                    state.drag_fired = false;
                } else if let Some(on_press) = self.on_press.as_ref().map(OnPress::get) {
                    if state.is_pressed {
                        state.is_pressed = false;
                        state.press_pos = None;
                        state.drag_fired = false;
                        if cursor.is_over(layout.bounds()) {
                            shell.publish(on_press);
                        }
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();
                state.is_pressed = false;
                state.press_pos = None;
                state.drag_fired = false;
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let state = tree.state.downcast_mut::<State>();
                if state.is_pressed && !state.drag_fired {
                    if let Some(origin) = state.press_pos {
                        let dx = position.x - origin.x;
                        let dy = position.y - origin.y;
                        if (dx * dx + dy * dy).sqrt() > 5.0 {
                            state.drag_fired = true;
                            if let Some(f) = &self.on_drag_start {
                                shell.publish(f(origin));
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        let current_status = if self.is_drag_source {
            Status::Dragging
        } else if self.in_drag_mode && cursor.is_over(layout.bounds()) {
            Status::DropTarget
        } else if cursor.is_over(layout.bounds()) {
            let state = tree.state.downcast_ref::<State>();
            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else if self.is_selected {
            Status::Selected
        } else {
            Status::Idle
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.status = Some(current_status);
        } else if self.status.is_some_and(|s| s != current_status) {
            self.status = Some(current_status);
            shell.request_redraw();
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

// ── Status ───────────────────────────────────────────────────────────────────

/// The visual state of a [`Slot`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Default state.
    Idle,
    /// Cursor is over the slot.
    Hovered,
    /// Slot is being pressed.
    Pressed,
    /// Slot is the currently selected Pokémon.
    Selected,
    /// Slot is the source of an in-progress drag — rendered dimmed.
    Dragging,
    /// A drag is in progress and the cursor is over this slot — rendered highlighted.
    DropTarget,
}

// ── Style ────────────────────────────────────────────────────────────────────

/// Visual style of a [`Slot`].
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
            background: Some(Background::Color(color!(0x000000, 0.5))),
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

// ── Catalog ──────────────────────────────────────────────────────────────────

/// Styling catalog for [`Slot`].
pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given [`Status`].
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Slot`].
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

/// Default [`Slot`] style using the active [`Theme`] palette.
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    match status {
        Status::Idle | Status::Pressed => Style::default(),
        Status::Selected => Style {
            background: Some(Background::Color(palette.primary.strong.color)),
            ..Style::default()
        },
        Status::Hovered => Style {
            background: Some(Background::Color(palette.primary.base.color)),
            ..Style::default()
        },
        Status::Dragging => Style {
            background: Some(Background::Color(Color {
                a: 0.2,
                ..Color::BLACK
            })),
            ..Style::default()
        },
        Status::DropTarget => Style {
            background: Some(Background::Color(palette.success.base.color)),
            border: Border {
                width: 2.0,
                color: palette.success.strong.color,
                radius: 5.0.into(),
            },
            ..Style::default()
        },
    }
}
