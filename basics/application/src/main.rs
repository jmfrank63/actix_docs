use actix_web::{App, HttpServer};
use config::app_config;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let factory = || App::new().configure(app_config);
    HttpServer::new(factory).bind("localhost:8080")?.run().await
}
