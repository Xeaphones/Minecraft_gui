mod rcon;

use actix_web::{web, HttpResponse, Responder};

use rcon::rcon;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(hello))
            .route(web::post().to(echo))
    )
    .service(web::resource("/hey").route(web::get().to(manual_hello)))
    .configure(rcon);
}