use crate::dir_error::DirectoryState;
use crate::editor::Editor;
use std::path::Path;

pub struct EditCommand {
    editor: Editor,
}

impl EditCommand {
    pub fn new() -> Self {
        EditCommand {
            editor: Editor::new(),
        }
    }

    pub fn edit_file<P: AsRef<Path>>(
        &self,
        filename: P,
        dir_state: &DirectoryState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Remove quotes from the filename if present
        let clean_filename = filename
            .as_ref()
            .to_str()
            .map(|s| s.trim_matches('"'))
            .unwrap_or_else(|| filename.as_ref().to_str().unwrap());

        // Verify file exists and get its full path
        let file_path = dir_state.verify_file_exists(clean_filename)?;

        // Use the Editor to edit the file
        self.editor.edit_file(file_path)?;

        Ok(())
    }
}
