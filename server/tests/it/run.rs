//! Helpers for executing an ephemeral server instance.

use std::net::TcpListener;

use axum::Server;
use procession::api;

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

// https://stackoverflow.com/a/67140319
pub(crate) use request;

/// Runs a server on a random port in the current Tokio runtime.
#[track_caller]
pub fn server() -> String {
    let app = api::server();
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
