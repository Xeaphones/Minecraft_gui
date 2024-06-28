mod docker_compose;
mod server;

use serde_json;
use rand::Rng;
use minecraft_client_rs::Client;
use std::time::Duration;
use std::thread;

use docker_compose::DockerCompose;
use server::start_server;

fn generate_password(length: usize) -> String {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#@";

    let mut rng = rand::thread_rng();
    let random_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars.chars().nth(idx).unwrap()
        })
        .collect();

    random_string
}

async fn connect_rcon(password: String) -> Client {
    let mut client;
    loop {
        client = Client::new("127.0.0.1:25575".to_string()).unwrap();
        match client.authenticate(password.to_string()) {
            Ok(_) => { 
                println!("Authenticated"); 
                break;
            },
            Err(err) => { println!("Authentication Error.\nErr: {:?}", err); },
        }
        thread::sleep(Duration::from_secs(10));
    }
    client
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "docker-compose.yml";
    let mut docker_compose = DockerCompose::new(path)?;
    let rcon_password = generate_password(16);

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
    docker_compose.set_env("mc", "RCON_PASSWORD", &rcon_password)?;

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

    // Create a new client and connect to the server.
    let mut client = connect_rcon(rcon_password).await;

    match client.send_command("seed".to_string()) {
        Ok(resp) => { println!("{}", resp.body); }, // "Seed: [1871644822592853811]"
        Err(err) => { println!("Command Error.\nErr: {:?}", err);},
    }

    match client.send_command("list".to_string()) {
        Ok(resp) => { println!("{}", resp.body); }, // "Seed: [1871644822592853811]"
        Err(err) => { println!("Command Error.\nErr: {:?}", err);},
    }

    let bind_addr = "127.0.0.1:8080";
    let server = start_server(&bind_addr);

    println!();
    match server.await {
        Ok(_) => println!("Server terminated cleanly"),
        Err(err) => println!("Server terminated with an error!.\nErr: {:?}", err),
    }

    // Disconnect cleanly when finished.
    client.close().unwrap();
    docker_compose.stop();

    Ok(())
}