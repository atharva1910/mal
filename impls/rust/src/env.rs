use std::collections::HashMap;
use crate::{
    types::{MalAtomType, MalType, MalError},
};

pub struct Env {
}

impl Env {
    fn add(args: &[MalType]) -> Result<MalType, MalError> {
        let mut ret: f64 = 0.0;
        for arg in args {
            if let MalType::Atom(mat) = arg {
                match mat {
                    MalAtomType::Float(f) => ret += f.into_inner(),
                    _ => panic!("Other Mal Atom Type not handled"),
                }
            }
        }

        MalType::init_atom(ret.to_string())
    }

    fn map(s: &str)  -> Result<fn(&[MalType]) -> Result<MalType, MalError> , MalError> {
        match s {
            "+" => Ok(Env::add),
            _ => Err(MalError::InvalidToken)
        }
    }

    pub fn lookup(input: Option<MalType>) -> Result<fn(&[MalType]) -> Result<MalType, MalError> , MalError> {
        if let Some(MalType::Atom(mal)) = input {
            if let MalAtomType::Sym(s) = mal {
                return Env::map(&s);
            }
        }

        Err(MalError::InvalidToken)
    }
}
