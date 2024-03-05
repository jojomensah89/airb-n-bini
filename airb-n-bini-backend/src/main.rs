use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};

mod controllers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    server().await;
}


async fn server() {
    println!("Server started on port: {}","http://localhost:8080/");

   

    let app: Router = Router::new()
        .route("/", get(home));
       

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> impl IntoResponse {
    println!("Welcome Home");

    (StatusCode::ACCEPTED, "Hey There")
}
