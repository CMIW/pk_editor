//! Application-level error types for `pk_editor`.
//!
//! All fallible operations in the GUI layer return an [`Error`], which is then
//! surfaced to the user through a native message dialog (via [`rfd`]).

use std::io;
use thiserror::Error;

/// Errors that can occur during GUI-layer operations.
#[derive(Error, Debug, Clone)]
pub enum Error {
    /// The user closed the file dialog without selecting a file.
    #[error("File dialog closed")]
    DialogClosed,
    /// An I/O error occurred while reading or writing a file.
    #[error("IO Error{0}")]
    IO(io::ErrorKind),
    /// A save operation was attempted before any file was opened.
    #[error("No file has been opened, first open a file the you can save it")]
    NoFileOpened,
    /// The item index does not correspond to a known game item.
    #[error("{0} is not a valid item")]
    InvalidItem(usize),
    /// A required asset directory was not found inside the embedded assets.
    #[error("{0} does not exists")]
    MissingDirectory(String),
}
