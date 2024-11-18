// src/dr.rs
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use colored::*;

#[derive(Serialize, Deserialize)]
struct Config {
    base_path: String,
    directories: Vec<String>,
    created_at: String,
}

pub struct DirectoryNavigator {
    current_path: PathBuf,
    base_path: PathBuf,
}

impl DirectoryNavigator {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = dirs::config_dir()
            .ok_or("Could not determine config directory")?
            .join("db-mg")
            .join("db.json");

        let config_content = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_content)?;
        let base_path = PathBuf::from(config.base_path);

        Ok(DirectoryNavigator {
            current_path: base_path.clone(),
            base_path,
        })
    }

    pub fn list_directories(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.current_path.exists() {
            return Err("Current directory does not exist".into());
        }

        println!("\nCurrent directory: {}", self.current_path.display().to_string().cyan());
        println!("Contents:");

        let entries = fs::read_dir(&self.current_path)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Invalid name");

            if path.is_dir() {
                println!("📁 {}", name.blue());
            } else {
                println!("📄 {}", name);
            }
        }
        println!();
        Ok(())
    }

    pub fn change_directory(&mut self, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Handle quoted directory names by removing quotes if present
        let dir = dir.trim_matches('"').trim_matches('\'');

        let new_path = if dir == ".." {
            if self.current_path == self.base_path {
                return Err("Already at base directory".into());
            }
            self.current_path.parent()
                .ok_or("Cannot go up from current directory")?
                .to_path_buf()
        } else {
            let target = self.current_path.join(dir);
            if !target.exists() {
                return Err(format!("Directory '{}' does not exist", dir).into());
            }
            if !target.is_dir() {
                return Err(format!("'{}' is not a directory", dir).into());
            }
            if !target.starts_with(&self.base_path) {
                return Err("Cannot navigate outside base directory".into());
            }
            target
        };

        self.current_path = new_path;
        Ok(())
    }
}