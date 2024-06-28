use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;
use std::error::Error;
use crate::rcon_client::RCON_CLIENT;

#[derive(Deserialize)]
struct Info {
    command: String,
}

async fn command(info: web::Json<Info>) -> Result<String, Box<dyn Error>> {
    let mut client = RCON_CLIENT.lock().unwrap();

    if !client.is_logged_in() {
        let _ = client.authenticate("127.0.0.1:25575".to_string());
    }

    match client.send_command(info.command.to_string()) {
        Ok(resp) => { Ok(format!("{}", resp.body)) },
        Err(err) => { Err(err) },
    }
}

pub fn rcon(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rcon")
            .route("command", web::post().to(command))
    );
}