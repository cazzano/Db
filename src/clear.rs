// src/clear.rs
pub fn clear_screen() {
    if let Err(e) = clearscreen::clear() {
        println!("Error clearing screen: {}", e);
    }
}