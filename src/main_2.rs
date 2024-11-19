use rustyline::DefaultEditor;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use crate::dir_error::{initialize_directory_state, DirectoryState};
use crate::dr::DirectoryNavigator;
use crate::main_1::run_command_loop;

#[derive(Deserialize)]
struct Config {
    base_path: String,
}

pub fn read_base_directory() -> Option<PathBuf> {
    let config_dir = dirs::config_dir()?.join("db-mg");
    let config_path = config_dir.join("db.json");

    if config_path.exists() {
        let config_contents = fs::read_to_string(config_path).ok()?;
        let config: Config = serde_json::from_str(&config_contents).ok()?;
        Some(PathBuf::from(config.base_path))
    } else {
        None
    }
}

pub fn initialize_application() -> (DefaultEditor, DirectoryState, DirectoryNavigator) {
    let mut rl = DefaultEditor::new().unwrap();

    // Initialize DirectoryState
    let mut dir_state = match initialize_directory_state() {
        Ok(state) => state,
        Err(e) => {
            println!(
                "Failed to initialize directory state: {}. Please run 'init' first.",
                e
            );
            panic!("Failed to initialize directory state");
        }
    };

    let mut navigator = match DirectoryNavigator::new() {
        Ok(nav) => nav,
        Err(e) => {
            println!(
                "Failed to initialize directory navigator: {}. Please run 'init' first.",
                e
            );
            DirectoryNavigator::new().unwrap_or_else(|_| panic!("Failed to create navigator"))
        }
    };

    // Change to base directory if it exists
    if let Some(base_dir) = read_base_directory() {
        if let Err(e) = navigator.change_directory(base_dir.to_str().unwrap()) {
            println!("Could not change to base directory: {}", e);
        }
        if let Err(e) = dir_state.sync_with_navigator(&base_dir) {
            println!("Could not sync directory state: {}", e);
        }
    } else {
        println!("No base directory configured. Please run 'init' first.");
    }

    (rl, dir_state, navigator)
}
