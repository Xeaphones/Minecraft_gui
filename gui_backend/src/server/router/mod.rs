use actix::prelude::*;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web_actors::ws;
use futures_util::StreamExt;
use hyper::{client, server};
use serde::de::value;
use std::error::Error;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use tokio;
use tokio::sync::{oneshot, watch};
use tokio::task;

use crate::client::{docker_compose, StatResponse, CLIENT};

#[derive(Serialize, Deserialize, Debug)]
struct MyMessage {
    status: String,
    content: JsonValue,
    content_type: String,
}

struct MyWebSocket {
    hb: Instant,
    log_streaming: bool,
}
impl MyWebSocket {
    fn new() -> Self {
        Self { 
            hb: Instant::now(),
            log_streaming: false,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::new(5, 0), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::new(10, 0) {
                println!("WebSocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn send_stats(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::new(3, 0), |_, ctx| {
            // Call the async function to get stats
            let addr = ctx.address();
            actix::spawn(async move {
                match get_stats().await {
                    Ok(stats) => {
                        match stats.get("error") {
                            Some(error) => {
                                let error_message = MyMessage {
                                    status: "error".to_string(),
                                    content_type: "docker_stats".to_string(),
                                    content: json!({
                                        "error": error.clone(),
                                        "status": stats.get("status").unwrap().clone(),
                                    }),
                                };
                                addr.send(error_message).await.unwrap();
                            },
                            None => {
                                let server_status_message = MyMessage {
                                    status: "ok".to_string(),
                                    content_type: "docker_stats".to_string(),
                                    content: stats.clone(),
                                };
                                match addr.send(server_status_message).await {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("Failed to send stats: {:?}", err);
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("Failed to get stats: {:?}", err);
                    }
                }
            });
        });
    }

    fn stream_logs(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        if self.log_streaming {
            return; // Don't recreate the task if it's already running
        }

        self.log_streaming = true;

        let addr = ctx.address();
        let docker_compose = {
            let client = CLIENT.lock().unwrap();

            if client.docker_compose.is_none() {
                self.log_streaming = false;
                return;
            }

            client.docker_compose.as_ref().unwrap().clone()
        };

        actix::spawn(async move {
            match docker_compose.stream_container_logs().await {
                Ok(mut log_stream) => {
                    while let Some(log_result) = log_stream.next().await {
                        match log_result {
                            Ok(log) => {
                                let _log = String::from_utf8_lossy(&log);

                                {
                                    let mut client = CLIENT.lock().unwrap();

                                    if _log.contains("Starting net.minecraft.server.Main") {
                                        client.server_status = "starting".to_string();
                                    } else if _log.contains("Server thread/INFO]: Done") {
                                        client.server_status = "running".to_string();
                                    } else if _log.contains("[Server thread/INFO]: Stopping server") {
                                        client.server_status = "stopping".to_string();
                                    } else if _log.contains("INFO    mc-server-runner        Done") {
                                        client.server_status = "stopped".to_string();
                                    }
                                }

                                if !_log.contains("/INFO") {
                                    continue;
                                }

                                let log_message = MyMessage {
                                    content_type: "log".to_string(),
                                    status: "ok".to_string(),
                                    content: json!(_log.trim()),
                                };
                                match addr.send(log_message).await {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("Failed to send log message: {:?}", err);
                                        break;
                                    }
                                };
                            }
                            Err(err) => {
                                println!("Error streaming logs: {:?}", err);
                                break;
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Failed to stream logs: {:?}", err);
                }
            }
        });
    }

    fn restart_log_stream(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        self.log_streaming = false;
        self.stream_logs(ctx);
    }

    fn send_full_stats(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::new(3, 0), |_, ctx| {
            let addr = ctx.address();

            let server_status = {
                let client = CLIENT.lock().unwrap();
                client.server_status.clone()
            };

            if server_status == "stopped" || server_status == "stopping" {
                return;
            }

            actix::spawn(async move {
                match full().await {
                    Ok(stats) => {
                        match stats.get("error") {
                            Some(error) => {
                                let error_message = MyMessage {
                                    status: "error".to_string(),
                                    content_type: "server_stats".to_string(),
                                    content: json!({
                                        "error": error.clone(),
                                    }),
                                };
                                addr.send(error_message).await.unwrap();
                            },
                            None => {
                                let full_stats_message = MyMessage {
                                    status: "ok".to_string(),
                                    content_type: "server_stats".to_string(),
                                    content: stats.clone(),
                                };
                                addr.send(full_stats_message).await.unwrap();
                            }
                        }
                    },
                    Err(err) => {
                        println!("Failed to get full stats: {:?}", err);
                    }
                }
            });
        });
    }
}

async fn get_stats() -> Result<JsonValue, actix_web::Error> {
    let status = {
        let client = CLIENT.lock().unwrap();
        client.server_status.clone()
    };

    {
        let mut client = CLIENT.lock().unwrap();
        if client.docker_compose.is_none() {
            client.server_status = "stopped".to_string();
            return Ok(json!({"status": status, "error": "Docker Compose not attached"}));
        }
    }

    let docker_compose = {
        let client = CLIENT.lock().unwrap();
        client.docker_compose.as_ref().unwrap().clone()
    };

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
            
            Ok(json!({"status": status, "cpu": cpu_usage, "memory": memory_usage}))
        },
        Err(err) => {
            Ok(json!({"status": "stopped", "error": format!("{:?}", err)}))
        }
    }
}

async fn full() -> Result<JsonValue, actix_web::Error> {
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
        
                    Ok(json)
                },
                _ => {
                    Ok(json!({"error": "Failed to get full stats"}))
                }
            }
        },
        Err(err) => {
            Ok(json!({"error": format!("{:?}", err)}))
        }
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection established");
        self.hb(ctx);
        self.send_stats(ctx);
        self.stream_logs(ctx);
        self.send_full_stats(ctx);

        let welcome_message = MyMessage {
            status: "ok".to_string(),
            content: json!("Welcome to the Minecraft Server!"),
            content_type: "welcome".to_string(),
        };
        let message_text = serde_json::to_string(&welcome_message).unwrap();
        ctx.text(message_text);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection closed");
        self.log_streaming = false; 
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                if let Ok(message) = serde_json::from_str::<MyMessage>(&text) {
                    println!("Received message: {:?}", message);
                    let response = MyMessage {
                        status: "ok".to_string(),
                        content: json!("Message received!"),
                        content_type: "response".to_string(),
                    };
                    let response_text = serde_json::to_string(&response).unwrap();
                    ctx.text(response_text);

                    if message.content_type == "console" {
                        let command = message.content.as_str().unwrap().to_string();

                        actix::spawn(async move {
                            let docker_compose = {
                                let client = CLIENT.lock().unwrap();
                                client.docker_compose.as_ref().unwrap().clone()
                            };

                            let _ = docker_compose.send_command(command).await;
                        });
                    } else if message.content_type == "command" {
                        let command = message.content.as_str().unwrap().to_string();

                        if command == "toggle" {
                            let addr = ctx.address();
                            actix::spawn(async move {
                                let status = {
                                    let client = CLIENT.lock().unwrap();
                                    client.server_status.clone()
                                };

                                match status.as_str() {
                                    "running" => {
                                        let docker_compose = {
                                            let client = CLIENT.lock().unwrap();
                                            client.docker_compose.as_ref().unwrap().clone()
                                        };
                                        let _ = docker_compose.stop();

                                        {
                                            let mut client = CLIENT.lock().unwrap();
                                            client.server_status = "stopped".to_string();
                                            client.detach_docker().unwrap();
                                        }
                                    },
                                    "stopped" => {
                                        {
                                            let mut client = CLIENT.lock().unwrap();
                                            client.attach_docker("docker-compose.yml").unwrap();
                                        }

                                        let mut client = CLIENT.lock().unwrap();
                                        let docker_compose = client.docker_compose.as_mut().unwrap();

                                        let _ = docker_compose.start();

                                        let _ = docker_compose.get_container_id("mc".to_string());
                                        let _ = docker_compose.get_container_ip();
                                    },
                                    _ => {
                                        println!("Server is in an invalid state: {}", status);
                                    }
                                }

                                let status_message = MyMessage {
                                    status: "ok".to_string(),
                                    content: json!("Toggled server"),
                                    content_type: "response".to_string(),
                                };
                                addr.send(status_message).await.unwrap();
                            });
                        }
                    } else if message.content_type == "restart_logs" {
                        self.restart_log_stream(ctx);
                    } else if message.content_type == "properties" {
                        let addr = ctx.address();
                        actix::spawn(async move {
                            let mut docker_compose = {
                                let client = CLIENT.lock().unwrap();
                                client.docker_compose.as_ref().unwrap().clone()
                            };

                            let properties = message.content.as_object().unwrap();

                            // for (key, value) in properties {}

                            for (key, value) in properties.iter() {
                                let value = value.clone().to_string().replace("\"", "");

                                let _ = docker_compose.set_env("mc", key.replace("-", "_").to_uppercase().as_str(), value);
                            
                            }
                            
                            let _ = docker_compose.save();
                        });
                    }

                } else {
                    println!("Failed to parse JSON message");
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl Message for MyMessage {
    type Result = ();
}

impl Handler<MyMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: MyMessage, ctx: &mut Self::Context) {
        let json_message = json!({
            "status": msg.status,
            "content": msg.content,
            "content_type": msg.content_type,
        });
        ctx.text(json_message.to_string());
    }
}

async fn ws_index(r: HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(MyWebSocket::new(), &r, stream)
}

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ws/")
            .route(web::get().to(ws_index))
    );
}
