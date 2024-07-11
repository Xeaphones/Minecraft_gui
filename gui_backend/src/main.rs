mod docker_compose;
mod server;
mod client;

use serde_json;
use docker_compose::DockerCompose;
use server::start_server;
use client::CLIENT;

use sysinfo::{System, SystemExt, ProcessorExt, ComponentExt};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

async fn get_server_status() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "Server is running" }))
}

async fn get_cpu_usage() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_processor_info().cpu_usage();
    HttpResponse::Ok().json(json!({ "cpu": cpu_usage }))
}

async fn get_ram_usage() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    HttpResponse::Ok().json(json!({ "total_memory": total_memory, "used_memory": used_memory }))
}


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

    // Configurer le serveur Actix-web avec les routes API
    let http_server = HttpServer::new(|| {
        App::new()
            .route("/api/status", web::get().to(get_server_status))
            .route("/api/cpu", web::get().to(get_cpu_usage))
            .route("/api/ram", web::get().to(get_ram_usage))
    })
    .bind(&bind_addr)?
    .run();

    // Attendre le serveur HTTP
    tokio::spawn(http_server);

    // Démarrer le serveur Minecraft
    let mc_server = start_server(&bind_addr);

    println!();
    match mc_server.await {
        Ok(_) => println!("Minecraft Server terminated cleanly"),
        Err(err) => println!("Minecraft Server terminated with an error!.\nErr: {:?}", err),
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