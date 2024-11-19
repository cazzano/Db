use std::env;
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use colored::*;

pub struct DirectoryNavigator {
    current_dir: PathBuf,
    base_dir: PathBuf,
}

impl DirectoryNavigator {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        Ok(DirectoryNavigator {
            current_dir: current_dir.clone(),
            base_dir: current_dir,
        })
    }

    pub fn change_directory(&mut self, dir: &str) -> Result<PathBuf, Box<dyn Error>> {
        // Create the new path, handling both absolute and relative paths
        let new_path = if PathBuf::from(dir).is_absolute() {
            PathBuf::from(dir)
        } else {
            self.current_dir.join(dir)
        };

        // Check if the path exists and is a directory
        if new_path.exists() && new_path.is_dir() {
            let canonical_path = new_path.canonicalize()?;
            
            // Try to change the current directory
            match env::set_current_dir(&canonical_path) {
                Ok(_) => {
                    self.current_dir = canonical_path.clone();
                    Ok(canonical_path)
                },
                Err(e) => Err(format!("Failed to change directory: {}", e).into())
            }
        } else {
            Err(DirectoryError::InvalidPath(format!("Directory not found: {}", dir)).into())
        }
    }


    pub fn list_directories(&self) -> Result<(), Box<dyn Error>> {
        println!("Current directory: {}", self.current_dir.display());
        println!("Contents:");

        let entries = fs::read_dir(&self.current_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Invalid filename");

            if metadata.is_dir() {
                println!("📁 {}", name.blue());
            } else {
                println!("📄 {}", name);
            }
        }

        Ok(())
    }

    pub fn get_current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn verify_path_exists(&self, path: &str) -> Result<PathBuf, Box<dyn Error>> {
        let full_path = if PathBuf::from(path).is_absolute() {
            PathBuf::from(path)
        } else {
            self.current_dir.join(path)
        };

        if full_path.exists() {
            Ok(full_path.canonicalize()?)
        } else {
            Err(format!("Path does not exist: {}", full_path.display()).into())
        }
    }

    pub fn is_subdirectory(&self, path: &PathBuf) -> bool {
        if let Ok(canonical_path) = path.canonicalize() {
            if let Ok(canonical_base) = self.base_dir.canonicalize() {
                canonical_path.starts_with(canonical_base)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_relative_path(&self, path: &PathBuf) -> Option<PathBuf> {
        if let Ok(canonical_path) = path.canonicalize() {
            if let Ok(canonical_current) = self.current_dir.canonicalize() {
                canonical_path.strip_prefix(canonical_current).ok().map(PathBuf::from)
            } else {
                None
            }
        } else {
            None
        }
    }
}

// Error type for directory operations
#[derive(Debug)]
pub enum DirectoryError {
    IoError(std::io::Error),
    InvalidPath(String),
    PermissionDenied(String),
}

impl std::fmt::Display for DirectoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DirectoryError::IoError(err) => write!(f, "IO Error: {}", err),
            DirectoryError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            DirectoryError::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
        }
    }
}

impl Error for DirectoryError {}

impl From<std::io::Error> for DirectoryError {
    fn from(err: std::io::Error) -> Self {
        DirectoryError::IoError(err)
    }
}