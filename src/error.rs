use rusqlite;


#[derive(Clone, Copy, Debug)]
pub enum Error {
    CompressionError,
    DecompressionError,
    ResourceNotFound,

    /// An internal bug or error within librespk.
    InternalError,
}

impl From<rusqlite::Error> for Error {
    fn from(_: rusqlite::Error) -> Error {
        Error::InternalError
    }
}

