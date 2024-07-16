mod server;
mod client;

use std::env;

use server::start_server;
use client::CLIENT;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    let path = "docker-compose.yml";

    {
        let mut client = CLIENT.lock().unwrap();
        client.server_status = "stopped".to_string();

        client.attach_docker(path)?;

        let docker_compose = client.docker_compose.as_mut().unwrap();
        docker_compose.start()?;

        let _ = docker_compose.get_container_id("mc".to_string());
        let _ = docker_compose.get_container_ip();

        println!("Container: {:#?}", docker_compose);
    }

    let bind_addr = "127.0.0.1:8080";
    let server = start_server(&bind_addr);

    println!();
    match server.await {
        Ok(_) => println!("Minecraft Server terminated cleanly"),
        Err(err) => println!("Minecraft Server terminated with an error!.\nErr: {:?}", err),
    }
    
    // Disconnect cleanly when finished.   
    // {
    //     let mut client = CLIENT.lock().unwrap();
    
    //     if let Some(rcon_client) = client.rcon_client.take() {
    //         rcon_client.close().await?;
    //     }

    //     if let Some(docker_compose) = client.docker_compose.take() {
    //         docker_compose.stop()?;
    //     }
    // }

    Ok(())
}