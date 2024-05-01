use std::process::ExitCode;

use egg_api::presentation;

#[tokio::main]
async fn main() -> ExitCode {
    presentation::api().await
}
