// Actix Web provides a facility for type-safe request information access called extractors
// (i.e., impl FromRequest). There are lots of built-in extractor implementations (see implementors).

// An extractor can be accessed as an argument to a handler function.
// Actix Web supports up to 12 extractors per handler function.
// Argument position does not matter.

use actix_web::{get, web, App, HttpRequest, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyInfo {
    id: i32,
    username: String,
}

async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}

// curl -X POST "http://127.0.0.1:8080/index/path1/path2" -d '{ "id":1, "username":"ljs" }' -H "Content-Type: application/json"

// Path provides information that is extracted from the request's path.
// Parts of the path that are extractable are called "dynamic segments"
// and are marked with curly braces. You can deserialize any variable
// segment from the path.

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn user_friend(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

// curl "http://127.0.0.1:8080/users/1/ljs"

// It is also possible to extract path information to a type that implements
// the Deserialize trait from serde by matching dynamic segment names with
// field names. Here is an equivalent example that uses serde instead of a tuple type.
#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info using serde
#[get("/users1/{user_id}/{friend}")] // <- define path parameters
async fn user_friend1(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}

// curl "http://127.0.0.1:8080/users1/1/ljs"

// As a non-type-safe alternative, it's also possible to query
// (see match_info docs) the request for path parameters by name within a handler:
#[get("/users2/{user_id}/{friend}")] // <- define path parameters
async fn user_friend2(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}

// curl "http://127.0.0.1:8080/users2/2/ljs2"

// The Query<T> type provides extraction functionality for the request's
// query parameters. Underneath it uses serde_urlencoded crate.

#[derive(Deserialize)]
struct Info1 {
    username: String,
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/query")]
async fn query(info: web::Query<Info1>) -> String {
    format!("Welcome {}!", info.username)
}

// curl "http://127.0.0.1:8080/query?username=ljs"

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(user_friend)
            .service(user_friend1)
            .service(user_friend2)
            .service(query)
            .route("/index/{path1}/{path2}", web::post().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
