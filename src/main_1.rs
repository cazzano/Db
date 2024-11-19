// main_1.rs
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use crate::{
    version,
    clear,
    initialize,
    help,
    dr::DirectoryNavigator,
    store::UrlStore,
    store_2::SecretStore,
    store_3::EmergencyStore,
    edit::EditCommand,
    dir_error::DirectoryState,
};

pub fn run_command_loop(
    rl: &mut DefaultEditor,
    dir_state: &mut DirectoryState,
    navigator: &mut DirectoryNavigator,
) {
    let url_store = match UrlStore::new() {
        Ok(store) => store,
        Err(e) => {
            println!("Failed to initialize URL store: {}. Please run 'init' first.", e);
            UrlStore::new().unwrap_or_else(|_| panic!("Failed to create URL store"))
        }
    };

    let secret_store = match SecretStore::new() {
        Ok(store) => store,
        Err(e) => {
            println!("Failed to initialize Secret store: {}. Please run 'init' first.", e);
            SecretStore::new().unwrap_or_else(|_| panic!("Failed to create Secret store"))
        }
    };

    let emergency_store = match EmergencyStore::new() {
        Ok(store) => store,
        Err(e) => {
            println!("Failed to initialize Emergency store: {}. Please run 'init' first.", e);
            EmergencyStore::new().unwrap_or_else(|_| panic!("Failed to create Emergency store"))
        }
    };

    let edit_command = EditCommand::new();
    
    println!("Welcome to DB MG Tool. Type 'help' for help, 'exit' to quit.");
    
    loop {
        let readline = rl.readline("db-mg> ");
        
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                
              // Handle cd command with proper quote handling
              if line.trim().starts_with("cd ") {
                let path_str = line.trim()
                    .strip_prefix("cd ")
                    .unwrap()
                    .trim();
                
                // Handle quoted paths
                let dir = if path_str.starts_with("\"") && path_str.ends_with("\"") {
                    // Remove the quotes
                    &path_str[1..path_str.len() - 1]
                } else {
                    path_str
                };
                
                if let Ok(absolute_path) = navigator.change_directory(dir) {
                    if let Err(e) = dir_state.sync_with_navigator(&absolute_path) {
                        println!("Error syncing directory state: {}", e);
                    }
                } else {
                    println!("Error changing directory: {}", dir);
                }
                continue;
                }

                // Handle edit command
                if line.trim().starts_with("edit ") {
                    let filename = line.trim()
                        .strip_prefix("edit ")
                        .unwrap()
                        .trim();
                    
                    if let Err(e) = edit_command.edit_file(filename, dir_state) {
                        println!("Error editing file: {}", e);
                    }
                    continue;
                }

                // Handle store url command
                if line.trim().starts_with("store url ") {
                    let filename = line.trim()
                        .strip_prefix("store url ")
                        .unwrap()
                        .trim();
                    
                    if let Err(e) = url_store.store_url(filename) {
                        println!("Error storing URL: {}", e);
                    }
                    continue;
                }

                // Handle store secret command
                if line.trim().starts_with("store secret ") {
                    let filename = line.trim()
                        .strip_prefix("store secret ")
                        .unwrap()
                        .trim();
                    
                    if let Err(e) = secret_store.store_secret(filename) {
                        println!("Error storing secret: {}", e);
                    }
                    continue;
                }

                // Handle store critical command
                if line.trim().starts_with("store critical ") {
                    let filename = line.trim()
                        .strip_prefix("store critical ")
                        .unwrap()
                        .trim();
                    
                    if let Err(e) = emergency_store.store_critical(filename) {
                        println!("Error storing emergency file: {}", e);
                    }
                    continue;
                }

                // Handle other commands
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                match parts.get(0).map(|&s| s) {
                    Some("version") => version::show_version(),
                    Some("clear") => clear::clear_screen(),
                    Some("init") => {
                        if let Err(e) = initialize::init_directories() {
                            println!("Initialization failed: {}", e);
                        }
                    },
                    Some("ls") => {
                        if let Err(e) = navigator.list_directories() {
                            println!("Error listing directories: {}", e);
                        }
                    },
                    Some("exit") | Some("quit") => {
                        println!("Goodbye!");
                        break;
                    }
                    Some("help") => help::show_help(),
                    Some("") => continue,
                    Some(cmd) => println!("Unknown command: {}", cmd),
                    None => continue,
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}