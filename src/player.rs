//this file is the players informations
use crate::case::Case;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Human,
    Bot,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    p_type: Type,
    sign: Case,
}

impl Player {
    //initialisation of the 2 players (X and O)
    pub fn init_multi() -> (Self, Self) {
        (Player{p_type: Type::Human, sign: Case::Cross}, Player{p_type: Type::Human, sign: Case::Circle})
    }

    //initialisation of the 1 players (X) and a 1 bot (O)
    pub fn init_alone() -> (Self, Self) {
        (Player{p_type: Type::Human, sign: Case::Cross}, Player{p_type: Type::Bot, sign: Case::Circle})
    }

    //returns the Type variant (Human, Bot) of every players
    pub fn p_type(&self) -> &Type {
        &self.p_type
    }

    //returns the Case variant (Cross, Circle) of every players
    pub fn sign(&self) -> &Case {
        &self.sign
    }
}