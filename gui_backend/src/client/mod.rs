use mc_query::query::{stat_basic, stat_full, BasicStatResponse, FullStatResponse};
use rcon::RconClient;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::error::Error;

pub(crate) mod rcon;

pub struct Client {
    pub address: String,
    pub rcon_client: Option<RconClient>,
    rcon_password: String,
}

impl Client {
    pub fn new(address: String) -> Self {
        Client {
            address: address.clone(),
            rcon_client: None,
            rcon_password: RconClient::generate_password(16),
        }
    }

    pub async fn attach_rcon(&mut self) -> Result<(), Box<dyn Error>> {
        let rcon_client = RconClient::new(&self.address, &self.rcon_password).await?;
        self.rcon_client = Some(rcon_client);
        Ok(())
    }

    pub fn get_rcon_password(&self) -> String {
        self.rcon_password.clone()
    }

    // Fonction pour obtenir les statistiques de base
    pub async fn get_basic_stats(&self) -> Result<BasicStatResponse, Box<dyn Error>> {
        let stats = stat_basic(&self.address, 25565).await?;
        Ok(stats)
    }
    
    // Fonction pour obtenir les statistiques complètes
    pub async fn get_full_stats(&self) -> Result<FullStatResponse, Box<dyn Error>> {
        let stats = stat_full(&self.address, 25565).await?;
        Ok(stats)
    }
}

lazy_static! {
    pub static ref CLIENT: Mutex<Client> = Mutex::new(Client::new("localhost".to_string())
    );
}