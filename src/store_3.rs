use crate::dir_error::DirectoryState;
use crate::edit::EditCommand;
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

pub struct EmergencyStore {
    emergency_dir: PathBuf,
    edit_command: EditCommand,
    source_file: Option<PathBuf>,
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
    Others,
}

impl EmergencyStore {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = dirs::config_dir()
            .ok_or("Could not determine config directory")?
            .join("db-mg")
            .join("db.json");

        let config_content = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_content)?;

        let emergency_dir = PathBuf::from(config.base_path).join("Emergency");

        Ok(EmergencyStore {
            emergency_dir,
            edit_command: EditCommand::new(),
            source_file: None,
        })
    }

    pub fn set_source_file(&mut self, source_path: PathBuf) {
        self.source_file = Some(source_path);
    }

    fn copy_with_contents(
        source: &PathBuf,
        dest: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if source.is_file() {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(source, dest)?;
            return Ok(());
        }

        if !dest.exists() {
            fs::create_dir_all(dest)?;
        }

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let src_path = entry.path();
            let dest_path = dest.join(entry.file_name());

            if src_path.is_dir() {
                fs::create_dir_all(&dest_path)?;
                Self::copy_with_contents(&src_path, &dest_path)?;
            } else {
                fs::copy(&src_path, &dest_path)?;
            }
        }
        Ok(())
    }

    fn get_emergency_type() -> Result<EmergencyType, Box<dyn std::error::Error>> {
        let emergency_types = vec![
            "Videos",
            "Audios",
            "Images",
            "Compressed",
            "Coding",
            "Apps",
            "Secrets",
            "Others",
        ];

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
            7 => Ok(EmergencyType::Others),
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
            EmergencyType::Others => self.emergency_dir.clone(),
        }
    }

    fn ensure_emergency_directories(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(self.emergency_dir.join("Videos"))?;
        fs::create_dir_all(self.emergency_dir.join("Audios"))?;
        fs::create_dir_all(self.emergency_dir.join("Images"))?;
        fs::create_dir_all(self.emergency_dir.join("Compressed"))?;
        fs::create_dir_all(self.emergency_dir.join("Coding"))?;
        fs::create_dir_all(self.emergency_dir.join("Apps"))?;
        fs::create_dir_all(self.emergency_dir.join("Secrets"))?;
        fs::create_dir_all(self.emergency_dir.join("Others"))?;
        Ok(())
    }

    pub fn store_critical(
        &self,
        filename: &str,
        dir_state: &DirectoryState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.ensure_emergency_directories()?;

        let emergency_type = Self::get_emergency_type()?;
        let category_path = self.get_category_path(&emergency_type);

        let use_subfolder = Confirm::new()
            .with_prompt("Do you want to create a subfolder under this category? (y/n)")
            .interact()?;

        let final_path = if use_subfolder {
            let subfolder_name: String = Input::new()
                .with_prompt("Enter subfolder name")
                .interact()?;

            let subfolder_path = category_path.join(&subfolder_name);
            fs::create_dir_all(&subfolder_path)?;

            subfolder_path.join(filename)
        } else {
            category_path.join(filename)
        };

        // Copy the source file contents if available, otherwise create empty file
        if let Some(source_path) = &self.source_file {
            Self::copy_with_contents(source_path, &final_path)?;
        } else {
            fs::write(&final_path, "")?;
        }

        let relative_path = final_path
            .strip_prefix(&self.emergency_dir)
            .unwrap_or(&final_path)
            .display();

        println!(
            "{} Emergency file saved in: Emergency/{}",
            "✓".green(),
            relative_path
        );

        let edit_file = Confirm::new()
            .with_prompt("Do you want to edit this emergency file now? (y/n)")
            .interact()?;

        if edit_file {
            self.edit_command.edit_file(&final_path, dir_state)?;
        }

        // Ask if user wants to drop files or folders
        let drop_files = Confirm::new()
            .with_prompt("Do you want to drop files or folders? (y/n)")
            .interact()?;

        if drop_files {
            let current_dir = std::env::current_dir()?;
            Self::copy_with_contents(&self.emergency_dir, &current_dir)?;
            println!("{} Files and folders dropped successfully", "✓".green());
        }

        Ok(())
    }
}
