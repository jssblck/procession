//! Helpers for executing an ephemeral server instance.

use std::{env, net::TcpListener};

use axum::Server;
use procession::{api, redis_ext};
use redis::aio::ConnectionManager;

use crate::{DEFAULT_REDIS, REDIS_ENV};

/// Run the client method and unwrap the result.
macro_rules! request {
    ($client:ident, $method:ident) => {{
        $client
        .$method()
        .await
        .expect("communicate with server")
    }};
    ($client:ident, $method:ident, $( $input:expr ),*) => {{
        $client
        .$method( $($input),* )
        .await
        .expect("communicate with server")
    }};
}

/// Runs a server on a random port in the current Tokio runtime.
#[track_caller]
pub async fn server() -> String {
    let conn = connect_redis().await;
    let app = api::server(conn);

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral socket");
    let addr = listener.local_addr().expect("get bound address");

    tokio::spawn(async move {
        let server = Server::from_tcp(listener)
            .expect("create server")
            .serve(app.into_make_service());
        server.await.expect("server error");
    });

    format!("http://{addr}")
}

#[track_caller]
async fn connect_redis() -> ConnectionManager {
    let redis_addr = env::var(REDIS_ENV).unwrap_or_else(|_| DEFAULT_REDIS.into());
    let redis_addr = redis_ext::parse_url(&redis_addr).expect("must parse redis address");

    let mut conn = redis_ext::connect(&redis_addr)
        .await
        .unwrap_or_else(|_| panic!("must connect to redis at {redis_addr}"));

    redis_ext::ping(&mut conn)
        .await
        .expect("redis must be online");

    conn
}

// https://stackoverflow.com/a/67140319
pub(crate) use request;
