use axum::{routing::get, Router};

mod ping;

/// Powers the API.
pub fn server() -> Router {
    let api = Router::new().route("/ping", get(ping::handle));
    Router::new().nest("/api/v1", api)
}
