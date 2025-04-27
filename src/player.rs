//this file is the players informations
use crate::case::Case;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    sign: Case,
}

impl Player {
    //initialisation of the 2 players (X and O)
    pub fn init() -> (Self, Self) {
        (Player{sign: Case::Cross}, Player{sign: Case::Circle})
    }

    //returns the Case variant (Cross, Circle) of every players
    pub fn sign(&self) -> &Case {
        &self.sign
    }
}