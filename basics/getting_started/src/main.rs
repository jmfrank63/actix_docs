use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, };

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let factory = move || {
        App::new()
            .configure(app_config)
    };
    HttpServer::new(factory)
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

fn app_config(service_config: &mut web::ServiceConfig) {
    service_config
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello));
}
