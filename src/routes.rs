use crate::state::AppState;
use axum::extract::{Extension, Json, Path};
use axum::http::StatusCode;
use axum::response::Redirect;
use rand::{thread_rng, Rng};
use redis::AsyncCommands;
use serde::Deserialize;
use std::sync::Arc;

pub async fn handle_request(
    Path(key): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Redirect, StatusCode> {
    let redis_client = &state.redis_client;

    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let urls: Vec<String> = con
        .lrange(&key, 0, -1)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if urls.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    }

    let backend_url = {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..urls.len());
        urls[index].clone()
    };

    Ok(Redirect::temporary(&backend_url))
}

#[derive(Deserialize)]
pub struct AddUrlRequest {
    url: String,
}

pub async fn add_url(
    Path(key): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AddUrlRequest>,
) -> Result<StatusCode, StatusCode> {
    let redis_client = &state.redis_client;

    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    con.rpush(&key, payload.url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
