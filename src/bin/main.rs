use actix_web::{web, App, HttpServer};
extern crate api;

#[actix_web::main] // or #[tokio::main]
pub async fn main() -> std::io::Result<()> {
    println!("listening on 0.0.0.0:8080");
    HttpServer::new(|| App::new().route("/health", web::get().to(api::healthcheck)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
