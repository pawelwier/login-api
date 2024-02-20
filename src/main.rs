use crate::hello::{routes_hello, routes_static};
use crate::model::ModelController;
use axum::{middleware, response::Response, Router};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

pub use self::error::{Error, Result};

mod hello;
mod model;
mod error;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let model_controller = ModelController::new().await?;
    
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(model_controller))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:5050").await.unwrap();
    println!("Listening on: {:<12?}", listener.local_addr());

    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!(">> {:<12} - main_response_mapper\n", "RES_MAPPER");
    res
}