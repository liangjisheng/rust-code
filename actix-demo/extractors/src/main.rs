use actix_web::{
    error, get, http::header::ContentType, post, web, App, Error, HttpResponse, HttpServer,
    Responder, Result,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// extract `Info` using serde
async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

// curl -X POST "http://127.0.0.1:8080/" -d '{ "username":"alice" }' -H "Content-Type: application/json"

// You may also manually load the payload into memory and then deserialize it.

// In the following example, we will deserialize a MyObj struct. We need to
// load the request body first and then deserialize the json into an object.

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/index")]
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

async fn index1() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body("data")
}

#[get("/a/{name}")]
async fn index2(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
        number: 1,
    };
    Ok(web::Json(obj))
}

// curl "http://127.0.0.1:8080/a/alice"

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(index))
            .service(index_manual)
            .route("/index1", web::get().to(index1))
            .service(index2)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().service(index2)).await;
        let req = test::TestRequest::get()
            .uri("/a/alice")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[post("/a/{name}")]
    async fn index3(info: web::Json<Info>) -> Result<String> {
        Ok(format!("Welcome {}!", info.username))
    }

    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(App::new().service(index3)).await;
        let req = test::TestRequest::post().uri("/a/alice").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
