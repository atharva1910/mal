use std::fmt;

#[derive (Debug)]
pub enum MalAtomType {
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
}

#[derive (Debug)]
pub enum MalType {
    Atom(MalAtomType),
    List(Vec<MalType>),
    Vec(Vec<MalType>),
}

impl MalType {
    pub fn init_list() -> MalType {
        MalType::List(Vec::new())
    }

    pub fn init_vec() -> MalType {
        MalType::Vec(Vec::new())
    }

    pub fn push(&mut self, input: MalType) {
        match self {
            Self::List(list) => list.push(input),
            Self::Vec(list) => list.push(input),
            _ => panic!("Non list type"),
        }
    }

    pub fn init_atom(input: String) -> Result<MalType, MalError> {
        Ok(MalType::Atom(MalAtomType::create_atom_type(input)?))
    }
}

impl MalAtomType {
    pub fn create_atom_type(input: String) -> Result<MalAtomType, MalError> {

        if input.parse::<f64>().is_ok() {
            return Ok(MalAtomType::Float(input.parse::<f64>().unwrap()));
        }

        if input.parse::<i64>().is_ok() {
            return Ok(MalAtomType::Int(input.parse::<i64>().unwrap()));
        }

        if input.starts_with("\"") {
            return Ok(MalAtomType::create_string_type(input)?);
        }

        return Ok(MalAtomType::create_symbol_type(input)?);
    }

    fn create_symbol_type(input:String) -> Result<MalAtomType, MalError> {
        match input.as_str() {
            "+" | "-" | "*" | "/" => Ok(MalAtomType::Sym(input)),
            "'" => Ok(MalAtomType::Sym("quote".to_string())),
            "`" => Ok(MalAtomType::Sym("quasiquote".to_string())),
            "~" => Ok(MalAtomType::Sym("unquote".to_string())),
            "~@" => Ok(MalAtomType::Sym("splice-unquote".to_string())),
            _ => Ok(MalAtomType::Sym(input)),
        }
    }

    fn create_string_type(input:String) -> Result<MalAtomType, MalError> {
        // The minimum string token can be "" which is of length 2
        if input.len() == 1 {
            return Err(MalError::Unbalanced);
        }

        let mut escape = false;
        for (i, c) in input.chars().enumerate() {
            // The first and the last character should always be a non-escaped "
            if i == 0 || i == input.len() - 1 {
                if escape || c != '"' {
                    return Err(MalError::Unbalanced);
                }
                continue;
            }

            if c == '"' {
                if !escape {
                    return Err(MalError::Unbalanced);
                } else {
                    escape = false;
                }
            }

            if c == '\\' {
                escape = !escape;
            }
        }

        Ok(MalAtomType::Str(input))
    }

    pub fn atom_to_string(self) -> String {
        match self {
            MalAtomType::Int(i) => i.to_string(),
            MalAtomType::Float(f) => f.to_string(),
            MalAtomType::Sym(c) => c,
            MalAtomType::Str(s) => s,
        }
    }
}

pub enum MalError {
    EOF,
    Unbalanced,
    ParsingError,
    InvalidToken,
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           MalError::EOF => write!(f, "EOF"),
           MalError::Unbalanced => write!(f, "unbalanced"),
           MalError::ParsingError => write!(f, "Failed to Parse MalString"),
           MalError::InvalidToken => write!(f, "Invalid Token"),
       }
    }
}
