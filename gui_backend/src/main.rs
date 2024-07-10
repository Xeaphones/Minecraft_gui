mod docker_compose;
mod server;
mod client;

use serde_json;
use docker_compose::DockerCompose;
use server::start_server;
use client::CLIENT;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "docker-compose.yml";
    let mut docker_compose = DockerCompose::new(path)?;

    // Adding a new service
    let mc_service = serde_json::json!({
        "image": "itzg/minecraft-server",
        "tty": true,
        "stdin_open": true,
        "volumes": ["./data:/data"],
        "environment": serde_json::json!({
            "EULA": "true",
            "TYPE": "VANILLA",
            "VERSION": "LATEST",
            "MEMORY": "1G",
            "LOG_TIMESTAMP": "true", 
        })
    });
    docker_compose.set_service("mc", mc_service);
    docker_compose.set_value("mc", "ports", serde_json::json!(["25565:25565","25575:25575"]))?;
    {
        let client = CLIENT.lock().unwrap();
        docker_compose.set_env("mc", "RCON_PASSWORD", &client.get_rcon_password())?;
    }

    // Getting basic stats
    {
        let client = CLIENT.lock().unwrap();
        match client.get_basic_stats().await {
            Ok(stats) => println!("Basic Stats: {:?}", stats),
            Err(err) => println!("Error getting basic stats: {:?}", err),
        }

        // Getting full stats
        match client.get_full_stats().await {
            Ok(stats) => println!("Full Stats: {:?}", stats),
            Err(err) => println!("Error getting full stats: {:?}", err),
        }
    }

    // Getting a value from the nginx service
    // if let Some(value) = docker_compose.get_value("mc", "environment") {
    //     println!("Env: {:#?}", value);
    // }

    // Getting a service
    // if let Some(service) = docker_compose.get_service("mc") {
    //     println!("Service found: {:#?}", service);
    // } else {
    //     println!("Service not found");
    // }


    // Saving changes back to the file
    docker_compose.save()?;
    docker_compose.start()?;

    let bind_addr = "127.0.0.1:8080";
    let server = start_server(&bind_addr);

    println!();
    match server.await {
        Ok(_) => println!("Server terminated cleanly"),
        Err(err) => println!("Server terminated with an error!.\nErr: {:?}", err),
    }

    // Disconnect cleanly when finished.   
    {
        let mut client = CLIENT.lock().unwrap();
    
        if let Some(rcon_client) = client.rcon_client.take() {
            rcon_client.close().await?;
        }
    }
    docker_compose.stop()?;

    Ok(())
}