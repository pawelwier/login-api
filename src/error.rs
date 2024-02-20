use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    TicketDeleteIdNotFound { id: u64 }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("INTO_RES: {:?}", self);

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()        
    }
}