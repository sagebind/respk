#[derive(Debug)]
pub enum Error {
    CompressionError,
    DecompressionError,
    SQLError,
    ResourceNotFound,
}
