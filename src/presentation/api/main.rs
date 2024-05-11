use std::{net::SocketAddr, process::ExitCode};

use axum::Router;
use tokio::net::TcpListener;

use super::setup;

pub async fn api() -> ExitCode {
    let config = match setup::read_config() {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("{e}");
            return ExitCode::FAILURE;
        },
    };
    setup::run_logging();

    tracing::info!("Initializing...");
    let app = setup::create_app();

    tracing::info!("Starting server...");
    tracing::warn!("Be careful with eggs!");
    let result = serve(app, config.server.addr).await;
    tracing::info!("Server stopped");

    if let Err(e) = result {
        tracing::error!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

async fn serve(app: Router, addr: SocketAddr) -> Result<(), std::io::Error> {
    let socket = TcpListener::bind(addr).await.unwrap();
    axum::serve(socket, app)
        .with_graceful_shutdown(setup::get_shutdown_signal())
        .await
}