use actix_web::{get, web ,App, HttpServer};

// This struct represents the state
struct AppState {
    app_name: String,
    next_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // get app_name
    let next_name = &data.next_name; // get next_name
    format!("Hello {} with {}!", app_name, next_name) // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
                next_name: String::from("Next-web"),
            })
            .service(index)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
