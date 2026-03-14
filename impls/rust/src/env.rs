use std::collections::HashMap;
use ordered_float::OrderedFloat;
use crate::{
    types::{MalType, MalError},
};

pub struct Env {
    env: HashMap<String, fn(&[MalType]) -> Result<MalType, MalError>>
}

impl Env {
    pub fn init() -> Self {
        let mut env: HashMap<String, fn(&[MalType]) -> Result<MalType, MalError>> = HashMap::new();
        env.insert("+".into(), Env::add);
        env.insert("*".into(), Env::mul);
        env.insert("/".into(), Env::div);
        env.insert("-".into(), Env::sub);
        Self {
            env
        }
    }

    pub fn lookup(&self, input: MalType) -> Result<fn(&[MalType]) -> Result<MalType, MalError> , MalError> {
        if let MalType::Sym(s) = input {
            return self.map(&s);
        }

        Err(MalError::InvalidToken)
    }

    fn div(args: &[MalType]) -> Result<MalType, MalError> {
        if args.len() == 0 {
            return Err(MalError::InvalidArgsLen);
        }

        args.iter().skip(1).try_fold(args[0].clone(), |ret, mt| {
            match mt {
                MalType::Int(i) => {
                    match ret {
                        MalType::Int(y) => Ok(MalType::Int(y / i)),
                        MalType::Float(y) => Ok(MalType::Float(OrderedFloat(y.into_inner() / *i as f64))),
                        _ => Err(MalError::InvalidToken),
                    }
                }

                MalType::Float(f) => {
                    match ret {
                        MalType::Int(y) => Ok(MalType::Float(OrderedFloat(y as f64) / f.into_inner())),
                        MalType::Float(y) => Ok(MalType::Float(y / f)),
                        _ => Err(MalError::InvalidToken),
                    }
                },

                _ => Err(MalError::InvalidToken),
            }
        })
    }

    fn sub(args: &[MalType]) -> Result<MalType, MalError> {
        if args.len() == 0 {
            return Ok(MalType::Int(0));
        }

        args.iter().skip(1).try_fold(args[0].clone(), |ret, mt| {
            match mt {
                MalType::Int(i) => {
                    match ret {
                        MalType::Int(y) => Ok(MalType::Int(y - i)),
                        MalType::Float(y) => Ok(MalType::Float(OrderedFloat(y.into_inner() - *i as f64))),
                        _ => Err(MalError::InvalidToken),
                    }
                }

                MalType::Float(f) => {
                    match ret {
                        MalType::Int(y) => Ok(MalType::Float(OrderedFloat(y as f64) - f.into_inner())),
                        MalType::Float(y) => Ok(MalType::Float(y - f)),
                        _ => Err(MalError::InvalidToken),
                    }
                },

                _ => Err(MalError::InvalidToken),
            }
        })
    }

    fn mul(args: &[MalType]) -> Result<MalType, MalError> {
        args.iter().try_fold(MalType::Int(1), |ret, mt| {
            match mt {
                MalType::Int(i) => {
                    match ret {
                        MalType::Int(y) => Ok(MalType::Int(y * i)),
                        MalType::Float(y) => Ok(MalType::Float(OrderedFloat(y.into_inner() * *i as f64))),
                        _ => Err(MalError::InvalidToken),
                    }
                }

                MalType::Float(f) => {
                    match ret {
                        MalType::Int(y) => Ok(MalType::Float(OrderedFloat(y as f64) * f.into_inner())),
                        MalType::Float(y) => Ok(MalType::Float(y * f)),
                        _ => Err(MalError::InvalidToken),
                    }
                },

                _ => Err(MalError::InvalidToken),
            }
        })
    }

    fn add(args: &[MalType]) -> Result<MalType, MalError> {
        args.iter().try_fold(MalType::Int(0), |ret, mt| {
            match mt {
                MalType::Int(i) => {
                    match ret {
                        MalType::Int(y) => Ok(MalType::Int(y + i)),
                        MalType::Float(y) => Ok(MalType::Float(OrderedFloat(y.into_inner() + *i as f64))),
                        _ => Err(MalError::InvalidToken),
                    }
                }

                MalType::Float(f) => {
                    match ret {
                        MalType::Int(y) => Ok(MalType::Float(OrderedFloat(y as f64) + f.into_inner())),
                        MalType::Float(y) => Ok(MalType::Float(y + f)),
                        _ => Err(MalError::InvalidToken),
                    }
                },

                _ => Err(MalError::InvalidToken),
            }
        })
    }

    fn map(&self, s: &str)  -> Result<fn(&[MalType]) -> Result<MalType, MalError> , MalError> {
        if let Some(&func) = self.env.get(s) {
            return Ok(func);
        }

        Err(MalError::InvalidToken)
    }
}
