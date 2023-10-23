use axum::{response::IntoResponse, routing::get, Router};

async fn get_agents() -> impl IntoResponse {
    "agents"
}

pub fn app() -> Router {
    Router::new().route("/", get(get_agents))
}
