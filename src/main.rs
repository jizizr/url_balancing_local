mod routes;
mod state;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use routes::{add_url, handle_request};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:key", get(handle_request))
        .route("/add/:key", post(add_url))
        .layer(Extension(state::APP_STATE.clone()));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
