use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use std::error::Error;

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("new client connected");

    let mut buf = [0; 1024];
    stream
        .write_all(b"Welcome to the PingPong server!\n")
        .await?;

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        stream.write_all(b"Pong\n").await?;
    }

    println!("client disconnected");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("error: {}", e);
            }
        });
    }
}
