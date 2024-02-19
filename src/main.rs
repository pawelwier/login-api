use axum::{middleware, response::Response, Router};
use tokio::net::TcpListener;
use crate::hello::{routes_hello, routes_static};

pub use self::error::{Error, Result};

mod hello;
mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on: {:<12?}", listener.local_addr());

    axum::serve(listener, routes_all).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!(">> {:<12} - main_response_mapper\n", "RES_MAPPER");
    res
}