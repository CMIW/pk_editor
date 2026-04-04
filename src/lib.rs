pub mod error;
pub mod icon;
pub mod message;
pub mod misc;
pub mod screen;
pub mod theme;
pub mod widgets;

pub use error::Error;
pub use message::Message;
pub use screen::*;
pub use theme::*;
pub use widgets::*;

use iced::widget::image;
use iced::Point;
use pk_edit::StorageType;

/// Holds the state of an in-progress drag operation.
#[derive(Debug)]
pub struct DragState {
    pub storage: StorageType,
    pub cursor: Point,
    pub handle: image::Handle,
    pub index: usize,
}
