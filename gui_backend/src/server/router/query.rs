use actix_web::{web, Result, HttpResponse, Responder};
use std::error::Error;
use serde_json::json;
use crate::client::{StatResponse, CLIENT};

async fn full() -> impl Responder {
    let client = CLIENT.lock().unwrap();

    match client.get_stats("full".to_string()).await {
        Ok(stats) => {
            match stats {
                StatResponse::Full(stats) => {
                    let json = serde_json::json!({
                        "motd": stats.motd,
                        "game_type": stats.game_type,
                        "game_id": stats.game_id,
                        "version": stats.version,
                        "plugins": stats.plugins,
                        "map": stats.map,
                        "num_players": stats.num_players,
                        "max_players": stats.max_players,
                        "host_port": stats.host_port,
                        "host_ip": stats.host_ip,
                        "players": stats.players,
                    });
        
                    HttpResponse::Ok().json(json)
                },
                _ => {
                    HttpResponse::Ok().json(json!({"error": "Invalid data"}))
                }
            }
        },
        Err(err) => {
            HttpResponse::Ok().json(json!({"error": format!("{:?}", err)}))
        }
    }
}

async fn basic() -> impl Responder {
    let client = CLIENT.lock().unwrap();

    match client.get_stats("basic".to_string()).await {
        Ok(stats) => {
            match stats {
                StatResponse::Basic(stats) => {
                    let json = serde_json::json!({
                        "motd": stats.motd,
                        "game_type": stats.game_type,
                        "map": stats.map,
                        "num_players": stats.num_players,
                        "max_players": stats.max_players,
                        "host_port": stats.host_port,
                        "host_ip": stats.host_ip,
                    });
        
                    HttpResponse::Ok().json(json)
                },
                _ => {
                    HttpResponse::InternalServerError().json(json!({"error": "Invalid data"}))
                }
            }
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(json!({"error": format!("{:?}", err)}))
        }
    }
}

pub fn query(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/query")
            .route("full", web::get().to(full))
            .route("basic", web::get().to(basic))
    );
}