use std::{fmt::Display, io::Error as IoError};
use toml::de::Error as TomlError;

pub enum AppError {
    ParsingError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ParsingError(e) => writeln!(f, "{e}")
        }
    }
}

trait ParsingError: ToString {}
impl ParsingError for TomlError {}
impl ParsingError for IoError {}

impl<T: ParsingError> From<T> for AppError {
    fn from(value: T) -> Self {
        AppError::ParsingError(value.to_string())
    }
}
