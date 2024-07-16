use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::client::CLIENT;

async fn start() -> impl Responder {
    let path = "docker-compose.yml";
    let mut client = CLIENT.lock().unwrap();

    println!("Server Status: {}", client.server_status);
    if client.server_status != "stopped" {
        return HttpResponse::BadRequest().json(json!("Server already started"));
    }

    client.attach_docker(path).unwrap();
    
    let docker_compose = client.docker_compose.as_mut().unwrap();

    match docker_compose.start() {
        Ok(_) => {
            docker_compose.get_container_id("mc".to_string()).unwrap();
            docker_compose.get_container_ip().unwrap();
            client.server_status = "starting".to_string();
            HttpResponse::Ok().json(json!("Server started"))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(json!(format!("{:?}", err)))
        }
        
    }
}

async fn stop() -> impl Responder {
    let mut client = CLIENT.lock().unwrap();

    if client.server_status != "running" {
        return HttpResponse::BadRequest().json(json!("Server already stopped"));
    }
    
    let docker_compose = client.docker_compose.as_mut().unwrap();

    match docker_compose.stop() {
        Ok(_) => {
            client.server_status = "stopped".to_string();
            HttpResponse::Ok().json(json!("Server stopped"))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(json!(format!("{:?}", err)))
        }
        
    }
}

pub fn server(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/server")
            .route("start", web::get().to(start))
            .route("stop", web::get().to(stop))
    );
}