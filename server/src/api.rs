//! Provides the API layer, represented by an axum [`Router`].

use axum::{routing::get, Extension, Router};
use redis::aio::ConnectionManager;

mod ping;

/// Powers the API.
pub fn server(pool: ConnectionManager) -> Router {
    // `ConnectionManager` can just be cloned, it handles multiplexing and new connection creation.
    let api = Router::new()
        .route("/ping", get(ping::handle))
        .layer(Extension(pool));

    Router::new().nest("/api/v1", api)
}
