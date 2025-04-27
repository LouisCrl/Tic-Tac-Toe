//this file is the saving system
use std::fs;
use std::path::Path;
use std::io::{Write, Result};
use std::fs::{File, OpenOptions};
use crate::game::Game;

pub struct Save;

impl Save {
    //write every turns in a file named save.txt
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

    //delete the previous inscriptions in a file named save.txt and if doesn't exist, create one
    pub fn delete_previous() -> Result<()> {
        File::create("history/save.txt")?;

        Ok(())
    }

    //rename a file
    pub fn rename_file(old_name: &str, new_name: &str) -> std::io::Result<()> {
        fs::rename(old_name, new_name)?;
        Ok(())
    }

    //return a string of the new file index (to get the the (1) in the file Draw(1).txt)
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
