use std::net::SocketAddr;

use serde::Deserialize;

use crate::presentation::api::errors::AppError;

#[derive(Deserialize)]
pub struct Config {
    pub addr: SocketAddr,
}

pub fn parse_config(config: String) -> Result<Config, AppError> {
    toml::from_str(&config).map_err(|_| AppError::ParsingError)
}
