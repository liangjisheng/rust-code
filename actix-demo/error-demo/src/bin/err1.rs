use actix_web::{error, web, App, HttpServer, Result};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
    name: &'static str,
}

// ResponseError has a default implementation for error_response() that will
// render a 500 (internal server error), and that's what will happen when the
// index handler executes above

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

async fn index() -> Result<&'static str, MyError> {
    Err(MyError { name: "test" })
}

// curl "http://127.0.0.1:8080/index"

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/index", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
