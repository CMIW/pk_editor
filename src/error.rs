use std::io;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("File select dialog closed")]
    DialogClosed,
    #[error("IO Error")]
    IO(io::ErrorKind),
    #[error("No file has been opened, first open a file the you can save it")]
    NoFileOpened,
}
