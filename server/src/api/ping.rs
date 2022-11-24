use std::time::Instant;

use axum::http::StatusCode;

pub async fn handle() -> Result<String, StatusCode> {
    let now = Instant::now();
    Ok(format!("running :: {now:?}"))
}
