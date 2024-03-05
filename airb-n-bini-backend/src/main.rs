use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};
use sea_orm::{Database, DatabaseConnection};

mod controllers;
mod models;
mod routes;
mod utils;
use routes::{home_routes::home_routes,user_routes::user_routes};
use utils::constants::DATABASE_URL;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {
    let db_url = DATABASE_URL.clone();

    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    let app: Router = Router::new()
        .route("/", get(home))
        .merge(user_routes().await)
        .merge(home_routes().await)
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Server started on port: {}", "http://localhost:8080/");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

}

async fn home() -> impl IntoResponse {
    (StatusCode::ACCEPTED, "Hey There")
}
