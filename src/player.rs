use crate::case::Case;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    sign: Case,
}

impl Player {
    pub fn init() -> (Self, Self) {
        (Player{sign: Case::Cross}, Player{sign: Case::Circle})
    }

    pub fn sign(&self) -> &Case {
        &self.sign
    }
}