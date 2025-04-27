use std::fs;
use std::path::Path;
use std::io::{Write, Result};
use std::fs::{File, OpenOptions};
use crate::game::Game;

pub struct Save;

impl Save {
    pub fn write_to_file(game: &Game) -> Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open("history/save.txt")?;
        for line in game.board().printed_board() {
            if let Err(e) = writeln!(file, "{}", line) {
                eprintln!("Failed to write to file: {}", e);
                return Err(e);
            }
        }
        writeln!(file, "Current turn: {:?}", game.current_turn().sign())?;
        writeln!(file, "Game state: {:?}", game.state())?;

        writeln!(file, "----------------------------")?;

        Ok(())
    }

    pub fn delete_previous() -> Result<()> {
        File::create("history/save.txt")?;

        Ok(())
    }

    pub fn rename_file(old_name: &str, new_name: &str) -> std::io::Result<()> {
        fs::rename(old_name, new_name)?;
        Ok(())
    }

    pub fn get_new_file_index(base_name: &str) -> String {
        let mut index = 1;
        let mut new_file_name = format!("{}({})", base_name, index);

        while Path::new(&format!("{}.txt", new_file_name)).exists() {
            index += 1;
            new_file_name = format!("{}({})", base_name, index);
        }

        format!("{}", index)
    }
}
