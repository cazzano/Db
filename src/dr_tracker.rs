use std::fs;
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub base_path: String,
    pub directories: Vec<String>,
    pub created_at: String,
    pub subdirectories: HashMap<String, Vec<String>>,
}

#[derive(Clone)]
pub struct DirectoryTracker {
    config_path: PathBuf,
    pub config: Config,
}


impl DirectoryTracker {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = dirs::config_dir()
            .ok_or("Could not determine config directory")?
            .join("db-mg")
            .join("db.json");
        
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let mut config: Config = serde_json::from_str(&content)?;
            
            if config.subdirectories.is_empty() {
                config.subdirectories = HashMap::new();
            }
            config
        } else {
            Config {
                base_path: String::new(),
                directories: Vec::new(),
                created_at: chrono::Local::now().to_string(),
                subdirectories: HashMap::new(),
            }
        };

        Ok(DirectoryTracker { config_path, config })
    }

    pub fn add_subdirectory(&mut self, parent: &str, subdir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let subdirs = self.config.subdirectories.entry(parent.to_string())
            .or_insert_with(Vec::new);
        
        if !subdirs.contains(&subdir.to_string()) {
            subdirs.push(subdir.to_string());
            self.save_config()?;
        }
        
        Ok(())
    }

    pub fn get_subdirectories(&self, parent: &str) -> Vec<String> {
        self.config.subdirectories.get(parent)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_base_path(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.config.base_path.clone())
    }

    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }
}