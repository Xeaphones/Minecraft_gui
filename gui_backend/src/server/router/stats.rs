use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::client::CLIENT;

async fn get_stats() -> impl Responder {
    let client = CLIENT.lock().unwrap();
    let status = client.server_status.clone();

    if client.docker_compose.is_none() {
        return HttpResponse::InternalServerError().json(json!({"status": status, "error": "Docker Compose not attached"}));
    }

    let docker_compose = client.docker_compose.as_ref().unwrap();

    match docker_compose.get_container_stats().await {
        Ok(stats) => {
            let cpu_usage = json!({
                "total_usage": stats.cpu_stats.cpu_usage.total_usage,
                "system_cpu_usage": stats.cpu_stats.system_cpu_usage,
                "online_cpus": stats.cpu_stats.online_cpus,
            });
            let memory_usage = json!({
                "usage": stats.memory_stats.usage,
                "limit": stats.memory_stats.limit,
            });
            
            HttpResponse::Ok().json(json!({"status": status, "cpu":  cpu_usage, "memory": memory_usage}))
        },
        Err(err) => {
            HttpResponse::Ok().json(json!({"status": status, "error": format!("{:?}", err)}))
        }
    }
}

pub fn stats(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/stats")
            .route(web::get().to(get_stats))
    );
}