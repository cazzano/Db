use std::fs;
use std::path::PathBuf;
use dialoguer::{Input, Select};
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    base_path: String,
    directories: Vec<String>,
    created_at: String,
}

pub struct EmergencyStore {
    emergency_dir: PathBuf,
}

#[derive(Debug)]
enum EmergencyType {
    Videos,
    Audios,
    Images,
    Compressed,
    Coding,
    Apps,
    Secrets,
}

impl EmergencyStore {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Read config to get base path
        let config_path = dirs::config_dir()
            .ok_or("Could not determine config directory")?
            .join("db-mg")
            .join("db.json");

        let config_content = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_content)?;
        
        // Construct path to Emergency directory
        let emergency_dir = PathBuf::from(config.base_path).join("Emergency");
        
        Ok(EmergencyStore { emergency_dir })
    }

    fn get_emergency_type() -> Result<EmergencyType, Box<dyn std::error::Error>> {
        let emergency_types = vec!["Videos", "Audios", "Images", "Compressed", "Coding", "Apps", "Secrets"];
        
        let selection = Select::new()
            .with_prompt("What type of emergency file is this?")
            .items(&emergency_types)
            .default(0)
            .interact()?;

        match selection {
            0 => Ok(EmergencyType::Videos),
            1 => Ok(EmergencyType::Audios),
            2 => Ok(EmergencyType::Images),
            3 => Ok(EmergencyType::Compressed),
            4 => Ok(EmergencyType::Coding),
            5 => Ok(EmergencyType::Apps),
            6 => Ok(EmergencyType::Secrets),
            _ => Err("Invalid selection".into()),
        }
    }

    fn get_category_path(&self, emergency_type: &EmergencyType) -> PathBuf {
        match emergency_type {
            EmergencyType::Videos => self.emergency_dir.join("Videos"),
            EmergencyType::Audios => self.emergency_dir.join("Audios"),
            EmergencyType::Images => self.emergency_dir.join("Images"),
            EmergencyType::Compressed => self.emergency_dir.join("Compressed"),
            EmergencyType::Coding => self.emergency_dir.join("Coding"),
            EmergencyType::Apps => self.emergency_dir.join("Apps"),
            EmergencyType::Secrets => self.emergency_dir.join("Secrets"),
        }
    }

    fn ensure_emergency_directories(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create all category directories if they don't exist
        fs::create_dir_all(self.emergency_dir.join("Videos"))?;
        fs::create_dir_all(self.emergency_dir.join("Audios"))?;
        fs::create_dir_all(self.emergency_dir.join("Images"))?;
        fs::create_dir_all(self.emergency_dir.join("Compressed"))?;
        fs::create_dir_all(self.emergency_dir.join("Coding"))?;
        fs::create_dir_all(self.emergency_dir.join("Apps"))?;
        fs::create_dir_all(self.emergency_dir.join("Secrets"))?;
        Ok(())
    }

    pub fn store_critical(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure all emergency directories exist
        self.ensure_emergency_directories()?;

        // Get the type of emergency file
        let emergency_type = Self::get_emergency_type()?;
        
        // Get the base path for this type of emergency file
        let category_path = self.get_category_path(&emergency_type);

        // For all types, optionally allow subfolder creation
        let use_subfolder = dialoguer::Confirm::new()
            .with_prompt("Do you want to create a subfolder under this category? (y/n)")
            .interact()?;

        let final_path = if use_subfolder {
            // Get subfolder name from user
            let subfolder_name: String = Input::new()
                .with_prompt("Enter subfolder name")
                .interact()?;

            let subfolder_path = category_path.join(&subfolder_name);
            fs::create_dir_all(&subfolder_path)?;
            
            subfolder_path.join(filename)
        } else {
            category_path.join(filename)
        };

        // Create the file
        fs::write(&final_path, "")?;
        
        // Get the relative path from the Emergency directory for display
        let relative_path = final_path.strip_prefix(&self.emergency_dir)
            .unwrap_or(&final_path)
            .display();
        
        println!("{} Emergency file saved in: Emergency/{}", "✓".green(), relative_path);
        Ok(())
    }
}