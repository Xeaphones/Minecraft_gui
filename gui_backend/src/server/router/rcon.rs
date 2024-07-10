use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;
use std::error::Error;
use crate::client::CLIENT;

#[derive(Deserialize)]
struct Info {
    command: String,
}

async fn command(info: web::Json<Info>) -> Result<String, Box<dyn Error>> {
    let mut client = CLIENT.lock().unwrap();

    // if no rconclient
    if client.rcon_client.is_none() {
        client.attach_rcon().await?;
    }

    let rcon_client = client.rcon_client.as_mut().unwrap();
    if !rcon_client.is_logged_in() {
        let _ = rcon_client.authenticate().await?;
    }

    match rcon_client.send_command(info.command.to_string()).await {
        Ok(resp) => { return Ok(format!("{}", resp)) },
        Err(err) => { return Err(err) },
    }
}

pub fn rcon(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rcon")
            .route("command", web::post().to(command))
    );
}