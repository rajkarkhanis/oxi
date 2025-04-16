use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::store::Store;
use crate::command::process_command;

pub async fn handle_connection(socket: &mut TcpStream, store: Store) -> anyhow::Result<()> {
    let mut buffer = [0u8; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected");
                return Ok(());
            }

            Ok(n) => {
                let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                println!("Received: {}", input);

                let response = process_command(&input, &store);

                socket.write_all(response.as_bytes()).await
                    .map_err(|e| {
                        eprintln!("Failed to write to socket: {}", e);
                        e
                    })?;
            }

            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return Err(e.into());
            }
        }
    }
}
