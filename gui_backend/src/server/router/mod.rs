mod rcon;
mod query;
mod stats;

use actix_web::{web, HttpResponse, Responder, Error};
use futures_util::stream::StreamExt;
use serde::Deserialize;
use tokio_stream::wrappers::ReceiverStream;
use serde_json::json;
use tokio::sync::mpsc;
use crate::client::{docker_compose, CLIENT};

use rcon::rcon;
use query::query;
use stats::stats;

#[derive(Deserialize)]
struct Info {
    command: String,
}

async fn get_minecraft_port() -> impl Responder {
    HttpResponse::Ok().json(json!({ "port": CLIENT.lock().unwrap().get_minecraft_port() }))
}

async fn stream_logs() -> impl Responder {
    let (log_tx, log_rx) = mpsc::channel(100);
    let log_stream = ReceiverStream::new(log_rx);

    let docker_compose = {
        let client = CLIENT.lock().unwrap();
        client.docker_compose.clone().unwrap()
    };

    actix_web::rt::spawn(async move {
        match docker_compose.stream_container_logs().await {
            Ok(mut logs) => {
                while let Some(log_chunk) = logs.next().await {
                    match log_chunk {
                        Ok(data) => {
                            let log_message = std::str::from_utf8(&data).unwrap_or("Invalid UTF-8 sequence");
                            {
                                let mut client = CLIENT.lock().unwrap();

                                if log_message.contains("Starting net.minecraft.server.Main") && client.server_status != "running" {
                                    client.server_status = "starting".to_string();
                                } else if log_message.contains("Server thread/INFO]: Done") {
                                    client.server_status = "running".to_string();
                                } else if log_message.contains("[Server thread/INFO]: Stopping server") {
                                    client.server_status = "stopping".to_string();
                                } else if log_message.contains("INFO    mc-server-runner        Done") {
                                    client.server_status = "stopped".to_string();
                                }
                            }

                            if !log_message.contains("/INFO") {
                                continue;
                            }

                            if log_tx.send(Ok(log_message.to_string())).await.is_err() {
                                eprintln!("Log channel closed unexpectedly");
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error streaming logs: {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to stream logs: {:?}", e);
            }
        }
    });

    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .streaming(log_stream.map(|msg: Result<std::string::String, Error>| {
            Ok::<_, actix_web::Error>(web::Bytes::from(format!("data: {}\n\n", msg.unwrap())))
        }))
}

async fn command(info: web::Json<Info>) -> impl Responder {
    let client = CLIENT.lock().unwrap();
    
    if client.server_status != "running" {
        return HttpResponse::BadRequest().json(json!({ "error": "Server is not running" }));
    }

    let docker_compose = client.docker_compose.clone().unwrap();

    match docker_compose.send_command(info.command.clone()).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "response": "Command sent" })),
        Err(e) => {
            eprintln!("Failed to send command: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({ "error": "Failed to send command" }));
        }
    }
}

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.configure(rcon);
    cfg.configure(query);
    cfg.configure(stats);
    cfg.service(
        web::resource("/port")
            .route(web::get().to(get_minecraft_port))
    );
    cfg.service(
        web::resource("/logs")
            .route(web::get().to(stream_logs))
    );
    cfg.service(
        web::resource("/command")
            .route(web::post().to(command))
    );
}
