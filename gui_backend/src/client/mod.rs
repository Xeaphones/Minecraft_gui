use docker_compose::DockerCompose;
use mc_query::query::{stat_basic, stat_full, BasicStatResponse, FullStatResponse};
use rcon::RconClient;
use lazy_static::lazy_static;
use std::{path::Path, sync::Mutex};
use std::error::Error;
use tokio::time::{timeout, Duration};

pub(crate) mod rcon;
pub(crate) mod docker_compose;

pub struct Client {
    pub address: String,
    pub rcon_client: Option<RconClient>,
    pub docker_compose: Option<DockerCompose>,
    rcon_password: String,
    minecraft_port: u16,
    pub server_status: String,
}

pub enum StatResponse {
    Basic(BasicStatResponse),
    Full(FullStatResponse),
}

impl Client {
    pub fn new(address: String, minecraft_port: u16) -> Self {
        Client {
            address: address.clone(),
            rcon_client: None,
            docker_compose: None,
            rcon_password: RconClient::generate_password(16),
            minecraft_port,
            server_status: "stopped".to_string(),
        }
    }

    pub fn get_minecraft_port(&self) -> u16 {
        self.minecraft_port
    }
    
    pub async fn attach_rcon(&mut self) -> Result<(), Box<dyn Error>> {
        let rcon_client = RconClient::new(&self.address, &self.rcon_password).await?;
        self.rcon_client = Some(rcon_client);
        Ok(())
    }

    pub fn attach_docker<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>> {
        let mut docker_compose = DockerCompose::new(path)?;

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
                "ENABLE_QUERY": "true",
                "ENABLE_RCON": "false",
                "CREATE_CONSOLE_IN_PIPE": "true",
            })
        });
        docker_compose.set_service("mc", mc_service);
        docker_compose.set_value("mc", "ports", serde_json::json!([format!("{}:25565", self.get_minecraft_port()), "25575:25575"]))?;
        docker_compose.set_env("mc", "RCON_PASSWORD", &self.get_rcon_password())?;

        docker_compose.save()?;
        self.docker_compose = Some(docker_compose);
        Ok(())
    }

    pub fn get_rcon_password(&self) -> String {
        self.rcon_password.clone()
    }

    pub async fn get_stats(&self, stat: String) -> Result<StatResponse, Box<dyn Error>> {
        if !self.docker_compose.is_some() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Docker Compose not set")));
        }

        match stat.as_str() {
            "full" => self.get_full_stats().await.map(|stats| StatResponse::Full(stats)),
            _ => self.get_basic_stats().await.map(|stats| StatResponse::Basic(stats)),
        }
    }

    // Fonction pour obtenir les statistiques de base
    async fn get_basic_stats(&self) -> Result<BasicStatResponse, Box<dyn Error>> {
        if !self.docker_compose.is_some() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Docker Compose not set")));
        }

        let docker_compose = self.docker_compose.as_ref().unwrap();

        let result = timeout(Duration::from_secs(3), stat_basic(&docker_compose.docker_ip, self.minecraft_port)).await?;

        match result {
            Ok(stats) => {
                Ok(stats)
            },
            Err(_) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Operation timed out"))),
        }
    }

    
    // Fonction pour obtenir les statistiques complètes
    async fn get_full_stats(&self) -> Result<FullStatResponse, Box<dyn Error>> {
        if !self.docker_compose.is_some() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Docker Compose not set")));
        }

        let docker_compose = self.docker_compose.as_ref().unwrap();
        
        let result = timeout(Duration::from_secs(3), stat_full(&docker_compose.docker_ip, self.minecraft_port)).await?;

        match result {
            Ok(stats) => {
                Ok(stats)
            },
            Err(_) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Operation timed out"))),
        }
    }
}

lazy_static! {
    pub static ref CLIENT: Mutex<Client> = Mutex::new(Client::new("localhost".to_string(), 25565));
}