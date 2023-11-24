//! When working with BESS goes wrong.

use thiserror::Error;

use crate::block::Ident;

/// A specialized [`Result`] type for BESS operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The error type for BESS operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Message(String),
    #[error("required block is missing: `{0}`")]
    Required(Ident),
    #[error("buffer is too large")]
    TooLarge,
    #[error("buffer is too short")]
    TooShort,
    #[error("unsupported type")]
    Unsupported,
}

#[cfg(feature = "serde")]
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}
