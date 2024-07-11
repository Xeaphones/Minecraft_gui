use mc_query::query::{stat_basic, stat_full, BasicStatResponse, FullStatResponse};
use rcon::RconClient;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::error::Error;
use tokio::time::{timeout, Duration};

pub(crate) mod rcon;

pub struct Client {
    pub address: String,
    pub rcon_client: Option<RconClient>,
    rcon_password: String,
    container_ip: String,
    minecraft_port: u16,
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
            rcon_password: RconClient::generate_password(16),
            container_ip: "empty".to_string(),
            minecraft_port,
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

    pub fn get_rcon_password(&self) -> String {
        self.rcon_password.clone()
    }

    pub fn set_container_address(&mut self, address: String) {
        self.container_ip = address;
    }

    pub async fn get_stats(&self, stat: String) -> Result<StatResponse, Box<dyn Error>> {
        match self.container_ip.as_str() {
            "empty" => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Container IP not set"))),
            _ => {},
        }

        match stat.as_str() {
            "full" => self.get_full_stats().await.map(|stats| StatResponse::Full(stats)),
            _ => self.get_basic_stats().await.map(|stats| StatResponse::Basic(stats)),
        }
    }

    // Fonction pour obtenir les statistiques de base
    async fn get_basic_stats(&self) -> Result<BasicStatResponse, Box<dyn Error>> {
        let result = timeout(Duration::from_secs(10), stat_basic(&self.container_ip, self.minecraft_port)).await?;

        match result {
            Ok(stats) => {
                Ok(stats)
            },
            Err(_) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Operation timed out"))),
        }
    }

    
    // Fonction pour obtenir les statistiques complètes
    async fn get_full_stats(&self) -> Result<FullStatResponse, Box<dyn Error>> {
        let result = timeout(Duration::from_secs(10), stat_full(&self.container_ip, self.minecraft_port)).await?;

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