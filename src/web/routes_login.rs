use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // TODO: Add auth/db logic
    if payload.username != "demo" || payload.pwd != "pass" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String
}