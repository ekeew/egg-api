use egg_api::presentation;

#[tokio::main]
async fn main() {
    presentation::api().await;
}
