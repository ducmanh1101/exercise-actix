use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Numbers {
    num1: i32,
    num2: i32,
}

#[post("/add")]
async fn add(numbers: web::Json<Numbers>) -> impl Responder {
    let result = numbers.num1 + numbers.num2;
    format!("Sum: {}", result)
}

#[get("/health")]
async fn health_check() -> impl Responder {
    "Ok"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check).service(add))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
