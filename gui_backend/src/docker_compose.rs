use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer, Value, Mapping};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerCompose {
    content: serde_yaml::Value,
    path: PathBuf,
    use_docker_compose: bool,
}

impl DockerCompose {
    // Constructor to create a new DockerCompose instance from a file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(&path)?;
        let docker_compose: serde_yaml::Value = from_reader(file)?;

        Ok(DockerCompose {
            content: docker_compose,
            path: path.as_ref().to_path_buf(),
            use_docker_compose: false,
        })
    }

    // Method to add a service
    pub fn set_service(&mut self, name: &str, service: serde_json::Value) {
        let yaml_service: Value = serde_json::from_value(service).unwrap();
        if let Some(services) = self.content.get_mut("services").and_then(Value::as_mapping_mut) {
            services.insert(Value::String(name.to_string()), yaml_service);
        } else {
            let mut services = serde_yaml::Mapping::new();
            services.insert(Value::String(name.to_string()), yaml_service);
            self.content.as_mapping_mut().unwrap().insert(Value::String("services".to_string()), Value::Mapping(services));
        }
    }

    // Method to get a service
    pub fn get_service(&self, name: &str) -> Option<&Value> {
        self.content.get("services")
            .and_then(Value::as_mapping)
            .and_then(|services| services.get(&Value::String(name.to_string())))
    }

    // Method to remove a service
    pub fn remove_service(&mut self, name: &str) {
        if let Some(services) = self.content.get_mut("services").and_then(Value::as_mapping_mut) {
            services.remove(&serde_yaml::Value::String(name.to_string()));
        }
    }

    // Method to set a value in a specific service
    pub fn set_value(&mut self, service_name: &str, key: &str, value: serde_json::Value) -> Result<(), Box<dyn Error>> {
        let yaml_value: Value = serde_json::from_value(value)?;
        if let Some(service) = self.content.get_mut("services")
            .and_then(Value::as_mapping_mut)
            .and_then(|services| services.get_mut(&Value::String(service_name.to_string())))
            .and_then(Value::as_mapping_mut)
        {
            service.insert(Value::String(key.to_string()), yaml_value);
            Ok(())
        } else {
            Err("Service not found".into())
        }
    }

    // Method to get a value from a specific service
    pub fn get_value(&self, service_name: &str, key: &str) -> Option<&Value> {
        self.content.get("services")
            .and_then(Value::as_mapping)
            .and_then(|services| services.get(&Value::String(service_name.to_string())))
            .and_then(Value::as_mapping)
            .and_then(|service| service.get(&Value::String(key.to_string())))
    }

    // Method to remove a value from a specific service
    pub fn remove_value(&mut self, service_name: &str, key: &str) -> Result<(), Box<dyn Error>> {
        if let Some(service) = self.content.get_mut("services")
            .and_then(Value::as_mapping_mut)
            .and_then(|services| services.get_mut(&Value::String(service_name.to_string())))
            .and_then(Value::as_mapping_mut)
        {
            service.remove(&Value::String(key.to_string()));
            Ok(())
        } else {
            Err("Service not found".into())
        }
    }

        // Method to set an environment variable in a specific service
        pub fn set_env(&mut self, service_name: &str, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
            let yaml_value = Value::String(value.to_string());
            if let Some(service) = self.content.get_mut("services")
                .and_then(Value::as_mapping_mut)
                .and_then(|services| services.get_mut(&Value::String(service_name.to_string())))
                .and_then(Value::as_mapping_mut)
            {
                if let Some(env) = service.get_mut("environment").and_then(Value::as_mapping_mut) {
                    env.insert(Value::String(key.to_string()), yaml_value);
                } else {
                    let mut env = Mapping::new();
                    env.insert(Value::String(key.to_string()), yaml_value);
                    service.insert(Value::String("environment".to_string()), Value::Mapping(env));
                }
                Ok(())
            } else {
                Err("Service not found".into())
            }
        }
    
        // Method to get an environment variable from a specific service
        pub fn get_env(&self, service_name: &str, key: &str) -> Option<&Value> {
            self.content.get("services")
                .and_then(Value::as_mapping)
                .and_then(|services| services.get(&Value::String(service_name.to_string())))
                .and_then(Value::as_mapping)
                .and_then(|service| service.get("environment"))
                .and_then(Value::as_mapping)
                .and_then(|env| env.get(&Value::String(key.to_string())))
        }
    
        // Method to remove an environment variable from a specific service
        pub fn remove_env(&mut self, service_name: &str, key: &str) -> Result<(), Box<dyn Error>> {
            if let Some(service) = self.content.get_mut("services")
                .and_then(Value::as_mapping_mut)
                .and_then(|services| services.get_mut(&Value::String(service_name.to_string())))
                .and_then(Value::as_mapping_mut)
            {
                if let Some(env) = service.get_mut("environment").and_then(Value::as_mapping_mut) {
                    env.remove(&Value::String(key.to_string()));
                    Ok(())
                } else {
                    Err("Environment section not found".into())
                }
            } else {
                Err("Service not found".into())
            }
        }

    // Method to get the container ip address
    pub fn get_container_ip(&self, service: String) -> Result<String, Box<dyn Error>> {
        let command_output = if self.use_docker_compose {
            Command::new("docker")
                .arg("compose")
                .arg("ps")
                .arg("-q")
                .arg(service)
                .output()?
        } else {
            Command::new("docker-compose")
                .arg("ps")
                .arg("-q")
                .arg(service)
                .output()?
        };
        let container_id = std::str::from_utf8(&command_output.stdout)?.trim();

        if container_id.is_empty() {
            return Err("Failed to find container ID".into());
        }

        let inspect_output = Command::new("docker")
        .arg("inspect")
        .arg("--format")
        .arg("{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}")
        .arg(container_id)
        .output()?;

        let container_ip = std::str::from_utf8(&inspect_output.stdout)?.trim();

        if container_ip.is_empty() {
            return Err("Failed to find container IP address".into());
        }
        Ok(container_ip.to_string())
    }

    // Method to save the current Docker Compose configuration back to a file
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(&self.path)?;
        to_writer(file, &self.content)?;
        Ok(())
    }

    fn check_docker_compose(&mut self) {
        let status = Command::new("docker")
            .arg("compose")
            .arg("version")
            .status();
        
        if let Ok(status) = status {
            if status.success() {
                self.use_docker_compose = true;
            }
        }
    }

    // Method to start the docker
    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.check_docker_compose();

        let status = if self.use_docker_compose {
            Command::new("docker")
                .arg("compose")
                .arg("up")
                .arg("-d")
                .status()?
        } else {
            Command::new("docker-compose")
                .arg("up")
                .arg("-d")
                .status()?
        };
        
        if status.success() {
            Ok(())
        } else {
            Err("Failed to start docker-compose services".into())
        }
    }

    pub fn stop(&self) -> Result<(), Box<dyn Error>> {
        let status = if self.use_docker_compose {
            Command::new("docker")
                .arg("compose")
                .arg("down")
                .status()?
        } else {
            Command::new("docker-compose")
                .arg("down")
                .status()?
        };
        
        if status.success() {
            Ok(())
        } else {
            Err("Failed to stop docker-compose services".into())
        }
    }
}