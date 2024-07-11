use actix_web::{web, Result};
use std::error::Error;
use crate::client::{StatResponse, CLIENT};

async fn full() -> Result<String, Box<dyn Error>> {
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
        
                    Ok(json.to_string())
                },
                _ => {
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data")))
                }
            }
        },
        Err(err) => {
            Err(err)
        }
    }
}

async fn basic() -> Result<String, Box<dyn Error>> {
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
        
                    Ok(json.to_string())
                },
                _ => {
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data")))
                }
            }
        },
        Err(err) => {
            Err(err)
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