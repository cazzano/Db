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

pub struct SecretStore {
    secrets_dir: PathBuf,
}

#[derive(Debug)]
enum SecretType {
    Email,
    SocialMedia,
    Phone,
    Other,
}

impl SecretStore {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Read config to get base path
        let config_path = dirs::config_dir()
            .ok_or("Could not determine config directory")?
            .join("db-mg")
            .join("db.json");

        let config_content = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_content)?;
        
        // Construct path to Secrets directory
        let secrets_dir = PathBuf::from(config.base_path).join("Secrets");
        
        Ok(SecretStore { secrets_dir })
    }

    fn get_secret_type() -> Result<SecretType, Box<dyn std::error::Error>> {
        let secret_types = vec!["Email", "Social Media", "Phone Number", "Other"];
        
        let selection = Select::new()
            .with_prompt("What type of secret is this?")
            .items(&secret_types)
            .default(0)
            .interact()?;

        match selection {
            0 => Ok(SecretType::Email),
            1 => Ok(SecretType::SocialMedia),
            2 => Ok(SecretType::Phone),
            3 => Ok(SecretType::Other),
            _ => Err("Invalid selection".into()),
        }
    }

    fn get_category_path(&self, secret_type: &SecretType) -> PathBuf {
        match secret_type {
            SecretType::Email => self.secrets_dir.join("Emails"),
            SecretType::SocialMedia => self.secrets_dir.join("Social Media IDs"),
            SecretType::Phone => self.secrets_dir.join("Phone Numbers"),
            SecretType::Other => self.secrets_dir.clone(),
        }
    }

    fn ensure_secret_directories(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create all category directories if they don't exist
        fs::create_dir_all(self.secrets_dir.join("Emails"))?;
        fs::create_dir_all(self.secrets_dir.join("Social Media IDs"))?;
        fs::create_dir_all(self.secrets_dir.join("Phone Numbers"))?;
        Ok(())
    }

    pub fn store_secret(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure all secret directories exist
        self.ensure_secret_directories()?;

        // Get the type of secret
        let secret_type = Self::get_secret_type()?;
        
        // Get the base path for this type of secret
        let category_path = self.get_category_path(&secret_type);

        // For "Other" type, ask if they want to use a custom folder
        let final_path = if matches!(secret_type, SecretType::Other) {
            let use_folder = dialoguer::Confirm::new()
                .with_prompt("Do you want to save it inside a custom folder? (y/n)")
                .interact()?;

            if use_folder {
                // Get folder name from user
                let folder_name: String = Input::new()
                    .with_prompt("Enter folder name")
                    .interact()?;

                // Create folder if it doesn't exist
                let folder_path = category_path.join(&folder_name);
                fs::create_dir_all(&folder_path)?;
                
                folder_path.join(filename)
            } else {
                category_path.join(filename)
            }
        } else {
            // For predefined categories, optionally allow subfolder creation
            let use_subfolder = dialoguer::Confirm::new()
                .with_prompt("Do you want to create a subfolder under this category? (y/n)")
                .interact()?;

            if use_subfolder {
                let subfolder_name: String = Input::new()
                    .with_prompt("Enter subfolder name")
                    .interact()?;

                let subfolder_path = category_path.join(&subfolder_name);
                fs::create_dir_all(&subfolder_path)?;
                
                subfolder_path.join(filename)
            } else {
                category_path.join(filename)
            }
        };

        // Create the file
        fs::write(&final_path, "")?;
        
        // Get the relative path from the Secrets directory for display
        let relative_path = final_path.strip_prefix(&self.secrets_dir)
            .unwrap_or(&final_path)
            .display();
        
        println!("{} Secret file saved in: Secrets/{}", "✓".green(), relative_path);
        Ok(())
    }
}