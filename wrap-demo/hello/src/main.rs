use std::net::SocketAddr;
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    // let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    warp::serve(routes).run(addr).await;
}

// curl "http://127.0.0.1:3000/hello/wrap"
