use std::collections::HashMap;
use crate::{
    types::{MalAtomType, MalType, MalError},
};

pub struct Env {
    env: HashMap<String, fn(&[MalType]) -> Result<MalType, MalError>>
}

impl Env {
    pub fn init() -> Self {
        let mut env: HashMap<String, fn(&[MalType]) -> Result<MalType, MalError>> = HashMap::new();
        env.insert("+".into(), Env::add);
        env.insert("*".into(), Env::mul);
        Self {
            env
        }
    }

    pub fn lookup(&self, input: Option<MalType>) -> Result<fn(&[MalType]) -> Result<MalType, MalError> , MalError> {
        if let Some(MalType::Atom(mal)) = input {
            if let MalAtomType::Sym(s) = mal {
                return self.map(&s);
            }
        }

        Err(MalError::InvalidToken)
    }
}

impl Env {
    fn add(args: &[MalType]) -> Result<MalType, MalError> {
        let mut ret: i64 = 0;
        for arg in args {
            if let MalType::Atom(mat) = arg {
                match mat {
                    MalAtomType::Int(i) => ret += i,
                    _ => panic!("Other Mal Atom Type not handled"),
                }
            }
        }
        MalType::init_atom(ret.to_string())
    }

    fn mul(args: &[MalType]) -> Result<MalType, MalError> {
        let mut ret : i64 = 1;
        for arg in args {
            if let MalType::Atom(mat) = arg {
                match mat {
                    MalAtomType::Int(i) => ret *= i,
                    _ => panic!("Other Mal Atom Type not handled"),
                }
            }
        }
        MalType::init_atom(ret.to_string())
    }


    fn sub(args: &[MalType]) -> Result<MalType, MalError> {
        let mut ret:MalAtomType = args[0];

        for (idx, arg) in args.iter().enumerate() {
            if let MalType::Atom(mat) = arg {
                match mat {
                    MalAtomType::Int(_) | MalAtomType::Float(_) if idx == 0 => ret = mat,

                    MalAtomType::Int(i) => {
                    },

                    MalAtomType::Float(f) => {
                    },

                    _ => panic!("Other Mal Atom Type not handled"),
                }
            }

            return Err(MalError::InvalidSymbol);
        }

        Ok(ret)
    }


    fn map(&self, s: &str)  -> Result<fn(&[MalType]) -> Result<MalType, MalError> , MalError> {
        if let Some(&func) = self.env.get(s) {
            return Ok(func);
        }

        Err(MalError::InvalidToken)
    }


}
