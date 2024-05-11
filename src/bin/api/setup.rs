use std::{env, fs};

use axum::{routing, Router};
use tokio::signal;
use tracing_subscriber::{prelude::*, filter::LevelFilter};

use crate::{
    endpoints,
    errors::AppError,
    config::{self, Config},
};

pub fn run_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(LevelFilter::INFO)
        .init();
}

pub fn read_config() -> Result<Config, AppError> {
    let args = config::parse_args(env::args());
    let conf_file = fs::read_to_string(args.config)?;
    config::parse_config(conf_file)
}

pub fn create_app() -> Router {
    Router::new()
        .route("/", routing::get(endpoints::root_handler()))
}

pub async fn get_shutdown_signal() {
    signal::ctrl_c().await.unwrap();
}
