use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Numbers {
    num1: i32,
    num2: i32,
}

async fn add(numbers: web::Json<Numbers>) -> impl Responder {
    let result = numbers.num1 + numbers.num2;
    format!("Sum: {}", result)
}

async fn health_check() -> impl Responder {
    "Ok"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/health_check").route(web::get().to(health_check)))
            .service(web::resource("/add").route(web::post().to(add)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
