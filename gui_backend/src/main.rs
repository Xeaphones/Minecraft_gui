mod docker_compose;
mod server;

use serde_json;

use docker_compose::DockerCompose;
use server::start_server;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "docker-compose.yml";
    let mut docker_compose = DockerCompose::new(path)?;

    // Adding a new service
    let new_service = serde_json::json!({
        "image": "nginx:latest",
        "ports": ["80:80"]
    });
    docker_compose.set_service("nginx", new_service);

    // Removing an existing service
    docker_compose.remove_service("nginx");

    // Getting a value from the nginx service
    if let Some(value) = docker_compose.get_value("mc", "image") {
        println!("Image: {:?}", value);
    }
    
    // Removing a value from the nginx service
    docker_compose.remove_value("mc", "ports")?;

    // Getting a service
    if let Some(service) = docker_compose.get_service("mc") {
        println!("Service found: {:?}", service);
    } else {
        println!("Service not found");
    }

    docker_compose.set_value("mc", "ports", serde_json::json!(["25565:25565"]))?;

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

    Ok(())
}