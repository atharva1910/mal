use std::collections::HashMap;
use crate::{
    types::{MalType, MalError},
};

pub struct EnvStack {
    stack: Vec<Env>
}

pub struct Env {
    data: HashMap<MalType, MalType>
}

impl Env {
    pub fn init() -> Self {
        let mut ret = Self {
            data: HashMap::new()
        };

        ret.set(MalType::init_sym("+"), MalType::init_func(Env::add));
        ret.set(MalType::init_sym("*"), MalType::init_func(Env::mul));
        ret.set(MalType::init_sym("/"), MalType::init_func(Env::div));
        ret.set(MalType::init_sym("-"), MalType::init_func(Env::sub));

        ret
    }

    pub fn set(&mut self, key: MalType, val: MalType) {
        self.data.insert(key, val);
    }

    pub fn get(&mut self, key: MalType) -> Option<MalType> {
        self.data.get(&key).cloned()
    }

    fn div(args: &[MalType]) -> Result<MalType, MalError> {
        if args.len() == 0 {
            return Err(MalError::InvalidArgsLen);
        }

        args.iter().skip(1).try_fold(args[0].clone(), |ret, mt| {
            ret / mt.clone()
        })
    }

    fn sub(args: &[MalType]) -> Result<MalType, MalError> {
        if args.len() == 0 {
            return Ok(MalType::Int(0));
        }

        args.iter().skip(1).try_fold(args[0].clone(), |ret, mt| {
            ret - mt.clone()
        })
    }

    fn mul(args: &[MalType]) -> Result<MalType, MalError> {
        args.iter().try_fold(MalType::Int(1), |ret, mt| {
            ret * mt.clone()
        })
    }

    fn add(args: &[MalType]) -> Result<MalType, MalError> {
        args.iter().try_fold(MalType::Int(0), |ret, mt| {
            ret + mt.clone()
        })
    }
}
