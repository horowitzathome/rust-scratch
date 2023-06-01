use axum::{response::IntoResponse, routing, Json, Router};
use rand::Rng;
use tracing::info;

pub async fn health() -> impl IntoResponse {
    info!("healthy");
    Json("healthy")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_file(true).with_line_number(true).init();

    let mut rng = rand::thread_rng();

    let rand: u64 = rng.gen();

    info!("Hello, world with random number {}!", rand);

    let router = Router::new().route("/health", routing::get(health));

    let server =
        axum::Server::bind(&std::net::SocketAddr::from(([0, 0, 0, 0], 8080))).serve(router.into_make_service());

    tokio::select! {
        _ = server => info!("axum server exited"),
    }
}
