mod server;
mod store;
mod connection;
mod command;
mod resp;
mod aof;

use store::Store;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let store = Store::new();
    aof::replay(&store)
        .unwrap_or_else(|e| eprintln!("AOF Replay failed: {}", e));

    server::run("127.0.0.1:6379", store).await
}

