use std::io::{self, Write};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
    println!("Connected to Oxi (127.0.0.1:6379)");
    println!("Type commands like: SET key value or GET key\n");

    let mut buffer = vec![0; 1024];

    loop {
        print!("oxi> ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        // Handle exit command
        if parts[0].eq_ignore_ascii_case("exit") {
            println!("Bye!");
            break;
        }

        let resp = match parts[0].to_uppercase().as_str() {
            "SET" if parts.len() == 3 => {
                format!(
                    "*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
                    parts[1].len(),
                    parts[1],
                    parts[2].len(),
                    parts[2]
                )
            },

            "GET" if parts.len() == 2 => {
                format!(
                    "*2\r\n$3\r\nGET\r\n${}\r\n{}\r\n",
                    parts[1].len(),
                    parts[1]
                )
            },

            "DEL" if parts.len() > 1 => {
                let keys = parts[1..].iter()
                    .map(|key| format!("${}\r\n{}\r\n", key.len(), key))
                    .collect::<Vec<String>>()
                    .join("");

                format!(
                    "*{}\r\n$3\r\nDEL\r\n{}\r\n",
                    parts.len() - 1,
                    keys
                )
            },

            _ => {
                println!("Invalid command. Try: SET key value or GET key");
                continue;
            }
        };

        stream.write_all(resp.as_bytes()).await?;

        let n = stream.read(&mut buffer).await?;
        println!("{}", String::from_utf8_lossy(&buffer[..n]));
    }

    Ok(())
}
