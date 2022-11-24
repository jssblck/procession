//! Provides the API layer, represented by an axum [`Router`].
use std::time::Instant;

use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use tracing::info;

mod ping;

/// Powers the API.
pub fn server() -> Router {
    let api = Router::new()
        .route("/ping", get(ping::handle))
        .route_layer(middleware::from_fn(log));

    Router::new().nest("/api/v1", api)
}

/// Log when an endpoint is entered and when it is left.
pub async fn log<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let start = Instant::now();

    let route = req.uri().path().to_string();
    let query = req
        .uri()
        .query()
        .map(|qs| format!("?{qs}"))
        .unwrap_or_default();

    info!("Serving {route}{query}");
    let result = next.run(req).await;

    let latency = format!("{:#?}", start.elapsed());
    let status = result.status();
    info!("Served {route}{query} in {latency}: {status}");

    Ok(result)
}
