use std::fmt::Display;

use std::io::Error as IoError;
use toml::de::Error as TomlError;

pub enum AppError {
    ConfigError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ConfigError(e) => writeln!(f, "config error: {e}"),
        }
    }
}

trait ConfigError: ToString {}
impl ConfigError for IoError {}
impl ConfigError for TomlError {}

impl<T: ConfigError> From<T> for AppError {
    fn from(value: T) -> Self {
        AppError::ConfigError(value.to_string())
    }
}
