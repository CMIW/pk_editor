use iced::font;

use std::path::PathBuf;
use std::sync::Arc;

use pk_edit::data_structure::pokemon::Pokemon;
use pk_edit::StorageType;

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile,
    SaveFile,
    FileOpened(Result<PathBuf, Error>),
    FileSaved(Result<PathBuf, Error>),
    LoadFile(Result<Arc<Vec<u8>>, Error>),
    WriteFile(Result<(), Error>),
    UpdateChanges,
    SelectedPokemon((StorageType, Pokemon)),
    Increment,
    Decrement,
    ChangePokerusStatus,
    FriendshipIncrement,
    FriendshipDecrement,
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
    SpeciesSelected(String),
    HeldItemSelected(String),
    FriendshipChanged(String),
}
