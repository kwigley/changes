use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("Entry serialization error")]
    Ser(serde_frontmatter::SerdeFMError),
    #[error("Unable to generate file name")]
    UnableToGenerateFilename,
    #[error("Template parse error:\n{0}")]
    TemplateParseError(String),
    #[error("Template render error:\n{0}")]
    TemplateRenderError(String),
    #[error("Template error: `{0}`")]
    TemplateError(#[from] tera::Error),
    #[error("Invalid change type: `{0}`")]
    InvalidChangeType(String),
    #[error("Cannot parse config: `{0}`")]
    ConfigError(#[from] config::ConfigError),
    #[error("Unable to get input: `{0}`")]
    DialogueError(dialoguer::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
