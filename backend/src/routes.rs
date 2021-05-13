use actix_web::{get, web, web::ServiceConfig, HttpResponse, Responder};
use actix_web::body::Body;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use serde::Serialize;

pub (crate) fn init(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("/").route(web::get().to(index)))
        .service(
            web::scope("/")
                .service(ping)
                .service(
                    web::scope("/api/v1")
                        .service(ping),
                )
                .service(web::resource("/{_:.*}").route(web::get().to(frontend)))
        )
    ;
}

#[derive(RustEmbed)]
#[folder = "../static"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

fn index() -> HttpResponse {
    handle_embedded_file("index.html")
}

fn frontend(path: web::Path<String>) -> HttpResponse {
    handle_embedded_file(&path.0)
}

#[derive(Serialize, Debug)]
struct Pong {
    status: String,
    code: i16,
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(Pong {
        status: "ok".to_string(),
        code: 200,
    })
}