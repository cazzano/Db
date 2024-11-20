use colored::*;
use dialoguer::{Confirm, Select};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Editor;

#[derive(Debug)]
enum EditorType {
    Nano,
    Vim,
    Neovim,
    RustEditor,
}

impl EditorType {
    fn get_command(&self) -> &str {
        match self {
            EditorType::Nano => "nano",
            EditorType::Vim => "vim",
            EditorType::Neovim => "nvim",
            EditorType::RustEditor => "rust_editor",
        }
    }

    fn get_available_editors() -> Vec<EditorType> {
        let editors = vec![
            (EditorType::Nano, "nano"),
            (EditorType::Vim, "vim"),
            (EditorType::Neovim, "nvim"),
        ];

        // Always include RustEditor since it's built-in
        let available: Vec<EditorType> = editors
            .into_iter()
            .filter(|(_, cmd)| {
                Command::new("which")
                    .arg(cmd)
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            })
            .map(|(editor, _)| editor)
            .collect();

        let mut final_editors = available;
        final_editors.push(EditorType::RustEditor);
        final_editors
    }
}

impl Editor {
    pub fn new() -> Self {
        Editor
    }

    fn rust_based_editor<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut editor = DefaultEditor::new()?;
        let content = fs::read_to_string(file_path.as_ref())?;
        let current_content = content.lines().collect::<Vec<_>>();

        println!("Rust-based Editor Started. Press Ctrl+D to save and exit.");
        println!("Current content:");
        for (i, line) in current_content.iter().enumerate() {
            println!("{}: {}", i + 1, line);
        }

        let mut new_content = Vec::new();
        loop {
            match editor.readline("edit> ") {
                Ok(line) => {
                    editor.add_history_entry(line.as_str())?;
                    new_content.push(line);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Cancelled editing");
                    return Ok(());
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    return Err(err.into());
                }
            }
        }

        if !new_content.is_empty() {
            fs::write(file_path, new_content.join("\n"))?;
            println!("{} File saved successfully", "✓".green());
        }

        Ok(())
    }

    pub fn edit_file<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let edit_file = Confirm::new()
            .with_prompt("Do you want to edit this file now? (y/n)")
            .interact()?;

        if edit_file {
            let available_editors = EditorType::get_available_editors();

            if available_editors.is_empty() {
                return Err("No supported editors found on the system".into());
            }

            let editor_names: Vec<&str> = available_editors
                .iter()
                .map(|e| match e {
                    EditorType::Nano => "nano",
                    EditorType::Vim => "vim",
                    EditorType::Neovim => "neovim",
                    EditorType::RustEditor => "rust_editor",
                })
                .collect();

            let selection = Select::new()
                .with_prompt("Choose your editor")
                .items(&editor_names)
                .default(0)
                .interact()?;

            match &available_editors[selection] {
                EditorType::RustEditor => {
                    self.rust_based_editor(&file_path)?;
                }
                _ => {
                    let editor_command = available_editors[selection].get_command();
                    let status = Command::new(editor_command)
                        .arg(file_path.as_ref())
                        .status()?;

                    if status.success() {
                        println!(
                            "{} File edited successfully with {}",
                            "✓".green(),
                            editor_names[selection]
                        );
                    } else {
                        println!(
                            "{} Failed to edit file with {}",
                            "✗".red(),
                            editor_names[selection]
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
