// Actix Web provides a set of error helper functions that are useful
// for generating specific HTTP error codes from other errors. Here we
// convert MyError, which doesn't implement the ResponseError trait,
// to a 400 (bad request) using map_err.

use actix_web::{error, get, App, HttpServer};

#[derive(Debug)]
struct MyError {
    name: &'static str,
}

#[get("/index")]
async fn index() -> actix_web::Result<String> {
    let result = Err(MyError { name: "test error" });

    result.map_err(|err| error::ErrorBadRequest(err.name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
