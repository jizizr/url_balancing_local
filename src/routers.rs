use std::path::{Path, PathBuf};

use crate::{handler::*, state};
use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    response::IntoResponse,
    routing::{delete, get, get_service, post},
    Extension, Router,
};
use tokio::fs;
use tower_http::{
    cors::{self, CorsLayer},
    services::ServeDir,
};

pub fn init_router() -> Router {
    let api_routes = Router::new()
        .route("/key", post(create_key))
        .route("/key", get(get_keys))
        .route("/:key/url", post(add_url))
        .route("/:key/url", delete(delete_url))
        .route("/user", get(user_info))
        .route("/:key", post(url_balancing).get(url_balancing))
        .route("/:key/urls", get(get_urls));
    let static_files = get_service(ServeDir::new(PathBuf::from("build")));

    Router::new()
        .nest("/api", api_routes) // 挂载 API 路由
        .route("/", get(serve_index)) // 访问 "/" 时返回 index.html
        .fallback_service(static_files)
        .layer(Extension(state::APP_STATE.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(cors::Any) // 允许任何来源
                .allow_methods(cors::Any) // 允许所有方法
                .allow_headers(cors::Any) // 允许所有请求头
                .allow_credentials(false), // 不允许携带凭证
        )
}

async fn serve_index() -> impl IntoResponse {
    let index_path = Path::new("build/index.html");

    match fs::read(index_path).await {
        Ok(content) => (StatusCode::OK, [(CONTENT_TYPE, "text/html")], content).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Index file not found".to_string()).into_response(),
    }
}
