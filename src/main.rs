mod routes;
mod state;
mod token;

use axum::{
    routing::{delete, get, post},
    Extension, Router,
};
use routes::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/key", post(create_key))
        .route("/:key", get(handle_request))
        .route("/:key/url", post(add_url))
        .route("/:key/url", delete(delete_url))
        .route("/:key/urls", get(get_urls))
        .layer(Extension(state::APP_STATE.clone()));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
