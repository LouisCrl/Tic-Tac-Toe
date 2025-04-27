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
    pub fn init() -> Self {
        let (p1, p2) = Player::init();
        let players = (p1, p2);
        let current_turn = players.0.clone();
        Game {
            board: Board::init(),
            players,
            current_turn,
            state: GameState::Ongoing,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn current_turn(&self) -> &Player {
        &self.current_turn
    }

    pub fn play(&mut self, case: &str) -> bool {
        if self.state == GameState::Ongoing {
            if self.board.verify_case(case) {
                self.board.define_case(case, &self.current_turn.sign());
                self.board.print();
            }else {
                return false;
            }
        }
        
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
        
        self.change_turn();
        true
    }

    fn verify_win(&self) -> bool {
        self.board.check_lines(&self.current_turn.sign()) ||
        self.board.check_columns(&self.current_turn.sign()) ||
        self.board.check_diagonals(&self.current_turn.sign())
    }

    fn change_turn(&mut self) {
        self.current_turn = if *self.current_turn.sign() == Case::Cross {
            self.players.1.clone()
        }else{
            self.players.0.clone()
        }
    }
}