use std::fmt;

#[derive (Debug)]
pub enum MalAtomType {
    Int(i64),
    Float(f64),
    Str(String),
    Sym(char),
}

#[derive (Debug)]
pub enum MalType {
    Atom(MalAtomType),
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
        MalType::Atom(MalAtomType::create_atom_type(input))
    }
}

impl MalAtomType {
    pub fn create_atom_type(input: String) -> MalAtomType {
        if input.len() == 1 && ("+-/*").contains(&input) {
            return MalAtomType::Sym(input.chars().next().unwrap());
        }

        if input.parse::<f64>().is_ok() {
            return MalAtomType::Float(input.parse::<f64>().unwrap());
        }

        if input.parse::<i64>().is_ok() {
            return MalAtomType::Int(input.parse::<i64>().unwrap());
        }

        MalAtomType::Str(input)
    }

    pub fn atom_to_string(self) -> String {
        match self {
            MalAtomType::Int(i) => i.to_string(),
            MalAtomType::Float(f) => f.to_string(),
            MalAtomType::Sym(c) => c.to_string(),
            MalAtomType::Str(s) => s,
        }
    }
}

pub enum MalError {
    EOF,
    ParsingError,
    InvalidToken,
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           MalError::EOF => write!(f, "EOF"),
           MalError::ParsingError => write!(f, "Failed to Parse MalString"),
           MalError::InvalidToken => write!(f, "Invalid Token"),
       }
    }
}
