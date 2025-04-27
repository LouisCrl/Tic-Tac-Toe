#[derive(Debug, Clone, PartialEq)]
pub enum Case {
    Cross,
    Circle,
    Void,
}

impl Case {
    pub fn result(&self) -> char {
        match self{
            Case::Cross => 'X',
            Case::Circle => 'O',
            Case::Void => ' ',
        }
    }
}