use crate::controllers::user_controller::create_user;
use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

pub async fn user_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::DELETE, Method::PUT])
        .allow_origin(Any);

    Router::new()
        // .route("/users", get(get_all_users))
        .route("/user/create", post(create_user))
        // .route("/user/update", put(update_user))
        // .route("/user/:uuid/delete", delete(delete_user))
        .layer(cors)
}
