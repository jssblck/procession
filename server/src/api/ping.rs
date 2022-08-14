use std::time::Instant;

use crate::redis;
use ::redis::aio::ConnectionManager;
use axum::{http::StatusCode, Extension};
use tracing::error;

pub async fn handle(
    Extension(mut pool): Extension<ConnectionManager>,
) -> Result<String, StatusCode> {
    let start = Instant::now();
    redis::ping(&mut pool).await.map_err(|e| {
        error!("handle /ping: {e:#}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(format!("redis latency: {:#?}", start.elapsed()))
}
