use actix_web::{get, App, HttpServer, Responder};

#[get("/health")]
async fn greet() -> impl Responder {
    "ok"
}

#[actix_web::main] // or #[tokio::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
