use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

use z2p::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}



