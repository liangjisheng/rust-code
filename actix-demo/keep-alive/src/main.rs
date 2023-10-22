use actix_web::{get, http, http::KeepAlive, App, HttpRequest, HttpResponse, HttpServer};
use std::time::Duration;

#[get("/")]
async fn index(_req: HttpRequest) -> HttpResponse {
    let mut resp = HttpResponse::Ok()
        .force_close() // <- Close connection on HttpResponseBuilder
        .finish();

    // Alternatively close connection on the HttpResponse struct
    resp.head_mut()
        .set_connection_type(http::ConnectionType::Close);

    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set keep-alive to 75 seconds
    let _one = HttpServer::new(|| App::new().service(index)).keep_alive(Duration::from_secs(75));

    // Use OS's keep-alive (usually quite long)
    let _two = HttpServer::new(|| App::new().service(index)).keep_alive(KeepAlive::Os);

    // Disable keep-alive
    let _three = HttpServer::new(|| App::new().service(index)).keep_alive(None);

    Ok(())
}
