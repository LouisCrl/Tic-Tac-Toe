//this file is the board management
use crate::case::Case;

#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<Case>>,
}

impl Board {
    //initialisation of an empty board
    pub fn init() -> Self {
        Board {
            board: vec![
                vec![Case::Void, Case::Void, Case::Void],
                vec![Case::Void, Case::Void, Case::Void],
                vec![Case::Void, Case::Void, Case::Void],
            ],
        }
    }

    //returns the board
    pub fn board(&self) -> &Vec<Vec<Case>> {
        &self.board
    }

    //verification if a box is available or if the box exist, if yes return true, else return false
    pub fn verify_case(&self, case: &str) -> bool {
        match case {
            "a1" => self.board[0][0] == Case::Void,
            "a2" => self.board[1][0] == Case::Void,
            "a3" => self.board[2][0] == Case::Void,
            "b1" => self.board[0][1] == Case::Void,
            "b2" => self.board[1][1] == Case::Void,
            "b3" => self.board[2][1] == Case::Void,
            "c1" => self.board[0][2] == Case::Void,
            "c2" => self.board[1][2] == Case::Void,
            "c3" => self.board[2][2] == Case::Void,
            _ => {
                println!("Invalid Case");
                return false;
            },
        }
    }

    //changes the box by the sign given
    pub fn define_case(&mut self, case: &str, sign: &Case) {
        match case {
            "a1" => self.board[0][0] = sign.clone(),
            "a2" => self.board[1][0] = sign.clone(),
            "a3" => self.board[2][0] = sign.clone(),
            "b1" => self.board[0][1] = sign.clone(),
            "b2" => self.board[1][1] = sign.clone(),
            "b3" => self.board[2][1] = sign.clone(),
            "c1" => self.board[0][2] = sign.clone(),
            "c2" => self.board[1][2] = sign.clone(),
            "c3" => self.board[2][2] = sign.clone(),
            _ => {
                println!("Invalid");
            },
        }
    }

    //returns a vector of string with every line of the board in it (usefull for the save in a .txt file)
    pub fn printed_board(&self) -> Vec<String> {
        let mut lines = Vec::new();
        let mut i: u8 = 0;
        for row in &self.board {
            i += 1;
            lines.push("     #   #   ".to_string());
            lines.push(format!("{}  {} # {} # {} ", i, row[0].result(), row[1].result(), row[2].result()));
            lines.push("     #   #   ".to_string());
            if i < 3 {
                lines.push("  ###########".to_string());
            } else {
                lines.push("   A   B   C ".to_string());
            }
        }
        lines.push("".to_string());

        lines
    }

    //print the game in the console
    pub fn print(&self) {
        for lines in &self.printed_board() {
            println!("{}", lines);
        }
    }

    //check the lines, if the player wins: return true, else return false
    pub fn check_lines(&self, sign: &Case) -> bool {
        for row in &self.board {
            if row[1] == *sign {
                if row[0] == row[1] && row[1] == row[2] {
                    return true;
                }
            }
        }
        false
    }

    //check the columns, if the player wins: return true, else return false
    pub fn check_columns(&self, sign: &Case) -> bool {
        for col in 0..3 {
            if self.board[1][col] == *sign {
                if &self.board[0][col] == &self.board[1][col] && &self.board[1][col] == &self.board[2][col] {
                    return true;
                }
            }
        }
        false
    }

    //check the diagonals, if the player wins: return true, else return false
    pub fn check_diagonals(&self, sign: &Case) -> bool {
        if self.board[1][1] == *sign {
            if &self.board[0][0] == &self.board[1][1] && &self.board[1][1] == &self.board[2][2] {
                return true;
            }else if &self.board[0][2] == &self.board[1][1] && &self.board[1][1] == &self.board[2][0] {
                return true;
            }
        }
        false
    }

    //check if the board is full (no Void variat), if the board is full: return true, else return false
    pub fn check_full(&self) -> bool {
        for row in &self.board {
            for case in row {
                if case == &Case::Void {
                    return false;
                }
            }
        }
        true
    }
}