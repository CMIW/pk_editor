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
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
    SpeciesSelected(String),
    HeldItemSelected(String),
    FriendshipChanged(String),
    LevelInputChanged(String),
    NatureSelected(String),
    IVChanged(String, String),
    EVChanged(String, String),
    MoveSelected(usize, String),
    PokemonView,
    BagView,
    AddMove(usize),
    ItemChanged(usize, String),
    ItemQuantityChanged(usize, String),
    BallChanged(usize, String),
    BallQuantityChanged(usize, String),
    BerryChanged(usize, String),
    BerryQuantityChanged(usize, String),
    TmChanged(usize, String),
    TmQuantityChanged(usize, String),
    KeyChanged(usize, String),
}
