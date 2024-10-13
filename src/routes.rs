use crate::{state::AppState, token};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::Redirect,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn handle_request(
    Path(key): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Redirect, StatusCode> {
    let backend_url = state
        .get_url(&key)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Redirect::temporary(&backend_url))
}

#[derive(Deserialize)]
pub struct AddUrlRequest {
    url: String,
}

#[derive(Serialize)]
pub struct CommonResponse<S: Serialize> {
    code: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<S>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: i8,
    pub error: String,
}

pub async fn add_url(
    Path(key): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AddUrlRequest>,
) -> Result<(StatusCode, Json<CommonResponse<()>>), StatusCode> {
    if !state.check_key(&key).await.unwrap() {
        return Err(StatusCode::NOT_FOUND);
    }

    state
        .add_url(&key, &payload.url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        Json(CommonResponse {
            code: 0,
            data: None,
        }),
    ))
}

pub async fn create_key(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<CommonResponse<String>>), StatusCode> {
    let key = token::new_token();
    if state
        .check_key(&key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Ok((
            StatusCode::OK,
            Json(CommonResponse {
                code: 1,
                data: None,
            }),
        ));
    }
    state
        .add_key(&key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((
        StatusCode::OK,
        Json(CommonResponse {
            code: 0,
            data: Some(key),
        }),
    ))
}

pub async fn delete_url(
    Path(key): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
    Json(url): Json<AddUrlRequest>,
) -> Result<(StatusCode, Json<CommonResponse<()>>), StatusCode> {
    if !state.check_key(&key).await.unwrap() {
        return Err(StatusCode::NOT_FOUND);
    }

    state
        .delete_url(&key, &url.url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        Json(CommonResponse {
            code: 0,
            data: None,
        }),
    ))
}

pub async fn get_urls(
    Path(key): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<CommonResponse<Vec<String>>>), StatusCode> {
    if !state.check_key(&key).await.unwrap() {
        return Err(StatusCode::NOT_FOUND);
    }

    let urls = state
        .get_urls(&key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        Json(CommonResponse {
            code: 0,
            data: Some(urls),
        }),
    ))
}
