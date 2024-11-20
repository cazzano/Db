use crate::editor::Editor;
use colored::*;
use dialoguer::{Confirm, Input, Select};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Config {
    base_path: String,
    directories: Vec<String>,
    created_at: String,
}

pub struct SecretStore {
    secrets_dir: PathBuf,
    editor: Editor,
    target_directory: Option<PathBuf>,
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
        let config_path = dirs::config_dir()
            .ok_or("Could not determine config directory")?
            .join("db-mg")
            .join("db.json");

        let config_content = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_content)?;

        let secrets_dir = PathBuf::from(config.base_path).join("Secrets");

        Ok(SecretStore {
            secrets_dir,
            editor: Editor::new(),
            target_directory: None,
        })
    }

    pub fn set_target_directory(&mut self, target_dir: PathBuf) {
        self.target_directory = Some(target_dir);
    }

    fn copy_directory_contents(
        source: &PathBuf,
        destination: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !destination.exists() {
            fs::create_dir_all(destination)?;
        }

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();
            let dest_path = destination.join(entry.file_name());

            if path.is_dir() {
                Self::copy_directory_contents(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)?;
            }
        }
        Ok(())
    }

    fn copy_file_to_target(&self, source_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(target_dir) = &self.target_directory {
            if source_path.is_dir() {
                let dir_name = source_path
                    .file_name()
                    .ok_or("Could not determine directory name")?;
                let target_path = target_dir.join(dir_name);
                Self::copy_directory_contents(source_path, &target_path)?;
            } else {
                let file_name = source_path
                    .file_name()
                    .ok_or("Could not determine file name")?;
                let target_path = target_dir.join(file_name);
                fs::copy(source_path, target_path)?;
            }
        }
        Ok(())
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
        fs::create_dir_all(self.secrets_dir.join("Emails"))?;
        fs::create_dir_all(self.secrets_dir.join("Social Media IDs"))?;
        fs::create_dir_all(self.secrets_dir.join("Phone Numbers"))?;
        Ok(())
    }

    pub fn store_secret(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.ensure_secret_directories()?;

        let secret_type = Self::get_secret_type()?;
        let category_path = self.get_category_path(&secret_type);

        let final_path = if matches!(secret_type, SecretType::Other) {
            let use_folder = Confirm::new()
                .with_prompt("Do you want to save it inside a custom folder? (y/n)")
                .interact()?;

            if use_folder {
                let folder_name: String =
                    Input::new().with_prompt("Enter folder name").interact()?;
                let folder_path = category_path.join(&folder_name);
                fs::create_dir_all(&folder_path)?;
                folder_path.join(filename)
            } else {
                category_path.join(filename)
            }
        } else {
            let use_subfolder = Confirm::new()
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

        let relative_path = final_path
            .strip_prefix(&self.secrets_dir)
            .unwrap_or(&final_path)
            .display();

        println!(
            "{} Secret file saved in: Secrets/{}",
            "✓".green(),
            relative_path
        );

        let edit_file = Confirm::new()
            .with_prompt("Do you want to edit this secret file now? (y/n)")
            .interact()?;

        if edit_file {
            if let Err(e) = self.editor.edit_file(&final_path) {
                println!("Error editing file: {}", e);
            }
        }

        // Ask if user wants to drop files or folders
        if let Some(_target_dir) = &self.target_directory {
            let drop_items = Confirm::new()
                .with_prompt("Do you want to drop files or folders? (y/n)")
                .interact()?;

            if drop_items {
                // Copy the entire category directory
                let category_to_copy = self.get_category_path(&secret_type);
                self.copy_file_to_target(&category_to_copy)?;
                println!(
                    "{} Files and folders dropped to current directory",
                    "✓".green()
                );
            }
        }

        Ok(())
    }

    pub fn get_secrets_dir(&self) -> &PathBuf {
        &self.secrets_dir
    }
}
