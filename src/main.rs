use axum::Router;
use tokio::net::TcpListener;
use crate::hello::{routes_hello, routes_static};

mod hello;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on: {:?}", listener.local_addr());

    axum::serve(listener, routes_all).await.unwrap();
}
