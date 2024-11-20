// help.rs

pub fn show_help() {
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
    println!("  edit    - Edits an file");
    println!("  drop    - Copy file or folder , when you drag and drop files or folders");
    println!("  help    - Show this help message");
}
