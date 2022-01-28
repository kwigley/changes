use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("Entry serialization error")]
    Ser(serde_frontmatter::SerdeFMError),
    #[error("Unable to generate file name")]
    UnableToGenerateFilename,
    #[error("Template parse error:\n{0}")]
    TemplateParseError(String),
    /// Error that may occur while rendering the template.
    #[error("Template render error:\n{0}")]
    TemplateRenderError(String),
    /// Error that may occur during more general template operations.
    #[error("Template error: `{0}`")]
    TemplateError(#[from] tera::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
