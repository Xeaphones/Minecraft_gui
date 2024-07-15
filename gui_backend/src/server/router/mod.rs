mod rcon;
mod query;
mod stats;

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::client::CLIENT;

use rcon::rcon;
use query::query;
use stats::stats;

async fn get_minecraft_port() -> impl Responder {
    HttpResponse::Ok().json(json!({ "port": CLIENT.lock().unwrap().get_minecraft_port() }))
}

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.configure(rcon);
    cfg.configure(query);
    cfg.configure(stats);
    cfg.service(
        web::resource("/port")
            .route(web::get().to(get_minecraft_port))
    );
}
