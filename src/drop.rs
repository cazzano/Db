use crate::dir_error::DirectoryState;
use crate::progress::ProgressTracker;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct DropCommand;

impl DropCommand {
    pub fn new() -> Self {
        DropCommand
    }

    pub fn handle_drop(&self, dir_state: &DirectoryState) -> io::Result<()> {
        println!("Please drag and drop files/folders onto the terminal (separate by space, then press Enter):");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Improved path parsing that handles spaces in filenames
        let paths = parse_paths(&input);

        if paths.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No valid paths provided",
            ));
        }

        let progress_tracker = ProgressTracker::new();

        // Count total files first
        for path in &paths {
            if !path.exists() {
                println!("Warning: Path does not exist: {:?}", path);
                continue;
            }

            if path.is_file() {
                progress_tracker.increment_total_files();
            } else if path.is_dir() {
                for entry in WalkDir::new(path) {
                    if let Ok(entry) = entry {
                        if entry.path().is_file() {
                            progress_tracker.increment_total_files();
                        }
                    }
                }
            }
        }

        let current_dir = dir_state.get_current_path();

        // Process each path
        for path in paths {
            if !path.exists() {
                continue;
            }

            if path.is_file() {
                self.copy_file(&path, current_dir, &progress_tracker)?;
            } else if path.is_dir() {
                self.copy_directory(&path, current_dir, &progress_tracker)?;
            }
        }

        let (processed, total) = progress_tracker.get_progress();
        println!("\nCopying completed! Processed {processed} of {total} files");

        Ok(())
    }

    fn copy_file(
        &self,
        source: &Path,
        dest_dir: &Path,
        progress: &ProgressTracker,
    ) -> io::Result<()> {
        let file_name = source.file_name().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid source file name")
        })?;

        let destination = dest_dir.join(file_name);

        let pb = progress.create_progress_bar(1, &format!("Copying {:?}", file_name));
        fs::copy(source, destination)?;
        progress.increment_processed_files();
        pb.finish_with_message(format!("Copied {:?}", file_name));

        Ok(())
    }

    fn copy_directory(
        &self,
        source: &Path,
        dest_dir: &Path,
        progress: &ProgressTracker,
    ) -> io::Result<()> {
        let dir_name = source.file_name().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid source directory name")
        })?;

        let destination = dest_dir.join(dir_name);
        fs::create_dir_all(&destination)?;

        for entry in WalkDir::new(source) {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path
                .strip_prefix(source)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let target_path = destination.join(relative_path);

            if path.is_dir() {
                fs::create_dir_all(&target_path)?;
            } else if path.is_file() {
                let pb = progress.create_progress_bar(1, &format!("Copying {:?}", relative_path));
                fs::copy(path, &target_path)?;
                progress.increment_processed_files();
                pb.finish_with_message(format!("Copied {:?}", relative_path));
            }
        }

        Ok(())
    }
}

/// Helper function to parse paths from input string
fn parse_paths(input: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let mut current_path = String::new();
    let mut in_quotes = false;
    let mut escaped = false;

    for c in input.trim().chars() {
        match c {
            '"' if !escaped => {
                in_quotes = !in_quotes;
                // Add the quote character to preserve the original path
                current_path.push(c);
            }
            ' ' if !in_quotes => {
                if !current_path.is_empty() {
                    // Clean up the path and add it to the vector
                    if let Some(path) = clean_path(&current_path) {
                        paths.push(path);
                    }
                    current_path.clear();
                }
            }
            '\\' if !escaped => {
                escaped = true;
            }
            _ => {
                if escaped {
                    escaped = false;
                }
                current_path.push(c);
            }
        }
    }

    // Don't forget the last path
    if !current_path.is_empty() {
        if let Some(path) = clean_path(&current_path) {
            paths.push(path);
        }
    }

    paths
}

/// Helper function to clean up path string
fn clean_path(path: &str) -> Option<PathBuf> {
    let cleaned = path
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .trim()
        .to_string();

    if cleaned.is_empty() {
        None
    } else {
        Some(PathBuf::from(cleaned))
    }
}
