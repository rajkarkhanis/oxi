use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Oxi is running on 127.0.0.1:6379");

    loop {
        let (socket, _) = listener.accept().await?;
        println!("New client connected: {:?}", socket.peer_addr());

        tokio::spawn(async move {
            println!("hi bro")
        });
    }
}
