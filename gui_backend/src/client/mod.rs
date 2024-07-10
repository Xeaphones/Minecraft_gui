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
}

lazy_static! {
    pub static ref CLIENT: Mutex<Client> = Mutex::new(Client::new("localhost".to_string())
    );
}