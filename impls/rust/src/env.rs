use std::collections::HashMap;
use crate::{
    types::{MalType, MalError},
};
use std::rc::Rc;
use std::cell::RefCell;
pub type MalEnv = Rc<RefCell<Env>>;

pub struct Env {
    data: HashMap<MalType, MalType>,
    parent: Option<MalEnv>,
}


fn create_mal_env(env: Env) -> MalEnv {
    Rc::new(RefCell::new(env))
}

pub fn set(me: &MalEnv, key: MalType, val: MalType) {
    me.borrow_mut().set(key, val);
}

pub fn get(me: &MalEnv, key: MalType) -> Option<MalType> {
    me.borrow_mut().get(key)
}

pub fn create_child(me: &MalEnv, key: MalType, val: MalType) -> MalEnv {
    let mut ret = Env {
        data: HashMap::new(),
        parent: None,
    };

    ret.parent =  Some(Rc::clone(me));
    ret.set(key, val);
    create_mal_env(ret)
}

pub fn init() -> MalEnv {
    let mut ret = Env {
        data: HashMap::new(),
        parent: None
    };

    ret.set(MalType::init_sym("+"), MalType::init_func(Env::add));
    ret.set(MalType::init_sym("*"), MalType::init_func(Env::mul));
    ret.set(MalType::init_sym("/"), MalType::init_func(Env::div));
    ret.set(MalType::init_sym("-"), MalType::init_func(Env::sub));

    create_mal_env(ret)
}


impl Env {
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
