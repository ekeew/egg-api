use std::{env, fs};

use axum::{routing, Router};
use tokio::signal;
use tracing_subscriber::{prelude::*, filter::LevelFilter};

use super::{endpoints, errors::AppError};
use super::config::{self, Args, Config};

pub fn run_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(LevelFilter::INFO)
        .init();
}

pub fn read_config() -> Result<Config, AppError> {
    let args: Args = config::parse_args(env::args())?;
    let conf_file = fs::read_to_string(args.config).map_err(|_| AppError::ParsingError)?;
    config::parse_config(conf_file)
}

pub fn create_app() -> Router {
    Router::new()
        .route("/", routing::get(endpoints::root_handler()))
}

pub async fn get_shutdown_signal() {
    signal::ctrl_c().await.unwrap();
}
