use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;

use crate::store::Store;
use crate::connection::handle_connection;

pub async fn run(addr: &str, store: Store) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Oxi is running on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("New client connected: {:?}", socket.peer_addr());

        let store = store.clone();

        tokio::spawn(async move {
            if let Err(e) = socket.write_all("Connected to Oxi!\r\n".as_bytes()).await {
                eprintln!("Failed to write to socket: {}", e);
            }

            if let Err(e) = handle_connection(&mut socket, store).await {
                eprintln!("Client error: {}", e);
            }
        });
    }
}
