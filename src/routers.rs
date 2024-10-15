use crate::{handler::*, middleware, oauth::*};
use axum::{
    routing::{delete, get, post},
    Extension, Router,
};
use tower::ServiceBuilder;

use crate::state;

pub fn init_router() -> Router {
    let cookie_layer = ServiceBuilder::new().layer(axum::middleware::from_fn(middleware::jwt_auth));
    let routes_with_auth = Router::new()
        .route("/key", post(create_key))
        .route("/key", get(get_keys))
        .route("/:key/url", post(add_url))
        .route("/:key/url", delete(delete_url))
        .route("/user", get(user_info))
        .layer(cookie_layer);
    let router_without_auth = Router::new()
        .route("/:key", post(url_balancing).get(url_balancing))
        .route("/:key/urls", get(get_urls))
        .route("/auth/linuxdo", get(linuxdo_auth))
        .route("/auth/authorized", get(linuxdo_authorized));
    Router::new()
        .merge(routes_with_auth)
        .merge(router_without_auth)
        .layer(Extension(state::APP_STATE.clone()))
}
