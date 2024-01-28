use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

// 共享状态结构体
#[derive(Debug)]
struct AppState {
    // ...
    state: AtomicUsize,
}

// 方法1: 使用 State 状态提取器
async fn handler_as_state_extractor(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // ...
    state.state.fetch_add(1, Ordering::SeqCst); //请求一次 state 的值递增1
    format!("State extract shared_state: {:?}", state)
}

// 方法2: 使用 Extension 请求扩展提取器
// 这种方法的缺点是，如果尝试提取一个不存在的扩展，可能是因为忘记添加中间件，
// 或者因为提取了错误的类型，那么将得到运行时错误(特别是500 Internal Server Error 响应)。
async fn handler_as_extension_extractor(
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    // ...
    state.state.fetch_add(1, Ordering::SeqCst); //请求一次 state 的值递增1
    format!("Extension extract shared_state: {:?}", state)
}

// 方法3: 使用闭包捕获（closure captures）直接传递给处理器
async fn get_user(Path(user_id): Path<String>, state: Arc<AppState>) -> impl IntoResponse {
    // ...
    state.state.fetch_add(1, Ordering::SeqCst); //请求一次 state 的值递增1
    format!(
        "closure captures shared_state: {:?}, user_id {}",
        state, user_id
    )
}

// 注意添加应用状态不是强制的 - 仅在想要使用它时
async fn hello_front() -> &'static str {
    "Hello"
}

async fn do_something(State(state): State<AppState>) -> impl IntoResponse {
    // handle
}

#[tokio::main]
async fn main() {
    // 处理器共享状态（Sharing state with handlers）
    let shared_state = Arc::new(AppState {
        state: 0.into(), /* ... */
    });
    let shared_state_extension = Arc::clone(&shared_state);
    let shared_state_closure = Arc::clone(&shared_state);

    let app = Router::new()
        .route("/state", get(handler_as_state_extractor)) // 1.使用State提取器
        .with_state(shared_state)
        .route("/extension", get(handler_as_extension_extractor)) // 2.使用Extension提取器
        .layer(Extension(shared_state_extension))
        .route(
            "/users/:id",
            get({
                move |path| get_user(path, shared_state_closure) // 3.使用闭包捕获直接传递给处理器
            }),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// curl -X GET "http://127.0.0.1:3000/state"
// curl -X GET "http://127.0.0.1:3000/extension"
// curl -X GET "http://127.0.0.1:3000/users/111"
