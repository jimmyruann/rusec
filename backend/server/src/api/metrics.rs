use axum::{
    extract,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    routing::{post, put},
    Json, Router,
};
use db::models::metric_categories::{NewMetricCategory, UpdateMetricCategory};
use db::{establish_connection, models};

async fn get_metric_categories() -> impl IntoResponse {
    let mut conn = establish_connection();
    let results = models::metric_categories::find_metric_categories(None, &mut conn);

    return Json(results);
}

async fn get_metric_category(extract::Path(id): extract::Path<String>) -> impl IntoResponse {
    let mut conn = establish_connection();
    let result = models::metric_categories::find_metric_category(id.as_str(), &mut conn);

    if result.is_none() {
        return (StatusCode::NOT_FOUND, Json(result));
    }

    return (StatusCode::OK, Json(result));
}

async fn create_metric_category(
    extract::Json(payload): Json<NewMetricCategory>,
) -> impl IntoResponse {
    let mut conn = establish_connection();

    let exist = models::metric_categories::check_metric_category_exist(&payload.id, &mut conn);

    if exist {
        return (
            StatusCode::BAD_REQUEST,
            format!("Metric Category ID `{}` already exist.", &payload.id),
        )
            .into_response();
    }

    let result = models::metric_categories::create_metric_category(&payload, &mut conn);

    return (StatusCode::CREATED, Json(result)).into_response();
}

async fn update_metric_category(
    extract::Path(id): extract::Path<String>,
    extract::Json(payload): Json<UpdateMetricCategory>,
) -> impl IntoResponse {
    let mut conn = establish_connection();

    let result =
        models::metric_categories::update_metric_category(&id.as_str(), &payload, &mut conn);

    match result {
        Ok(val) => (StatusCode::OK, Json(val)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err).into_response(),
    }
}

pub fn app() -> Router {
    Router::new()
        .route("/categories", get(get_metric_categories))
        .route("/categories/:id", get(get_metric_category))
        .route("/categories/:id", put(update_metric_category))
        .route("/categories", post(create_metric_category))
}
