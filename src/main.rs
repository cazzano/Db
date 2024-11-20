// main.rs (remains unchanged)
mod clear;
mod dir_error;
mod dr;
mod drop;
mod drop_asks;
mod edit;
mod editor;
mod help;
mod initialize;
mod main_1;
mod main_2;
mod main_3; // Add this line
mod main_4;
mod progress;
mod store;
mod store_2;
mod store_3;
mod version;

use main_1::run_command_loop;
use main_2::initialize_application;

fn main() {
    let (mut rl, mut dir_state, mut navigator) = initialize_application();
    run_command_loop(&mut rl, &mut dir_state, &mut navigator);
}
