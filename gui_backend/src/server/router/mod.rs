mod rcon;
mod query;

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sysinfo::{System, SystemExt, ProcessorExt};

use rcon::rcon;
use query::query;

async fn get_minecraft_port() -> impl Responder {
    HttpResponse::Ok().json(json!({ "port": crate::client::CLIENT.lock().unwrap().get_minecraft_port() }))
}

async fn get_server_status() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "running" }))
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

    let total_memory = sys.total_memory() as f64;
    let used_memory = sys.used_memory() as f64;
    let ram_usage = (used_memory / total_memory) * 100.0;

    HttpResponse::Ok().json(json!({ "ram": ram_usage }))
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

fn configure_default_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(hello))
            .route(web::post().to(echo))
    );
}

fn configure_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/minecraft-port").route(web::get().to(get_minecraft_port)))
       .service(web::resource("/cpu").route(web::get().to(get_cpu_usage)))
       .service(web::resource("/ram").route(web::get().to(get_ram_usage)))
       .service(web::resource("/status").route(web::get().to(get_server_status)))
       .service(web::resource("/hey").route(web::get().to(manual_hello)));
}

pub fn route(cfg: &mut web::ServiceConfig) {
    configure_default_routes(cfg);
    configure_api_routes(cfg);
    cfg.configure(rcon);
    cfg.configure(query);
}
