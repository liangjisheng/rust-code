use salvo::prelude::*;

#[handler]
async fn hello(req: &mut Request) -> String {
    format!("Request id: {:?}", req.header::<String>("x-request-id"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    let router = Router::new().hoop(RequestId::new()).get(hello);
    Server::new(acceptor).serve(router).await;
}

// curl "http://127.0.0.1:5800/"
// curl "http://127.0.0.1:5800/" -H "x-request-id: 123456789"
