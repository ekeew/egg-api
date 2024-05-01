use std::process::ExitCode;

use axum::Router;
use tokio::{net::TcpListener, signal};
use tracing_subscriber::{prelude::*, filter::LevelFilter};

const ADDR: &str = "127.0.0.1:8080";

pub async fn api() -> ExitCode {
    setup_logging();

    tracing::info!("Initializing...");
    let app = create_app();

    tracing::info!("Starting server...");
    let result = serve(app).await;
    tracing::info!("Server stopped");

    if let Err(e) = result {
        tracing::error!("error: {e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn setup_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(LevelFilter::INFO)
        .init();
}

fn create_app() -> Router {
    Router::new()
}

async fn serve(app: Router) -> Result<(), std::io::Error> {
    let socket = TcpListener::bind(ADDR).await.unwrap();
    axum::serve(socket, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
}

async fn shutdown_signal() {
    signal::ctrl_c().await.unwrap();
}
