use actix_web::{get, App, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
    "Hello world!"
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(("127.0.0.1", 3013))?
        .run()
        .await
}