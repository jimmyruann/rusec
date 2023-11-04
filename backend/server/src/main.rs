use axum::{routing::get, Router};
use std::net::SocketAddr;

mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app: Router = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .nest("/api", api::app());

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
