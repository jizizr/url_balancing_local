mod error;
mod jwt;
mod middleware;
mod oauth;
mod routes;
mod state;
mod token;

use axum::{
    routing::{delete, get, post},
    Extension, Router,
};
use oauth::*;
use routes::*;
use std::net::SocketAddr;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let _cookie_layer = ServiceBuilder::new().layer(axum::middleware::from_fn(middleware::jwt_auth));

    let app = Router::new()
        .route("/key", post(create_key).get(create_key))
        .route("/:key", post(handle_request))
        .route("/:key/url", post(add_url))
        .route("/:key/url", delete(delete_url))
        .route("/:key/urls", get(get_urls))
        .route("/auth/linuxdo", get(linuxdo_auth))
        .route("/auth/authorized", get(linuxdo_authorized))
        // .route(
        //     "/info",
        //     get(|Extension(user): Extension<LinuxDoUser>| async move { println!("{:?}", user) })
        //         .route_layer(cookie_layer),
        // )
        .layer(Extension(state::APP_STATE.clone()));
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse().unwrap()));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
