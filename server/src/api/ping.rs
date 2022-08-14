use crate::redis;
use ::redis::aio::ConnectionManager;
use axum::{http::StatusCode, Extension};
use tracing::error;

pub async fn handle(
    Extension(mut pool): Extension<ConnectionManager>,
) -> Result<String, StatusCode> {
    let latency = redis::ping(&mut pool).await.map_err(|e| {
        error!("redis: {e:#}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(format!("redis latency: {:#?}", latency))
}
