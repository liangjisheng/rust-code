//! Run with
//!
//! ```not_rust
//! cargo run --bin hello-world
//! cargo run -p hello-world
//! ```

use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::{
    http::StatusCode, response::Html, response::IntoResponse, routing::get, routing::post, Json,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    // build our application with a single route
    // 这里 /foo 同时绑定了GET及POST方法的路由
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hello", get(handler))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/hello1", get(handler1))
        .route("/j1", get(response_json))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// 处理程序可以返回任何实现了 IntoResponse 的东西，它将被自动转换为响应

async fn handler() -> &'static str {
    "Hello, world!"
}

async fn string() -> String {
    "Hello, world!".to_string()
}

// Returning a tuple of `StatusCode` and another `IntoResponse` will
// change the status code
async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "not found")
}

async fn handler1() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn get_foo() -> String {
    String::from("get request foo")
}

async fn post_foo() -> String {
    String::from("post request foo")
}

// Path路径，eg. /users/<id>
async fn path(Path(user_id): Path<u32>) {}

// Query参数，eg. /users?id=123&name=jim
async fn query(Query(params): Query<HashMap<String, String>>) {}

// Json 格式参数，一般用于 POST 请求
async fn get_json(Json(payload): Json<Value>) {}

#[derive(Serialize, Deserialize, Debug)]
struct CreateUser {
    username: String,
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
    // ) -> (StatusCode, Json<CreateUser>) {
) -> impl IntoResponse {
    // insert your application logic here
    // let user = CreateUser {
    //     username: payload.username,
    // };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(payload))
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn response_json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

// curl "http://127.0.0.1:3000/"
// curl "http://127.0.0.1:3000/hello"
// curl "http://127.0.0.1:3000/hello1"
// curl "http://127.0.0.1:3000/j1"
// curl -X GET http://127.0.0.1:3000/foo
// curl -X POST http://127.0.0.1:3000/foo
// curl -H "Content-Type: application/json" -d '{"username":"someName"}' -X POST http://127.0.0.1:3000/users
