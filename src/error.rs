use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use strum_macros::EnumDiscriminants;
use thiserror::Error;

#[repr(i8)]
#[derive(Error, Debug, EnumDiscriminants)]
#[strum_discriminants(name(AppErrorKind))]
pub enum AppError {
    #[error("404 Not Found")]
    HTTPNotFound,
    #[error("url解析失败")]
    UrlParse(#[from] url::ParseError),
    #[error("sqlx错误: {0}")]
    Redis(#[from] sqlx::Error),
    #[error("state过期或不存在")]
    StateNotFound,
    #[error("未认证")]
    Unauthorized,
    #[error("无效的响应")]
    Invalid,
    #[error("数量达到上限")]
    Limit,
    #[error("未知错误")]
    Unknown,
}
impl From<&AppError> for i8 {
    fn from(error: &AppError) -> i8 {
        let kind: AppErrorKind = AppErrorKind::from(error); // 获取判别器
        kind as i8
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    code: i8,
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match &self {
            AppError::HTTPNotFound => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
            _ => (
                StatusCode::OK,
                Json(ErrorResponse {
                    code: <&AppError as Into<i8>>::into(&self),
                    error: self.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
