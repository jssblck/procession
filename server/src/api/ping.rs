use crate::{redis_ext, style};
use ::redis::aio::ConnectionManager;
use axum::{http::StatusCode, Extension};
use tracing::{error, info};

pub async fn handle(
    Extension(mut pool): Extension<ConnectionManager>,
) -> Result<String, StatusCode> {
    let latency = redis_ext::ping(&mut pool)
        .await
        .map_err(|e| {
            error!("redis: {e:#}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .map(|d| format!("{:#?}", d))?;

    info!("Redis latency: {}", style::constant(&latency));
    Ok(format!("redis latency: {latency}"))
}
