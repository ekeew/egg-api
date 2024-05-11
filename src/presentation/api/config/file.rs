use std::net::SocketAddr;

use serde::Deserialize;

use crate::presentation::api::AppError;

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
}

#[derive(Deserialize)]
pub struct Server {
    pub addr: SocketAddr,
}

pub fn parse_config(config: String) -> Result<Config, AppError> {
    toml::from_str(&config).map_err(|e| e.into())
}
