use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use tokio::net::TcpStream;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Oxi is running on 127.0.0.1:6379");

    let store = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("New client connected: {:?}", socket.peer_addr());

        let store = Arc::clone(&store);

        tokio::spawn(async move {
            if let Err(e) = socket.write_all("Connected to Oxi!\r\n".as_bytes()).await {
                eprintln!("Failed to write to socket: {}", e);
            }
            handle_connection(socket, store).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, store: Arc<Mutex<HashMap<String, String>>>)  {
    let mut buffer = [0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }

            Ok(n) => {
                let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                println!("Received: {}", input);

                let response = process_command(input, &store);

                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    eprintln!("Failed to write to socket: {}", e);
                    break;
                }
            }

            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}

fn process_command(input: String, store: &Arc<Mutex<HashMap<String, String>>>) -> String {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return "ERROR: No command entered. Try 'SET key value' or 'GET key'\r\n".to_string();
    }

    match parts[0].to_uppercase().as_str() {
        "SET" => {
            if parts.len() != 3 {
                return "ERROR: Invalid SET command. Use 'SET key value'\r\n".to_string();
            }

            let mut db = store.lock().unwrap();
            db.insert(parts[1].to_string(), parts[2].to_string());
            "OK\r\n".to_string()
        }

        "GET" => {
            if parts.len() != 2 {
                return "ERROR: Invalid GET command. Use 'GET key'\r\n".to_string();
            }

            let db = store.lock().unwrap();
            match db.get(parts[1]) {
                Some(val) => format!("{}: {}\r\n", parts[1], val),
                None => "ERROR: Key not found\r\n".to_string(),
            }
        }

        _ => "ERROR: Invaid command. Try 'GET key' or 'SET key value'\r\n".to_string()
    }
}
