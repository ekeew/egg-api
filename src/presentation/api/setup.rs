use axum::{routing, Router};
use tokio::signal;
use tracing_subscriber::{prelude::*, filter::LevelFilter};

use super::endpoints;

pub fn run_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(LevelFilter::INFO)
        .init();
}

pub fn create_app() -> Router {
    Router::new()
        .route("/", routing::get(endpoints::root()))
}

pub async fn shutdown_signal() {
    signal::ctrl_c().await.unwrap();
}
