use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use ocpc::media::{
    jrtt::jrtt::jrtt,
    media
};

#[get("/")]
async fn hello() -> impl Responder {
    media();
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(jrtt)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}