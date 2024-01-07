use axum::{body::Body, http::Request, response::Response, routing::get_service, Router};
use std::convert::Infallible;
use tower::service_fn;
use tower_http::services::ServeFile;

// axum 也可以将请求路由到任何 tower 服务。可以是你用 service_fn 编写的服务，
// 也可以是来自其他 crate 的东西，比如来自 tower-http 的ServeFile

#[tokio::main]
async fn main() {
    let app = Router::new()
        // GET `/static/Cargo.toml` goes to a service from tower-http
        .route("/static", get_service(ServeFile::new("Cargo.toml")))
        .route(
            // Any request to `/` goes to a some `Service`
            "/",
            get_service(service_fn(|_: Request<Body>| async {
                let res = Response::new(Body::from("Hi from `GET /`"));
                Ok::<_, Infallible>(res)
            })),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// curl "http://127.0.0.1:3000/static"
// curl "http://127.0.0.1:3000/"
