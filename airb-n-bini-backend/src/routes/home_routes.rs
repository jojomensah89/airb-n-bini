use axum::{http::Method, http::StatusCode, response::IntoResponse, routing::get, Router};

use tower_http::cors::{Any, CorsLayer};

pub async fn user_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::DELETE, Method::PUT])
        .allow_origin(Any);

    Router::new().route("/user/create", get(home)).layer(cors)
}

async fn home() -> impl IntoResponse {
    (StatusCode::ACCEPTED, "Welcome to homes")
}
