use axum::{routing::get, Router};

mod agents;
mod metrics;

pub fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Internal API" }))
        .nest("/metrics", metrics::app())
}
