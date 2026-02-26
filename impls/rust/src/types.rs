use std::fmt;

#[derive (Debug)]
pub enum MalType {
    Atom(String),
    List(Vec<MalType>),
}

impl MalType {
    pub fn init_list() -> MalType {
        MalType::List(Vec::new())
    }

    pub fn push(&mut self, input: MalType) {
        match self {
            Self::List(list) => list.push(input),
            _ => panic!("Non list type"),
        }
    }

    pub fn init_atom(input: String) -> MalType {
        MalType::Atom(input)
    }
}

pub enum MalError {
    ParsingError,
    InvalidToken,
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           MalError::ParsingError => write!(f, "Failed to Parse MalString"),
           MalError::InvalidToken => write!(f, "Invalid Token"),
       }
    }
}
