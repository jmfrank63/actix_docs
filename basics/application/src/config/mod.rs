use actix_web::{get, web, HttpResponse, Responder};

async fn index() -> impl Responder {
    "Hello World!"
}

#[get("/index.html")]
async fn idx_service() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust!")
}

pub fn app_config(service_config: &mut web::ServiceConfig) {
    service_config.service(
        web::scope("/app")
            .service(idx_service)
            .route("/hello", web::get().to(index)),
    );
}
