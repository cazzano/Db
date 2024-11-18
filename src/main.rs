use rustyline::error::ReadlineError;
use rustyline::{Editor, DefaultEditor};
mod version;
mod clear;
mod initialize;
mod dr;
mod store;
mod store_2;
mod store_3;

use dr::DirectoryNavigator;
use store::UrlStore;
use store_2::SecretStore;
use store_3::EmergencyStore;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    
    let mut navigator = match DirectoryNavigator::new() {
        Ok(nav) => nav,
        Err(e) => {
            println!("Failed to initialize directory navigator: {}. Please run 'init' first.", e);
            DirectoryNavigator::new().unwrap_or_else(|_| panic!("Failed to create navigator"))
        }
    };

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
    
    println!("Welcome to DB MG Tool. Type 'help' for help, 'exit' to quit.");
    
    loop {
        let readline = rl.readline("db-mg> ");
        
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                
                // Special handling for the cd command with quoted arguments
                if line.trim().starts_with("cd ") {
                    let dir = line.trim()
                        .strip_prefix("cd ")
                        .unwrap()
                        .trim();
                    
                    if let Err(e) = navigator.change_directory(dir) {
                        println!("Error changing directory: {}", e);
                    }
                    continue;
                }

                // Special handling for the store url command
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

                // Special handling for the store secret command
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

                // Special handling for the store critical command
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
                    Some("help") => {
                        println!("Available commands:");
                        println!("  version - Show DB MG version");
                        println!("  clear   - Clear the screen");
                        println!("  init    - Initialize directory structure");
                        println!("  ls      - List current directory contents");
                        println!("  cd DIR  - Change to directory (use quotes for names with spaces)");
                        println!("  store url FILENAME - Store a URL file");
                        println!("  store secret FILENAME - Store a secret file");
                        println!("  store critical FILENAME - Store an emergency file");
                        println!("  exit    - Exit the program");
                        println!("  help    - Show this help message");
                    }
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