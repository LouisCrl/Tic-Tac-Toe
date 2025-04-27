//this file is the enum Case to give us what's in the boxes
#[derive(Debug, Clone, PartialEq)]
pub enum Case {
    Cross,
    Circle,
    Void,
}

impl Case {
    //returns the sign in character type for every variant of Case
    pub fn result(&self) -> char {
        match self{
            Case::Cross => 'X',
            Case::Circle => 'O',
            Case::Void => ' ',
        }
    }
}