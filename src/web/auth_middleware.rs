use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!(">> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO: add real auth
    auth_token.ok_or(Error::AuthFailedNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}

// Parse the auth token: user-[user-id].[expiration].[signature]
// returns (user-id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    todo!()
}