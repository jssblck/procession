//! Integration tests for the server.
//!
//! Important: Redis must be running.
//! By default, these tests connect to the address specified by [`DEFAULT_REDIS`].
//!
//! To run them against a different database, set the redis URL in the
//! environment variable specified by [`REDIS_ENV`] before running these tests.

use procession_client::Client;

mod run;

/// The default redis server address.
pub const DEFAULT_REDIS: &str = "redis://localhost:6379/1";

/// The environment variable specifying the redis server address.
pub const REDIS_ENV: &str = "PROCESSION_TEST_REDIS";

#[tokio::test]
async fn ping() {
    let addr = run::server().await;
    let client = Client::new(&addr).expect("construct client");
    let response = run::request!(client, ping);
    assert!(response.contains("redis latency:"));
}
