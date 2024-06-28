mod docker_compose;
mod server;
mod rcon_client;

use serde_json;

use docker_compose::DockerCompose;
use server::start_server;
use rcon_client::{RCON_CLIENT};

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
        let client = RCON_CLIENT.lock().unwrap();
        docker_compose.set_env("mc", "RCON_PASSWORD", &client.get_password())?;
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
        let mut client = RCON_CLIENT.lock().unwrap();
        client.close().unwrap();
    }
    docker_compose.stop()?;

    Ok(())
}