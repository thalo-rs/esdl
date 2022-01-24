use thiserror::Error;

/// Error type.
#[derive(Debug, Error)]
pub enum Error {
    /// IO error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Parse schema error.
    #[error("parse error: {0}")]
    Parse(String),
    /// Validate schema error.
    #[error("validation error: {0}")]
    Validation(#[from] crate::schema::Error),
}
