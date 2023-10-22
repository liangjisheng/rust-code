use actix_web::{error, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

// Json<T> allows deserialization of a request body into a struct.
// To extract typed information from a request's body, the type T
// must implement serde::Deserialize.

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

// curl -X POST "http://127.0.0.1:8080/submit" -d '{ "username":"ljs" }' -H "Content-Type: application/json"

/// deserialize `Info` from request's body, max payload size is 4kb
async fn index(info: web::Json<Info>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

// curl -X POST "http://127.0.0.1:8080/" -d '{ "username":"ljs" }' -H "Content-Type: application/json"

// Some extractors provide a way to configure the extraction process.
// To configure an extractor, pass its configuration object to the
// resource's .app_data() method. In the case of Json extractor it
// returns a JsonConfig. You can configure the maximum size of the
// JSON payload as well as a custom error handler function.

#[derive(Deserialize)]
struct FormData {
    username: String,
}

/// extract form data using serde
/// this handler gets called only if the content type is *x-www-form-urlencoded*
/// and the content of the request could be deserialized to a `FormData` struct
#[post("/form")]
async fn form_data(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.username))
}

// curl -X POST "http://127.0.0.1:8080/form" -d 'username=ljs1' -H "Content-Type: application/x-www-form-urlencoded"

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            .service(
                web::resource("/")
                    // change json extractor configuration
                    .app_data(json_config)
                    .route(web::post().to(index)),
            )
            .service(submit)
            .service(form_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Actix Web also provides many other extractors, here's a few important ones:

// Data - For accessing pieces of application state.
// HttpRequest - HttpRequest is itself an extractor, in case you need access to other parts of the request.
// String - You can convert a request's payload to a String. An example is available in the rustdoc.
// Bytes - You can convert a request's payload into Bytes. An example is available in the rustdoc.
// Payload - Low-level payload extractor primarily for building other extractors.
// An example is available in the rustdoc.
