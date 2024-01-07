// 这个功能很关键，因为它允许我们只写一次中间件，并在不同的应用中分享它们。例如，axum 不需要提供自己的
// tracing/logging 中间件，可以直接使用来自 tower-http 的 TraceLayer。同样的中间件也可以用于用
// tonic 制作的客户端或服务器路由到任何 tower::Service

use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    let middleware_stack = ServiceBuilder::new()
        // add high level tracing of requests and responses
        .layer(TraceLayer::new_for_http())
        // compression responses
        .layer(CompressionLayer::new())
        // convert the `ServiceBuilder` into a `tower::Layer`;
        .into_inner();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(middleware_stack);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// curl "http://127.0.0.1:3000/"
