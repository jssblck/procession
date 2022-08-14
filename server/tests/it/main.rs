//! Integration tests for the server.
//!
//! Important: You must have a local redis running that these tests can use.
//! By default, these tests will connect to `redis://localhost:6379/1`.
//! If you wish to run them against a different database, set the redis URL in the
//! environment variable `PROCESSION_TEST_REDIS` before running these tests.

use procession_client::Client;

mod run;

#[tokio::test]
async fn ping() {
    let addr = run::server().await;
    let client = Client::new(&addr).expect("construct client");
    let response = run::request!(client, ping);
    assert!(response.contains("redis latency:"));
}
