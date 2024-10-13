use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    body::Body,
    http::{header::COOKIE, Request},
    middleware::Next,
    response::Response,
};
use cookie::Cookie;
use crate::{error::AppError, jwt};

pub async fn jwt_auth(mut req: Request<Body>, next: Next) -> Result<Response, AppError> {
    if let Some(cookie_header) = req.headers().get(COOKIE) {
        let cookie = Cookie::parse(cookie_header.to_str().map_err(|_| AppError::Invalid)?)
            .map_err(|_| AppError::Invalid)?;
        if cookie.name() == "jwt" {
            let claim = jwt::verify_jwt(cookie.value())?;
            let data = claim.claims;
            if data.exp
                < SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            {
                return Err(AppError::Unauthorized);
            }
            req.extensions_mut().insert(data.user);
            return Ok(next.run(req).await);
        }
    }
    Err(AppError::Unauthorized)
}
