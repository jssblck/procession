//! Provides the API layer, represented by an axum [`Router`].
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Extension, Router,
};

use redis::aio::ConnectionManager;
use tracing::info;

use crate::{
    internal::metrics::{Collection, Measure},
    style,
};

mod ping;

/// Powers the API.
pub fn server(pool: ConnectionManager) -> Router {
    let metrics = Collection::new(1000);

    // `ConnectionManager` can just be cloned, it handles multiplexing and new connection creation.
    let api = Router::new()
        .route("/ping", get(ping::handle))
        .layer(middleware::from_fn(log))
        .layer(Extension(pool))
        .layer(Extension(metrics));

    Router::new().nest("/api/v1", api)
}

/// Log when an endpoint is entered and when it is left.
pub async fn log<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let metrics = req
        .extensions()
        .get::<Collection>()
        .expect("Metrics must be layered in")
        .clone();

    let start = Instant::now();

    let route = style::url(&req.uri().path());
    let query = req
        .uri()
        .query()
        .map(|qs| format!("?{qs}"))
        .map(|qs| style::url(&qs))
        .unwrap_or_default();

    info!("Serving {route}{query}");
    let result = next.run(req).await;

    let latency = start.elapsed();
    metrics.rec_latency(Measure::EndpointLatency, latency).await;

    let latency = format!("{:#?}", start.elapsed());
    let latency = style::constant(&latency);
    let status = style::status_code(result.status());
    info!("Served {route}{query} in {latency}: {status}");

    Ok(result)
}
