// main.rs
mod clear;
mod dir_error;
mod dr;
mod edit;
mod editor;
mod help;
mod initialize;
mod main_1;
mod main_2;
mod store;
mod store_2;
mod store_3;
mod version; // New module

use main_1::run_command_loop;
use main_2::initialize_application;

fn main() {
    let (mut rl, mut dir_state, mut navigator) = initialize_application();
    run_command_loop(&mut rl, &mut dir_state, &mut navigator);
}
