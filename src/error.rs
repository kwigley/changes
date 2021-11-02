use thiserror::Error as ThisError;

/// Library related errors that we are exposing to the rest of the workspaces.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur while I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("Input error: `{0}`")]
    UserInputError(String),
}

/// Result type of the core library.
pub type Result<T> = core::result::Result<T, Error>;
