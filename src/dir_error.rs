// dir_error.rs

use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DirectoryState {
    base_path: PathBuf,
    current_path: PathBuf,
    target_directory: Option<PathBuf>,
}

impl DirectoryState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let base_path = env::current_dir()?;
        Ok(DirectoryState {
            base_path: base_path.clone(),
            current_path: base_path,
            target_directory: None,
        })
    }

    pub fn get_base_path(&self) -> Option<&PathBuf> {
        Some(&self.base_path)
    }

    pub fn get_target_directory(&self) -> Option<&PathBuf> {
        self.target_directory.as_ref()
    }

    pub fn set_target_directory(&mut self, path: PathBuf) {
        self.target_directory = Some(path);
    }

    pub fn get_current_path(&self) -> &Path {
        &self.current_path
    }

    pub fn set_current_path<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path_ref = path.as_ref();
        let new_path = if path_ref.is_absolute() {
            path_ref.to_path_buf()
        } else {
            self.current_path.join(path_ref)
        };

        if new_path.exists() && new_path.is_dir() {
            self.current_path = new_path.canonicalize()?;
            env::set_current_dir(&self.current_path)?;
            Ok(())
        } else {
            Err(format!("Invalid directory path: {}", path_ref.display()).into())
        }
    }

    pub fn resolve_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        let path_ref = path.as_ref();
        if path_ref.is_absolute() {
            path_ref.to_path_buf()
        } else {
            self.current_path.join(path_ref)
        }
    }

    pub fn verify_file_exists<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path_ref = file_path.as_ref();
        let full_path = self.resolve_path(path_ref);
        let canonical_path = full_path.canonicalize().map_err(|_| {
            format!(
                "File '{}' not found in current directory: {}",
                full_path.file_name().unwrap_or_default().to_string_lossy(),
                self.current_path.display()
            )
        })?;

        if canonical_path.is_file() {
            Ok(canonical_path)
        } else {
            Err(format!(
                "Path '{}' exists but is not a file",
                canonical_path.display()
            )
            .into())
        }
    }

    pub fn sync_with_navigator<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path_ref = path.as_ref();
        let absolute_path = if path_ref.is_absolute() {
            path_ref.to_path_buf()
        } else {
            self.current_path.join(path_ref)
        };

        let canonical_path = absolute_path.canonicalize()?;
        self.current_path = canonical_path.clone();
        env::set_current_dir(&canonical_path)?;
        Ok(())
    }

    pub fn get_base_directory(&self) -> &PathBuf {
        &self.base_path
    }

    pub fn get_current_directory(&self) -> &PathBuf {
        &self.current_path
    }
}

pub fn initialize_directory_state() -> Result<DirectoryState, Box<dyn std::error::Error>> {
    DirectoryState::new()
}
