use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub addr: SocketAddr,
}

pub fn parse_config(config: String) -> Config {
    toml::from_str(&config).unwrap()
}
