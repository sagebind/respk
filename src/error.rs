//! Error definitions.
use rusqlite;


#[derive(Clone, Copy, Debug)]
pub enum Error {
    /// An error occurred while compressing a resource.
    CompressionError,

    /// An error occurred while decompressing a resource.
    DecompressionError,

    /// A resource was not found for the given path name.
    ResourceNotFound,

    /// An internal bug or error within librespk.
    InternalError,
}

impl From<rusqlite::Error> for Error {
    fn from(_: rusqlite::Error) -> Error {
        Error::InternalError
    }
}

