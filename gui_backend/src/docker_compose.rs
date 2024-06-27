use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer, Value};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerCompose {
    content: serde_yaml::Value,
    path: PathBuf,
}

impl DockerCompose {
    // Constructor to create a new DockerCompose instance from a file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(&path)?;
        let docker_compose: serde_yaml::Value = from_reader(file)?;

        Ok(DockerCompose {
            content: docker_compose,
            path: path.as_ref().to_path_buf(),
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
    pub fn set_value(&mut self, service_name: &str, key: &str, value: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn remove_value(&mut self, service_name: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    // Method to save the current Docker Compose configuration back to a file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(&self.path)?;
        to_writer(file, &self.content)?;
        Ok(())
    }

    // Method to start the docker
    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("docker")
            .arg("compose")
            .arg("up")
            .arg("-d")
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            Err("Failed to start docker-compose services".into())
        }
    }
}