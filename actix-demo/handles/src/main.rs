// By default Actix Web provides Responder implementations for some
// standard types, such as &'static str, String, etc.

use actix_web::{
    body::BoxBody, get, http::header::ContentType, web, App, Either, Error, HttpRequest,
    HttpResponse, HttpServer, Responder,
};
use futures::{future::ok, stream::once};
use serde::Serialize;

#[get("/index")]
async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

#[get("/index1")]
async fn index1(_req: HttpRequest) -> String {
    "Hello world!".to_owned()
}

// You can also change the signature to return impl Responder
// which works well if more complex types are involved.

#[get("/index2")]
async fn index2(_req: HttpRequest) -> impl Responder {
    web::Bytes::from_static(b"Hello world!")
}

// To return a custom type directly from a handler function,
// the type needs to implement the Responder trait.

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/index3")]
async fn index3() -> impl Responder {
    MyObj { name: "user" }
}

// Response body can be generated asynchronously. In this case,
// body must implement the stream trait Stream<Item = Result<Bytes, Error>>
#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

// curl "http://127.0.0.1:8080/stream"

// Sometimes, you need to return different types of responses. For example,
// you can error check and return errors, return async responses, or any
// result that requires two different types.

// For this case, the Either type can be used. Either allows combining two
// different responder types into a single type.

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

#[get("/index4")]
async fn index4() -> RegisterResult {
    if is_a_variant() {
        // choose Left variant
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        // choose Right variant
        Either::Right(Ok("Hello!"))
    }
}

fn is_a_variant() -> bool {
    false
}

// curl "http://127.0.0.1:8080/index4"

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(index1)
            .service(index2)
            .service(index3)
            .service(stream)
            .service(index4)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
