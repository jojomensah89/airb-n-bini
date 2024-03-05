use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};
use sea_orm::{Database, DatabaseConnection};

mod controllers;
mod models;
mod routes;
mod utils;
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

    let app: Router = Router::new().route("/", get(home)).layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    println!("Server started on port: {}", "http://localhost:8080/");
}

async fn home() -> impl IntoResponse {
    (StatusCode::ACCEPTED, "Hey There")
}
