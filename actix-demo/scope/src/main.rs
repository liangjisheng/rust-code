// The web::scope() method allows setting a resource group prefix.

use actix_web::{get, web, App, HttpServer};

#[get("/show")]
async fn show_users() -> String {
    String::from("users")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let scope = web::scope("/users").service(show_users);
        App::new().service(scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// curl "http://127.0.0.1:8080/users/show"
