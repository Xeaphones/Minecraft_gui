use mc_query::rcon::RconClient as Rcon;
use rand::Rng;
use std::time::Duration;
use std::thread;
use std::error::Error;
use std::fmt;
use tokio::io;

pub struct RconClient {
    client: Rcon,
    password: String,
    address: String,
    logged_in: bool,
}

impl RconClient {
    pub async fn new(address: &String, _password: &String) -> io::Result<Self> {
        Ok(RconClient {
            client: Rcon::new(&address, 25575).await?,
            password: _password.clone(),
            address: address.clone(),
            logged_in: false,
        })
    }

    pub fn is_logged_in(&self) -> bool {
        self.logged_in
    }

    pub fn generate_password(length: usize) -> String {
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

    pub async fn authenticate(&mut self) -> Result<(), Box<dyn Error>> {
        if self.is_logged_in() {
            Err(AlreadyLoggedInError)?
        }
        loop {
            self.client = Rcon::new(&self.address, 25575).await?;
            match self.client.authenticate(&self.password).await {
                Ok(_) => { 
                    println!("Rcon Authenticated");
                    self.logged_in = true;
                    break;
                },
                Err(err) => {println!("Authentication Error.\nErr: {:?}", err);},
            }
            thread::sleep(Duration::from_secs(10));
        }
        Ok(())
    }

    pub async fn close(self) -> Result<(), Box<dyn Error>> {
        self.client.disconnect().await?;
        Ok(())
    }

    pub async fn send_command(&mut self, command: String) -> Result<String, Box<dyn Error>> {
        if !self.is_logged_in() {
            Err(Box::new(NotLoggedInError))?
        }
        match self.client.run_command(&command.to_string()).await {
            Ok(resp) => { Ok(resp) },
            Err(err) => { Err(Box::new(err))? },
        }
    } 
}

// lazy_static! {
//     pub static ref RCON_CLIENT: Mutex<RconClient> = Mutex::new(RconClient::new(&String::from("localhost")).unwrap());
// }

#[derive(Debug)]
struct NotLoggedInError;

impl Error for NotLoggedInError {}

impl fmt::Display for NotLoggedInError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Not logged in")
	}
}

#[derive(Debug)]
struct AlreadyLoggedInError;

impl Error for AlreadyLoggedInError {}

impl fmt::Display for AlreadyLoggedInError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Already logged in")
	}
}