//! Provides the API layer, represented by an axum [`Router`].

use axum::{middleware, routing::get, Extension, Router};
use redis::aio::ConnectionManager;

mod layer;
mod ping;

/// Powers the API.
pub fn server(pool: ConnectionManager) -> Router {
    // `ConnectionManager` can just be cloned, it handles multiplexing and new connection creation.
    let api = Router::new()
        .route("/ping", get(ping::handle))
        .layer(Extension(pool))
        .route_layer(middleware::from_fn(layer::log));

    Router::new().nest("/api/v1", api)
}
