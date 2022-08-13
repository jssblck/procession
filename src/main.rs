use std::{net::SocketAddr, str::FromStr};

use axum::{Router, Server};
use clap::Parser;
use color_eyre::{eyre::eyre, eyre::Context, Result};

use redis::{aio::ConnectionManager, Client};
use tracing::info;
use url::Url;

mod api;
mod style;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Listen address, in host:port format.
    #[clap(
        short = 'l',
        long,
        default_value = "0.0.0.0:3000",
        takes_value = true,
        parse(try_from_str = SocketAddr::from_str)
    )]
    listen: SocketAddr,

    /// Redis address, in host:port format.
    #[clap(
        short = 'r',
        long,
        default_value = "redis://localhost:6379/0",
        takes_value = true,
        parse(try_from_str = parse_redis_url)
    )]
    redis_host: Url,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let listen = args.listen;
    let redis = args.redis_host;

    let service_name = env!("CARGO_PKG_NAME");
    let service_version = env!("CARGO_PKG_VERSION");

    info!("🤔 Checking connection to {}", style::url(&redis));
    let client = Client::open(redis.clone()).wrap_err("create redis connection")?;
    let mut manager = ConnectionManager::new(client)
        .await
        .wrap_err("upgrade redis connection to managed pool")?;
    redis::cmd("PING")
        .query_async(&mut manager)
        .await
        .wrap_err("ping redis")?;
    info!("💚 Redis is running at {}", style::url(&redis));

    info!(
        "👩🏻‍💻 Starting {} {} on {}",
        style::constant(&service_name),
        style::constant(&service_version),
        style::url(&listen),
    );
    let app = Router::new()
        // .merge(api::server(client))
        ;
    let bind = Server::try_bind(&listen).wrap_err("bind listen address")?;
    let server = bind.serve(app.into_make_service());

    info!(
        "✨ Serving {} {} on {}",
        style::constant(&service_name),
        style::constant(&service_version),
        style::url(&listen)
    );
    server.await.wrap_err("run server")?;

    info!("😴 Service exited cleanly, shutting down");
    Ok(())
}

fn parse_redis_url(input: &str) -> Result<Url> {
    redis::parse_redis_url(input).ok_or_else(|| eyre!("invalid redis url: {input}"))
}
