// main_3.rs (modified version)
use crate::{
    dir_error::DirectoryState, drop_asks::DropAsks, main_4::handle_post_store_actions,
    store::UrlStore, store_2::SecretStore, store_3::EmergencyStore,
};

pub fn handle_store_commands(
    line: &str,
    dir_state: &mut DirectoryState,
    url_store: &UrlStore,
    secret_store: &SecretStore,
    emergency_store: &EmergencyStore,
    drop_asks: &DropAsks,
) {
    if line.trim().starts_with("store url ") {
        let filename = line.trim().strip_prefix("store url ").unwrap().trim();
        if let Err(e) = url_store.store_url(filename) {
            println!("Error storing URL: {}", e);
        } else {
            if let Err(e) = handle_post_store_actions(dir_state, drop_asks) {
                println!("Error during post-store actions: {}", e);
            }
        }
        return;
    }

    if line.trim().starts_with("store secret ") {
        let filename = line.trim().strip_prefix("store secret ").unwrap().trim();
        if let Err(e) = secret_store.store_secret(filename) {
            println!("Error storing secret: {}", e);
        } else {
            if let Err(e) = drop_asks.ask_for_drop(dir_state) {
                println!("Error during drop prompt: {}", e);
            }
        }
        return;
    }

    if line.trim().starts_with("store critical ") {
        let filename = line.trim().strip_prefix("store critical ").unwrap().trim();
        if let Err(e) = emergency_store.store_critical(filename, dir_state) {
            println!("Error storing emergency file: {}", e);
        } else {
            if let Err(e) = drop_asks.ask_for_drop(dir_state) {
                println!("Error during drop prompt: {}", e);
            }
        }
    }
}
