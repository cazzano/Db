// main_4.rs
use crate::dir_error::DirectoryState;
use crate::drop_asks::DropAsks;
use std::fs;
use std::path::PathBuf;

pub fn copy_directory_contents(
    source: &PathBuf,
    destination: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = destination.join(path.file_name().unwrap());

        if path.is_dir() {
            copy_directory_contents(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

pub fn handle_post_store_actions(
    dir_state: &DirectoryState,
    drop_asks: &DropAsks,
) -> Result<(), Box<dyn std::error::Error>> {
    // Copy contents of Internet Urls directory if it exists
    if let Some(base_path) = dir_state.get_base_path() {
        let source_urls = base_path.join("Internet Urls");
        if let Some(target_dir) = dir_state.get_target_directory() {
            let target_urls = target_dir.join("Internet Urls");
            if source_urls.exists() {
                copy_directory_contents(&source_urls, &target_urls)?;
            }
        }
    }

    // Ask for drop
    drop_asks.ask_for_drop(dir_state)?;

    Ok(())
}
