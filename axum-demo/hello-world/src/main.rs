//! Run with
//!
//! ```not_rust
//! cargo run --bin hello-world
//! cargo run -p hello-world
//! ```

use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::{
    http::StatusCode, response::Html, response::IntoResponse, response::Response, routing::get,
    routing::post, Json, Router,
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
        .route("/p/:uid", get(path))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/hello1", get(handler1))
        .route("/j1", get(response_json1))
        .route("/j2", get(response_json2))
        .route("/j3", get(response_json3))
        .route("/j4", get(response_json4))
        .route("/my", get(my_function))
        .route("/my1", get(my_function1))
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
async fn path(Path(user_id): Path<u32>) -> Json<u32> {
    println!("user id {}", user_id);
    Json(user_id)
}

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
async fn response_json1() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn response_json2() -> Json<String> {
    Json(String::from("response_json1"))
}

async fn response_json3() -> Json<Vec<String>> {
    Json(vec!["string1".to_string(), "string2".to_owned()])
}

async fn response_json4() -> Json<CreateUser> {
    let user = CreateUser {
        username: "alice".to_string(),
    };
    Json(user)
}

#[derive(Serialize)]
struct Message {
    message: String,
}

// 定义了几种 `API` 的响应类型。
// 1. `OK` 和 `Created` 对应不同的 `HTTP` 状态码;
// 2. `JsonData` 包装了 `Vec<Message>` 的 `JSON` 数据。
enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
}

// 这让 `ApiResponse` 可以被自动转换成一个 `axum Response`。
impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        // 检查枚举变量,返回相应的 HTTP 状态码和数据。
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

async fn my_function() -> ApiResponse {
    ApiResponse::JsonData(vec![Message {
        message: "hello message".to_owned(),
    }])
}

enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            ApiError::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            ApiError::Unauthorised => (StatusCode::UNAUTHORIZED).into_response(),
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}

async fn my_function1() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::JsonData(vec![Message {
        message: "hello message".to_owned(),
    }]))
    // Err(ApiError::BadRequest)
}

// curl "http://127.0.0.1:3000/"
// curl "http://127.0.0.1:3000/hello"
// curl "http://127.0.0.1:3000/hello1"
// curl "http://127.0.0.1:3000/p/123"
// curl "http://127.0.0.1:3000/j1"
// curl "http://127.0.0.1:3000/j2"
// curl "http://127.0.0.1:3000/j3"
// curl "http://127.0.0.1:3000/j4"
// curl "http://127.0.0.1:3000/my"
// curl -X GET http://127.0.0.1:3000/foo
// curl -X POST http://127.0.0.1:3000/foo
// curl -H "Content-Type: application/json" -d '{"username":"someName"}' -X POST http://127.0.0.1:3000/users
