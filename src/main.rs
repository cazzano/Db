// main.rs
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;  // Added back the import
mod version;
mod clear;
mod initialize;
mod dr;
mod store;
mod store_2;
mod store_3;
mod editor;
mod help;
mod edit;
mod dir_error;
mod main_1;

use main_1::run_command_loop;
use dir_error::initialize_directory_state;
use dr::DirectoryNavigator;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    
    // Initialize DirectoryState
    let mut dir_state = match initialize_directory_state() {
        Ok(state) => state,
        Err(e) => {
            println!("Failed to initialize directory state: {}. Please run 'init' first.", e);
            panic!("Failed to initialize directory state");
        }
    };

    let mut navigator = match DirectoryNavigator::new() {
        Ok(nav) => nav,
        Err(e) => {
            println!("Failed to initialize directory navigator: {}. Please run 'init' first.", e);
            DirectoryNavigator::new().unwrap_or_else(|_| panic!("Failed to create navigator"))
        }
    };

    run_command_loop(&mut rl, &mut dir_state, &mut navigator);
}