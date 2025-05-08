use actix_web::{App, HttpResponse, HttpServer, Responder, web::Path};
use clap::{Arg, command, value_parser};
use mime_guess::from_path;
use rust_embed::Embed;
use std::io::Result;

#[derive(Embed)]
#[folder = "frontend/out"]
struct Frontend;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Frontend::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

#[actix_web::get("/{_:.*}")]
async fn dist(path: Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

#[actix_web::main]
async fn main() -> Result<()> {
    let matches = command!()
        .arg(
            Arg::new("host")
                .long("host")
                .short('n')
                .default_value("127.0.0.1")
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .default_value("8000")
                .value_parser(value_parser!(usize)),
        )
        .get_matches();

    let host = matches.get_one::<String>("host").clone().unwrap().clone();
    let port = matches.get_one::<usize>("port").clone().unwrap().clone();
    let addr = format!("{}:{}", host, port);

    println!("Listening on http://{}", addr);

    HttpServer::new(|| App::new().service(index).service(dist))
        .bind(addr)?
        .run()
        .await
}
