//! Layers for the API endpoints.

use std::time::Instant;

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing::info;

use crate::style;

/// Log when an endpoint is entered and when it is left.
pub async fn log<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let start = Instant::now();

    let route = style::url(&req.uri().path());
    let query = req
        .uri()
        .query()
        .map(|qs| format!("?{qs}"))
        .map(|qs| style::url(&qs))
        .unwrap_or_default();

    info!("📨 Serving {route}{query}");
    let result = next.run(req).await;

    let latency = format!("{:#?}", start.elapsed());
    let latency = style::constant(&latency);
    let status = style::status_code(result.status());
    info!("💌 Served {route}{query} in {latency}: {status}");

    Ok(result)
}
