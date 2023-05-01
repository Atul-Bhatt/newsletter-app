use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;
use web::Form;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
    phone: String,
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/")]
async fn home() -> impl Responder {
    "This is home."
}

#[post("/subscriptions")]
async fn subscribe(_form: Form<FormData>) -> impl Responder {
    "Running Cargo Check."
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(|| {
            App::new()
            .service(health_check)
            .service(subscribe)
        })
        .listen(listener)?
        .run();

    Ok(server)
}