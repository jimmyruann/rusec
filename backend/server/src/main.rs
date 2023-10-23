use axum::{routing::get, Router};
use std::net::SocketAddr;

use agent_api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let internal_api = Router::new()
        .route("/", get(|| async { "Internal API" }))
        .nest("/agents", agent_api::app());

    let app: Router = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .nest("/internal", internal_api);

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
