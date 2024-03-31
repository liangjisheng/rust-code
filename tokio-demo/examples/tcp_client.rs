use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use std::error::Error;

async fn connect() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await?;
    println!("connected to {}", addr);

    stream.write_all(b"Ping\n").await?;
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    let pong = std::str::from_utf8(&buf[..n])?;
    println!("{}", pong);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    connect().await?;
    Ok(())
}
