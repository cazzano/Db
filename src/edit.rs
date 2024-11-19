// edit.rs

use std::path::Path;
use crate::editor::Editor;
use crate::dir_error::DirectoryState;

pub struct EditCommand {
    editor: Editor,
}

impl EditCommand {
    pub fn new() -> Self {
        EditCommand {
            editor: Editor::new(),
        }
    }

    pub fn edit_file<P: AsRef<Path>>(&self, filename: P, dir_state: &DirectoryState) -> Result<(), Box<dyn std::error::Error>> {
        // Verify file exists and get its full path
        let file_path = dir_state.verify_file_exists(filename)?;
        
        // Use the Editor to edit the file
        self.editor.edit_file(file_path)?;
        
        Ok(())
    }
}