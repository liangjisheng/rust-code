// HttpServer accepts an application factory rather than an application instance.
// An HttpServer constructs an application instance for each thread. Therefore,
// application data must be constructed multiple times. If you want to share data
// between different threads, a shareable object should be used, e.g. Send + Sync.

// Internally, web::Data uses Arc. So in order to avoid creating two Arcs, we should
// create our Data before registering it using App::app_data().

// In the following example, we will write an application with mutable, shared state.
// First, we define our state and create our handler

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// curl "http://127.0.0.1:8080/"

// State initialized inside the closure passed to HttpServer::new
// is local to the worker thread and may become de-synced if modified.
// To achieve globally shared state, it must be created outside of
// the closure passed to HttpServer::new and moved/cloned in.
