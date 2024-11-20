// store.rs (modified version)
use crate::editor::Editor;
use colored::*;
use dialoguer::{Confirm, Input};
use serde::{Deserialize, Serialize};
use std::fs;
// use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Config {
    base_path: String,
    directories: Vec<String>,
    created_at: String,
}

pub struct UrlStore {
    urls_dir: PathBuf,
    editor: Editor,
}

impl UrlStore {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = dirs::config_dir()
            .ok_or("Could not determine config directory")?
            .join("db-mg")
            .join("db.json");

        let config_content = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_content)?;

        let urls_dir = PathBuf::from(config.base_path).join("Internet Urls");

        Ok(UrlStore {
            urls_dir,
            editor: Editor::new(),
        })
    }

    pub fn store_url(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let use_folder = Confirm::new()
            .with_prompt("Do you want to save it inside a folder? (y/n)")
            .interact()?;

        let final_path = if use_folder {
            let folder_name: String = Input::new().with_prompt("Enter folder name").interact()?;
            let folder_path = self.urls_dir.join(&folder_name);
            fs::create_dir_all(&folder_path)?;
            folder_path.join(filename)
        } else {
            self.urls_dir.join(filename)
        };

        // Get URL content from user
        let url_content: String = Input::new()
            .with_prompt("Enter the URL content")
            .interact()?;

        // Create the file with content
        fs::write(&final_path, url_content)?;

        println!("{} URL file saved: {}", "✓".green(), final_path.display());

        // Add the editor functionality
        self.editor.edit_file(&final_path)?;

        Ok(())
    }
}
