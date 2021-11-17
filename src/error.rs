use std::time::SystemTimeError;

use serde_frontmatter::SerdeFMError;
use thiserror::Error as ThisError;

/// Library related errors that we are exposing to the rest of the workspaces.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur while I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("Input error: `{0}`")]
    UserInputError(String),
    #[error("Entry serialization error")]
    SerError(SerdeFMError),
    #[error("System time error: `{0}`")]
    SystemTimeError(SystemTimeError),
    #[error("Unable to generate file name")]
    UnableToGenerateFilenameError,
}

/// Result type of the core library.
pub type Result<T> = core::result::Result<T, Error>;
