use procession_client::Client;

mod run;

#[tokio::test]
async fn ping() {
    let addr = run::server();
    let client = Client::new(&addr).expect("construct client");
    run::request!(client, ping);
}
