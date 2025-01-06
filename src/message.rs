use iced::advanced::widget::Id;

use std::path::PathBuf;
use std::sync::Arc;

use pk_edit::data_structure::pokemon::Pokemon;
use pk_edit::StorageType;

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Message {
    BagView,
    OpenFile,
    SaveFile,
    Increment,
    Decrement,
    PokemonView,
    UpdateChanges,
    AddMove(usize),
    SelectedTab(Id),
    SelectedBag(Id),
    ChangePokerusStatus,
    NatureSelected(String),
    SpeciesSelected(String),
    TmChanged(usize, String),
    HeldItemSelected(String),
    FriendshipChanged(String),
    LevelInputChanged(String),
    IVChanged(String, String),
    EVChanged(String, String),
    KeyChanged(usize, String),
    Loaded(Result<(), String>),
    BallChanged(usize, String),
    ItemChanged(usize, String),
    MoveSelected(usize, String),
    BerryChanged(usize, String),
    WriteFile(Result<(), Error>),
    TmQuantityChanged(usize, String),
    FileSaved(Result<PathBuf, Error>),
    FileOpened(Result<PathBuf, Error>),
    ItemQuantityChanged(usize, String),
    BallQuantityChanged(usize, String),
    BerryQuantityChanged(usize, String),
    LoadFile(Result<Arc<Vec<u8>>, Error>),
    SelectedPokemon((StorageType, Pokemon)),
    Selected(Option<Id>, Option<StorageType>, Option<Pokemon>),
}
