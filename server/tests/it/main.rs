//! Integration tests for the server.
//!
//! Important: Redis must be running.
//! By default, these tests connect to the address specified by [`DEFAULT_REDIS`].
//!
//! To run them against a different database, set the redis URL in the
//! environment variable specified by [`REDIS_ENV`] before running these tests.

use procession_client::Client;

mod run;

#[tokio::test]
async fn ping() {
    let addr = run::server().await;
    let client = Client::new(&addr).expect("construct client");
    let response = run::request!(client, ping);
    assert!(response.contains("redis latency:"));
}
