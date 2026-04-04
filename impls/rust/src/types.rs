use std::fmt;
use std::collections::{HashMap, VecDeque};
use ordered_float::OrderedFloat;
use std::hash::{ Hash, Hasher};

type MalFunc = fn(&[MalType]) -> Result<MalType, MalError>;

#[derive (Debug, Eq, PartialEq, Clone)]
pub enum MalType {
    Int(i64),
    Float(OrderedFloat<f64>),
    Str(String),
    Sym(String),
    List(VecDeque<MalType>),
    Vec(Vec<MalType>),
    Hash(HashMap<MalType, MalType>),
    Func(MalFunc),
}

impl Hash for MalType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            MalType::Int(i) => i.hash(state),
            MalType::Float(f) => f.hash(state),
            MalType::Str(s) | MalType::Sym(s) =>  s.hash(state),
            MalType::Func(f) => f.hash(state),

            MalType::List(list) => {
                for mat in list {
                    mat.hash(state);
                }
            },

            MalType::Vec(list) => {
                for mat in list {
                    mat.hash(state);
                }
            }
            MalType::Hash(hash) => {
                for (k, v) in hash.iter() {
                    k.hash(state);
                    v.hash(state);
                }
            }
        }
    }
}

impl MalType {
    pub fn init_sym(input: &str) -> MalType {
        MalType::Sym(input.into())
    }

    pub fn init_list() -> MalType {
        MalType::List(VecDeque::new())
    }

    pub fn init_vec() -> MalType {
        MalType::Vec(Vec::new())
    }

    pub fn init_dict() -> MalType {
        MalType::Hash(HashMap::new())
    }

    pub fn init_func(f: MalFunc) -> MalType {
        MalType::Func(f)
    }

    pub fn push(&mut self, input: MalType) {
        match self {
            Self::List(list) => list.push_back(input),
            Self::Vec(list) => list.push(input),
            _ => panic!("Non list type"),
        }
    }

    pub fn init_atom(input: String) -> Result<MalType, MalError> {
        Ok(MalType::create_atom_type(input)?)
    }

    pub fn insert(&mut self, key: MalType, value: MalType) {
        match self {
            Self::Hash(hash) => _ = hash.insert(key, value),
            _ => panic!("Non list type"),
        }
    }

    pub fn create_atom_type(input: String) -> Result<MalType, MalError> {
        if input.parse::<i64>().is_ok() {
            return Ok(MalType::Int(input.parse::<i64>().unwrap()));
        }

        if input.parse::<f64>().is_ok() {
            return Ok(MalType::Float(OrderedFloat(input.parse::<f64>().unwrap())));
        }

        if input.starts_with("\"") {
            return Ok(MalType::create_string_type(input)?);
        }

        return Ok(MalType::create_symbol_type(input)?);
    }

    pub fn to_bool(input: Option<&MalType>) -> bool {
        let Some(_) = input else {
            return false;
        };
        true
    }

    fn create_symbol_type(input:String) -> Result<MalType, MalError> {
        match input.as_str() {
            "+" | "-" | "*" | "/" => Ok(MalType::Sym(input)),
            "'" => Ok(MalType::Sym("quote".into())),
            "@" => Ok(MalType::Sym("deref".into())),
            "`" => Ok(MalType::Sym("quasiquote".into())),
            "~" => Ok(MalType::Sym("unquote".into())),
            "~@" => Ok(MalType::Sym("splice-unquote".into())),
            "^" => Ok(MalType::Sym("with-meta".into())),
            _ => Ok(MalType::Sym(input)),
        }
    }

    fn create_string_type(input:String) -> Result<MalType, MalError> {
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
            } else if c == '\\' {
                escape = !escape;
            } else if c == 'n'  && escape {
                escape = false;
            }
        }

        Ok(MalType::Str(input))
    }
}

pub enum MalError {
    EOF,
    Unbalanced,
    ParsingError,
    InvalidToken,
    InvalidArgsLen,
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           MalError::EOF => write!(f, "EOF"),
           MalError::Unbalanced => write!(f, "unbalanced"),
           MalError::InvalidToken => write!(f, "not found"),
           MalError::ParsingError => write!(f, "Failed to Parse MalString"),
           MalError::InvalidArgsLen => write!(f, "Invalid Length of arguments"),
       }
    }
}

impl fmt::Display for MalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MalType::Int(i) => write!(f, "{}", i),
            MalType::Float(x) => write!(f, "{}", x),

            MalType::Str(ref s) => write!(f, "{}", s),
            MalType::Sym(ref s) => write!(f, "{}", s),

            MalType::Vec(ref v) => {
                write!(f, "[")?;
                v.iter().for_each(|mt| {
                    _ = write!(f, "{}", mt.clone());
                });
                write!(f, "]")
            }

            MalType::List(ref l) => {
                write!(f, "(")?;
                l.iter().for_each(|mt| {
                    _ = write!(f, "{}", mt.clone());
                });
                write!(f, ")")
            }

            MalType::Hash(ref h) => {
                write!(f, "{{")?;
                h.iter().for_each(|(k, v)| {
                    _ = write!(f, "{} : {}", k.clone(), v.clone());
                });
                write!(f, "}}")
            }

            MalType::Func(_) => Ok(()),
       }
    }
}
