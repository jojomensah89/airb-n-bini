use axum::{
    http::Method,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};

use crate::controllers::home_controller::{create_home, delete_home, get_homes};
use tower_http::cors::{Any, CorsLayer};

pub async fn home_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::DELETE, Method::PUT, Method::PUT])
        .allow_origin(Any);

    Router::new()
        .route("/home", get(home))
        .route("/homes", get(get_homes))
        .route("/home/create", post(create_home))
        // .route("/home/update", put(update_home))
        .route("/home/id/delete", delete(delete_home))
        .layer(cors)
}

async fn home() -> impl IntoResponse {
    (StatusCode::ACCEPTED, "Welcome to homes")
}
