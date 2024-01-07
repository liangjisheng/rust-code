use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let cert = include_bytes!("../certs/cert.pem").to_vec();
    let key = include_bytes!("../certs/key.pem").to_vec();

    let router = Router::new().get(hello);
    let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));
    let listener = TcpListener::new(("127.0.0.1", 8080)).rustls(config.clone());

    let acceptor = QuinnListener::new(config, ("127.0.0.1", 8080))
        .join(listener)
        .bind()
        .await;

    Server::new(acceptor).serve(router).await;
}

// curl "http://127.0.0.1:8080/"
