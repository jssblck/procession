use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use redis::{aio::ConnectionManager, Client};
use url::Url;

use crate::style;

pub async fn connect(addr: &Url) -> Result<ConnectionManager> {
    let client = Client::open(addr.clone()).wrap_err("create redis connection")?;
    let manager = ConnectionManager::new(client)
        .await
        .wrap_err("upgrade redis connection to managed pool")?;
    Ok(manager)
}

pub async fn ping(connection: &mut ConnectionManager) -> Result<()> {
    redis::cmd("PING")
        .query_async(connection)
        .await
        .wrap_err("ping redis")?;
    Ok(())
}

/// This is like [`redis::parse_redis_url`], but doesn't throw away errors.
pub fn parse_url(input: &str) -> Result<Url> {
    let valid_schemes = vec!["redis", "rediss", "redis+unix", "unix"];
    let display_schemes = || {
        valid_schemes
            .iter()
            .map(style::constant)
            .collect::<Vec<_>>()
            .join(", ")
    };

    match Url::parse(input) {
        Ok(result) => match result.scheme() {
            scheme if valid_schemes.contains(&scheme) => Ok(result),
            _ => Err(eyre!(
                "invalid scheme, must be one of {}",
                display_schemes()
            )),
        },
        Err(e) => Err(eyre!("parse url: {e}")),
    }
}
