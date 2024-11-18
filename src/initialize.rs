// src/initialize.rs
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use dialoguer::Input;
use colored::*;

#[derive(Serialize, Deserialize)]
struct Config {
    base_path: String,
    directories: Vec<String>,
    created_at: String,
}

pub fn init_directories() -> Result<(), Box<dyn std::error::Error>> {
    // Get the base directory from user input
    let base_path: String = Input::new()
        .with_prompt("Enter the base directory path for initialization")
        .interact()?;

    let base_path = shellexpand::tilde(&base_path).to_string();
    let base_path = PathBuf::from(base_path);

    // Create the base directory if it doesn't exist
    fs::create_dir_all(&base_path)?;

    // Define the directories to create
    let directories = vec![
        "Internet Urls",
        "Secrets",
        "Emergency"
    ];

    // Create each directory
    for dir in &directories {
        let dir_path = base_path.join(dir);
        fs::create_dir_all(&dir_path)?;
        println!("{} Created directory: {}", "✓".green(), dir_path.display());
    }

    // Create config directory
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("db-mg");
    fs::create_dir_all(&config_dir)?;

    // Create and save configuration
    let config = Config {
        base_path: base_path.to_string_lossy().to_string(),
        directories: directories.iter().map(|s| s.to_string()).collect(),
        created_at: chrono::Local::now().to_rfc3339(),
    };

    // Save to db.json
    let config_path = config_dir.join("db.json");
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_json)?;

    println!("\n{} Initialization completed successfully!", "✓".green());
    println!("Configuration saved to: {}", config_path.display());
    println!("Created directories in: {}", base_path.display());

    Ok(())
}