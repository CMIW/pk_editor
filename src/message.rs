use iced::advanced::widget::Id;

use std::path::PathBuf;
use std::sync::Arc;

use pk_edit::data_structure::pokemon::Pokemon;
use pk_edit::StorageType;

use crate::bag;
use crate::error::Error;
use crate::menu_bar;
use crate::pokemon_info;

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    HideModal,
    ShowModal,
    UpdateChanges,
    Bag(bag::Message),
    MenuBar(menu_bar::Message),
    PokemonInfo(pokemon_info::Message),
    Loaded(Result<(), String>),
    WriteFile(Result<(), Error>),
    FileSaved(Result<PathBuf, Error>),
    FileOpened(Result<PathBuf, Error>),
    LoadFile(Result<Arc<Vec<u8>>, Error>),
    Selected(Option<Id>, Option<StorageType>, Option<Pokemon>),
}
