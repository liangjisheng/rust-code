use actix_web::{
    get, guard,
    guard::{Guard, GuardContext},
    http,
    http::header,
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

#[get("/show/{id}")]
async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[get("/a/{v1}/{v2}/")]
async fn index1(req: HttpRequest) -> Result<String> {
    let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
    let v2: u8 = req.match_info().query("v2").parse().unwrap();
    let (v3, v4): (u8, u8) = req.match_info().load().unwrap();
    Ok(format!("Values {} {} {} {}", v1, v2, v3, v4))
}

#[get("/{username}/{id}/index.html")] // <- define path parameters
async fn index2(info: web::Path<(String, u32)>) -> Result<String> {
    let info = info.into_inner();
    Ok(format!("Welcome {}! id: {}", info.0, info.1))
}

// Use the HttpRequest.url_for() method to generate URLs based on resource
// patterns. For example, if you've configured a resource with the name "foo"
// and the pattern "{a}/{b}/{c}", you might do this
#[get("/test/")]
async fn index3(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("foo", ["1", "2", "3"])?; // <- generate url for "foo" resource

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url.as_str()))
        .finish())
}

// curl -i "http://127.0.0.1:8080/test/1/2/3"

struct ContentTypeHeader;

impl Guard for ContentTypeHeader {
    fn check(&self, req: &GuardContext) -> bool {
        req.head()
            .headers()
            .contains_key(http::header::CONTENT_TYPE)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .service(
                web::resource("/user/{name}")
                    .name("user_detail")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(HttpResponse::Ok))
                    .route(web::put().to(HttpResponse::Ok)),
            )
            .service(
                // A Route can contain any number of guards but only one handler.
                // In this example, HttpResponse::Ok() is returned for GET requests
                // if the request contains Content-Type header, the value of this
                // header is text/plain, and path equals to /path.
                web::resource("/path").route(
                    web::route()
                        .guard(guard::Get())
                        .guard(guard::Header("content-type", "text/plain"))
                        .to(HttpResponse::Ok),
                ),
            )
            .service(
                web::scope("/users")
                    .service(show_users)
                    .service(user_detail),
            )
            .service(index1)
            .service(index2)
            .service(
                web::resource("/test/{a}/{b}/{c}")
                    .name("foo") // <- set resource name, then it could be used in `url_for`
                    .guard(guard::Get())
                    .to(HttpResponse::Ok),
            )
            .service(index3)
            .route("/", web::get().to(index))
            .route("/user", web::post().to(index))
            .route(
                "/guard",
                web::route().guard(ContentTypeHeader).to(HttpResponse::Ok),
            )
            .route(
                "/guard1",
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// curl "http://127.0.0.1:8080/"
// curl -X POST "http://127.0.0.1:8080/user"
// curl "http://127.0.0.1:8080/user/alice"
