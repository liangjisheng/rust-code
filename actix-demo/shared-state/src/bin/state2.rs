// Application State Extractor

// Application state is accessible from the handler with the web::Data extractor;
// however, state is accessible as a read-only reference. If you need mutable
// access to state, it must be implemented.

// Here is an example of a handler that stores the number of processed requests

use actix_web::{web, App, HttpServer, Responder};
use std::cell::Cell;

#[derive(Clone)]
struct AppState {
    count: Cell<usize>,
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.get())
}

async fn add_one(data: web::Data<AppState>) -> impl Responder {
    let count = data.count.get();
    data.count.set(count + 1);

    format!("count: {}", data.count.get())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppState {
        count: Cell::new(0),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .route("/", web::to(show_count))
            .route("/add", web::to(add_one))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// curl "http://127.0.0.1:8080/"
// curl "http://127.0.0.1:8080/add"

// Although this handler will work, data.count will only count the number of
// requests handled by each worker thread. To count the number of total requests
// across all threads, one should use shared Arc and atomics.
