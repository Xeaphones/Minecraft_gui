mod server;
mod client;

use server::start_server;
use client::CLIENT;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "docker-compose.yml";

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

    {
        let mut client = CLIENT.lock().unwrap();

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
    {
        let mut client = CLIENT.lock().unwrap();
    
        if let Some(rcon_client) = client.rcon_client.take() {
            rcon_client.close().await?;
        }

        if let Some(docker_compose) = client.docker_compose.take() {

            match docker_compose.get_container_stats().await {
                Ok(stats) => {
                    println!("CPU Usage: {:?}", stats.cpu_stats.cpu_usage.total_usage);
                    println!("Memory Usage: {:?}", stats.memory_stats.usage);
                }
                Err(e) => {
                    eprintln!("Error getting container stats: {}", e);
                }
            }

            docker_compose.stop()?;
        }
    }

    Ok(())
}