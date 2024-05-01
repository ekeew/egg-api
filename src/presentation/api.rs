use std::process::ExitCode;

use axum::Router;
use tokio::net::TcpListener;

mod endpoints;
mod setup;

const ADDR: &str = "127.0.0.1:8080";

pub async fn api() -> ExitCode {
    setup::run_logging();

    tracing::info!("Initializing...");
    let app = setup::create_app();

    tracing::info!("Starting server...");
    let result = serve(app).await;
    tracing::info!("Server stopped");

    if let Err(e) = result {
        tracing::error!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

async fn serve(app: Router) -> Result<(), std::io::Error> {
    let socket = TcpListener::bind(ADDR).await.unwrap();
    axum::serve(socket, app)
        .with_graceful_shutdown(setup::shutdown_signal())
        .await
}

