use serde_json::Error as SerdeError;
use std::io::Error as IoError;
use thiserror::Error;

/// The errors that this library can return.
#[derive(Debug, Error)]
pub enum Error {
    /// An error that happened during IO operations.
    #[error("io error")]
    Io(#[from] IoError),
    /// An error that happened during the parsing stage.
    #[error("failed to parse package.json file")]
    Parse(#[from] SerdeError),
}
