use std::fs;
use std::path::PathBuf;
use dialoguer::{Input, Confirm};
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    base_path: String,
    directories: Vec<String>,
    created_at: String,
}

pub struct UrlStore {
    urls_dir: PathBuf,
}

impl UrlStore {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Read config to get base path
        let config_path = dirs::config_dir()
            .ok_or("Could not determine config directory")?
            .join("db-mg")
            .join("db.json");

        let config_content = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_content)?;
        
        // Construct path to Internet Urls directory
        let urls_dir = PathBuf::from(config.base_path).join("Internet Urls");
        
        Ok(UrlStore { urls_dir })
    }

    pub fn store_url(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Ask if user wants to save in a folder
        let use_folder = Confirm::new()
            .with_prompt("Do you want to save it inside a folder? (y/n)")
            .interact()?;

        let final_path = if use_folder {
            // Get folder name from user
            let folder_name: String = Input::new()
                .with_prompt("Enter folder name")
                .interact()?;

            // Create folder if it doesn't exist
            let folder_path = self.urls_dir.join(&folder_name);
            fs::create_dir_all(&folder_path)?;
            
            folder_path.join(filename)
        } else {
            self.urls_dir.join(filename)
        };

        // Create the file
        fs::write(&final_path, "")?;
        
        println!("{} URL file saved: {}", "✓".green(), final_path.display());
        Ok(())
    }
}