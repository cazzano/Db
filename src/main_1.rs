// main_1.rs
use crate::{
    clear, dir_error::DirectoryState, dr::DirectoryNavigator, drop::DropCommand,
    drop_asks::DropAsks, edit::EditCommand, help, initialize, main_3::handle_store_commands,
    store::UrlStore, store_2::SecretStore, store_3::EmergencyStore, version,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub fn run_command_loop(
    rl: &mut DefaultEditor,
    dir_state: &mut DirectoryState,
    navigator: &mut DirectoryNavigator,
) {
    let url_store = match UrlStore::new() {
        Ok(store) => store,
        Err(e) => {
            println!(
                "Failed to initialize URL store: {}. Please run 'init' first.",
                e
            );
            UrlStore::new().unwrap_or_else(|_| panic!("Failed to create URL store"))
        }
    };

    let secret_store = match SecretStore::new() {
        Ok(store) => store,
        Err(e) => {
            println!(
                "Failed to initialize Secret store: {}. Please run 'init' first.",
                e
            );
            SecretStore::new().unwrap_or_else(|_| panic!("Failed to create Secret store"))
        }
    };

    let emergency_store = match EmergencyStore::new() {
        Ok(store) => store,
        Err(e) => {
            println!(
                "Failed to initialize Emergency store: {}. Please run 'init' first.",
                e
            );
            EmergencyStore::new().unwrap_or_else(|_| panic!("Failed to create Emergency store"))
        }
    };

    let edit_command = EditCommand::new();
    let drop_command = DropCommand::new();
    let drop_asks = DropAsks::new();

    println!("Welcome to DB MG Tool. Type 'help' for help, 'exit' to quit.");

    loop {
        let readline = rl.readline("db-mg> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();

                if !handle_navigation_and_edit(&line, dir_state, navigator, &edit_command) {
                    handle_store_commands(
                        &line,
                        dir_state,
                        &url_store,
                        &secret_store,
                        &emergency_store,
                        &drop_asks,
                    );
                    handle_basic_commands(&line, dir_state, &drop_command, navigator);
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

fn handle_navigation_and_edit(
    line: &str,
    dir_state: &mut DirectoryState,
    navigator: &mut DirectoryNavigator,
    edit_command: &EditCommand,
) -> bool {
    if line.trim().starts_with("cd ") {
        let path_str = line.trim().strip_prefix("cd ").unwrap().trim();
        let dir = if path_str.starts_with("\"") && path_str.ends_with("\"") {
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
        return true;
    }

    if line.trim().starts_with("edit ") {
        let filename = line.trim().strip_prefix("edit ").unwrap().trim();
        if let Err(e) = edit_command.edit_file(filename, dir_state) {
            println!("Error editing file: {}", e);
        }
        return true;
    }

    false
}

fn handle_basic_commands(
    line: &str,
    dir_state: &mut DirectoryState,
    drop_command: &DropCommand,
    navigator: &mut DirectoryNavigator,
) {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    match parts.get(0).map(|&s| s) {
        Some("drop") => {
            if let Err(e) = drop_command.handle_drop(dir_state) {
                println!("Error handling drop: {}", e);
            }
        }
        Some("version") => version::show_version(),
        Some("clear") => clear::clear_screen(),
        Some("init") => {
            if let Err(e) = initialize::init_directories() {
                println!("Initialization failed: {}", e);
            }
        }
        Some("ls") => {
            if let Err(e) = navigator.list_directories() {
                println!("Error listing directories: {}", e);
            }
        }
        Some("exit") | Some("quit") => {
            println!("Goodbye!");
        }
        Some("help") => help::show_help(),
        Some("") => (),
        Some(cmd) => println!("Unknown command: {}", cmd),
        None => (),
    }
}
