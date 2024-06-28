use lazy_static::lazy_static;
use std::sync::Mutex;
use minecraft_client_rs::{Client, Message};
use rand::Rng;
use std::time::Duration;
use std::thread;
use std::error::Error;
use std::fmt;

pub struct RconClient {
    client: Option<Client>,
    password: String,
    logged_in: bool,
}

impl RconClient {
    fn new() -> Self {
        RconClient {
            client: None,
            password: RconClient::generate_password(16),
            logged_in: false,
        }
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
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

    pub fn authenticate(&mut self, address: String) -> Result<(), Box<dyn Error>> {
        if self.is_logged_in() {
            Err(AlreadyLoggedInError)?
        }
        loop {
            self.client = Some(Client::new(address.clone()).unwrap());
            match self.client.as_mut().unwrap().authenticate(self.password.clone()) {
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

    pub fn close(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.as_mut().unwrap().close()?;
        Ok(())
    }

    pub fn send_command(&mut self, command: String) -> Result<Message, Box<dyn Error>> {
        if !self.is_logged_in() {
            Err(Box::new(NotLoggedInError))?
        }
        match self.client.as_mut().unwrap().send_command(command.to_string()) {
            Ok(resp) => { Ok(resp) },
            Err(err) => { Err(err) },
        }
    } 
}

lazy_static! {
    pub static ref RCON_CLIENT: Mutex<RconClient> = Mutex::new(RconClient::new());
}

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