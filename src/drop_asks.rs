// drop_asks.rs
use crate::dir_error::DirectoryState;
use crate::drop::DropCommand;
use std::io::{self, Write};

pub struct DropAsks;

impl DropAsks {
    pub fn new() -> Self {
        DropAsks
    }

    pub fn ask_for_drop(&self, dir_state: &DirectoryState) -> io::Result<()> {
        print!("Would you like to drop any files/folders? (y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().eq_ignore_ascii_case("y") {
            let drop_command = DropCommand::new();
            drop_command.handle_drop(dir_state)?;
        }

        Ok(())
    }
}
