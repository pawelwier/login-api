use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::{get, get_service}, Router};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

/* Practice routes */
async fn handler_hello_query(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("Entering {params:?}");
    let name = params.name.as_deref().unwrap_or("World");
    Html (format!("Hello <b>{name}</b>"))
}

async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    Html (format!("Hello <b>{name}</b>"))
}

pub fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello_query))
        .route("/hello2/:name", get(handler_hello_path))
}

pub fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}