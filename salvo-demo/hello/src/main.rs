use salvo::prelude::*;

// The original format is:
#[handler]
async fn original_hello(
    _req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
) {
    res.render("Hello world");
}

// You can omit function arguments if they are not used, like _req, _depot, _ctrl in this example:
#[handler]
async fn omit_hello(res: &mut Response) {
    res.render("Hello world");
}

// Any type can be function handler's return value if it implements Writer.
// For example &str implements Writer and it will render string as plain text:

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

// The more common situation is we want to return a Result<T, E> to simplify error handling.
// If T and E implements Writer, Result<T, E> can be function handler's return type:
#[handler]
async fn result_hello(_res: &mut Response) -> Result<&'static str, ()> {
    // return Result
    Ok("Hello world")
}

#[handler]
async fn hello_zh() -> Result<&'static str, ()> {
    Ok("你好，世界！")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .get(hello)
        .push(Router::with_path("hello_zh").get(hello_zh));
    println!("{:?}", router);

    let acceptor = TcpListener::new("127.0.0.1:8080").bind().await;
    Server::new(acceptor).serve(router).await;
}

// curl "http://127.0.0.1:8080/"
// curl "http://127.0.0.1:8080/hello_zh"
