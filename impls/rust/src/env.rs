use std::collections::HashMap;
use crate::{
    types::{MalAtomType, MalType, MalError},
};

pub struct Env {
}

impl Env {
    pub fn map(s: &str) -> Result<fn (), MalError> {
        match s {
            "+" => {
                return Ok(|| {panic!("++++++")});
            },
            _ => return Err(MalError::InvalidToken),
        }
    }
}
