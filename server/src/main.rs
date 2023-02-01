use actix_web::{get, App, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 80))?
        .run()
        .await
}
