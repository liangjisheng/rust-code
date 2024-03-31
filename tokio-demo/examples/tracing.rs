use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tracing::{debug, error, info, span, Level};
use tracing_futures::Instrument;

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];

    loop {
        let n = stream.read(&mut buf).await?;

        if n == 0 {
            return Ok(());
        }

        stream.write_all(&buf[0..n]).await?;
    }
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((stream, _)) = listener.accept().await {
        let span = span!(Level::INFO, "client", remote_addr = %stream.peer_addr()?);
        let _enter = span.enter();

        info!("accepted connection");

        tokio::spawn(async move {
            handle_client(stream)
                .instrument(span!(Level::INFO, "handle_client"))
                .await
                .unwrap_or_else(|e| error!("error: {:?}", e));
        });
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("starting server");

    run_server().await?;

    Ok(())
}
