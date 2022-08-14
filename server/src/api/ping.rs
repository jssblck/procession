use tracing::info;

pub async fn handle() -> &'static str {
    info!("Got a ping, responding");
    "pong"
}
