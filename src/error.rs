use thiserror::Error as ThisError;

/// Library related errors that we are exposing to the rest of the workspaces.
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("Input error: `{0}`")]
    UserInput(String),
    #[error("Entry serialization error")]
    Ser(serde_frontmatter::SerdeFMError),
    #[error("System time error: `{0}`")]
    SystemTime(std::time::SystemTimeError),
    #[error("Unable to generate file name")]
    UnableToGenerateFilename,
}
