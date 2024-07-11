mod rcon;

use actix_web::{web, HttpResponse, Responder};

use rcon::rcon;

async fn get_server_status() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "Server is running" }))
}

async fn get_cpu_usage() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_processor_info().cpu_usage();
    HttpResponse::Ok().json(json!({ "cpu": cpu_usage }))
}

async fn get_ram_usage() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    HttpResponse::Ok().json(json!({ "total_memory": total_memory, "used_memory": used_memory }))
}

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