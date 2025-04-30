//this file is the core program of the game progress
use std::io;
use crate::case::Case;
use crate::board::Board;
use crate::player::Player;

#[derive(Debug, PartialEq)]
pub enum GameState {
    Ongoing,
    Win(Player),
    Draw,
}

pub struct Game {
    board: Board,
    players: (Player, Player),
    current_turn: Player,
    state: GameState,
}

impl Game {
    //initialisation of the game
    pub fn init() -> Self {
        let mode: u32 = loop {
            println!("Want to play:");
            println!("1/ Alone");
            println!("2/ Multi");
        
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            
            match input.trim().parse() {
                Ok(num) if num == 1 || num == 2 => break num,
                _ => {
                    println!("Invalid input. Please enter 1 or 2.");
                }
            }
        };
        
        let (p1, p2): (Player, Player) = if mode == 1 {
            Player::init_alone()
        }else {
            Player::init_multi()
        };
        
        let players = (p1, p2);
        let current_turn = players.0.clone();
        Game {
            board: Board::init(),
            players,
            current_turn,
            state: GameState::Ongoing,
        }
    }

    //returns the board
    pub fn board(&self) -> &Board {
        &self.board
    }

    //returns the gamestate
    pub fn state(&self) -> &GameState {
        &self.state
    }

    //returns the current_turn
    pub fn current_turn(&self) -> &Player {
        &self.current_turn
    }

    //the main method for the progress of one turn
    pub fn play(&mut self, case: &str) -> bool {
        if self.state == GameState::Ongoing {
            if self.board.verify_case(case) {
                self.board.define_case(case, &self.current_turn.sign());
                self.board.print();
            }else {
                return false;
            }
        }
        
        if self.check_end() {
            return true;
        }

        self.change_turn();
        true
    }

    fn check_end(&mut self) -> bool {
        if self.verify_win() {
            self.state = GameState::Win(self.current_turn.clone());
            println!("Player {} win !", match self.current_turn.sign() {
                Case::Cross => "X",
                Case::Circle => "O",
                _ => "?",
            });
            return true;
        }
        
        if self.board.check_full() {
            self.state = GameState::Draw;
            println!("Draw, nobody wins !");
            return true;
        }
        false
    }

    //the checks if there is a winner, return true if there is one, else return false
    fn verify_win(&self) -> bool {
        self.board.check_lines(&self.current_turn.sign()) ||
        self.board.check_columns(&self.current_turn.sign()) ||
        self.board.check_diagonals(&self.current_turn.sign())
    }

    //change the current_player turn
    fn change_turn(&mut self) {
        self.current_turn = if *self.current_turn.sign() == Case::Cross {
            self.players.1.clone()
        }else{
            self.players.0.clone()
        }
    }
}