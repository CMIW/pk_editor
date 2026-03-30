//! Root message type for the `pk_editor` application.
//!
//! All events produced by the UI or async tasks flow through [`Message`],
//! which is processed by [`crate::State::update`].

use iced::advanced::widget::Id;
use iced::widget::image;
use iced::Point;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use pk_edit::pokemon::Pokemon;
use pk_edit::save::storage::StorageType;

use crate::bag;
use crate::error::Error;
use crate::menu_bar;
use crate::pokemon_info;

/// The root message enum for all UI events and async task results.
#[derive(Debug, Clone)]
pub enum Message {
    /// Advance to the next PC box.
    Increment,
    /// Go back to the previous PC box.
    Decrement,
    /// Dismiss the loading/blocking modal overlay.
    HideModal,
    /// Show the loading/blocking modal overlay.
    ShowModal,
    /// Flush the currently selected Pokémon back into the save file and refresh all UI state.
    UpdateChanges,
    /// Delegate a bag-screen event to [`bag::update`].
    Bag(bag::Message),
    /// Delegate a menu-bar event (open, save, tab switch) to the menu bar handler.
    MenuBar(menu_bar::Message),
    /// Delegate a Pokémon-info panel event to [`pokemon_info::update`].
    PokemonInfo(pokemon_info::Message),
    /// Result of an initial data load (currently unused / reserved).
    Loaded(Result<(), String>),
    /// Result of writing save data to disk.
    WriteFile(Result<(), Error>),
    /// Result of the save-file dialog (path chosen by the user).
    FileSaved(Result<PathBuf, Error>),
    /// Result of the open-file dialog (path chosen by the user).
    FileOpened(Result<PathBuf, Error>),
    /// Result of reading the raw bytes of a save file from disk.
    LoadFile(Result<Arc<Vec<u8>>, Error>),
    /// A Pokémon slot was selected. Carries the widget [`Id`], [`StorageType`], and the [`Pokemon`].
    Selected(Option<Id>, Option<StorageType>, Option<Pokemon>),
    /// Result of loading all sprite and icon images from the embedded asset directory.
    ImagesListed(Result<HashMap<String, image::Handle>, Error>),
    DragStart(usize, StorageType, Point, u16, usize),
    DragMoved(Point),
    DragReleased,
    DragDrop(usize, StorageType, usize),
}
