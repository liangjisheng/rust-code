// 这个功能很关键，因为它允许我们只写一次中间件，并在不同的应用中分享它们。例如，axum 不需要提供自己的
// tracing/logging 中间件，可以直接使用来自 tower-http 的 TraceLayer。同样的中间件也可以用于用
// tonic 制作的客户端或服务器路由到任何 tower::Service

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

#[derive(Debug, Clone)]
struct AppState {
    // ...
    state: u32,
}

#[tokio::main]
async fn main() {
    let shared_state = AppState { state: 0 };

    let middleware_stack = ServiceBuilder::new()
        // add high level tracing of requests and responses
        .layer(TraceLayer::new_for_http())
        // compression responses
        .layer(CompressionLayer::new())
        .layer(middleware::from_fn(check_hello_world))
        // convert the `ServiceBuilder` into a `tower::Layer`;
        .into_inner();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(middleware_stack)
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            check_state,
        ))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn check_hello_world(req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("check_hello_world");
    let default = headers::HeaderValue::from_str("").unwrap();
    if req.headers().get("Content-Type").unwrap_or(&default) != "application/json" {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(next.run(req).await)
}

async fn check_state(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    println!("state {}", state.state);

    Ok(next.run(req).await)
}

// curl "http://127.0.0.1:3000/"
