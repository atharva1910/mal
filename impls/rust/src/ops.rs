use std::ops::{Add, Mul, Div, Sub};
use ordered_float::OrderedFloat;
use crate::types::{MalError, MalType};

impl Add for MalType {
    type Output = Result<MalType, MalError>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            MalType::Int(s) => {
                match rhs {
                    MalType::Int(i) => Ok(MalType::Int(s + i)),
                    MalType::Float(f) => Ok(MalType::Float(OrderedFloat(f.into_inner() + s as f64))),
                    _ => Err(MalError::InvalidToken),
                }
            }

            MalType::Float(s) => {
                match rhs {
                    MalType::Int(i) => Ok(MalType::Float(OrderedFloat(i as f64) + s.into_inner())),
                    MalType::Float(f) => Ok(MalType::Float(f + s)),
                    _ => Err(MalError::InvalidToken),
                }
            },

            _ => Err(MalError::InvalidToken),
        }
    }

}

impl Mul for MalType {
    type Output = Result<MalType, MalError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            MalType::Int(s) => {
                match rhs {
                    MalType::Int(i) => Ok(MalType::Int(s * i)),
                    MalType::Float(f) => Ok(MalType::Float(OrderedFloat(f.into_inner() * s as f64))),
                    _ => Err(MalError::InvalidToken),
                }
            }

            MalType::Float(s) => {
                match rhs {
                    MalType::Int(i) => Ok(MalType::Float(OrderedFloat(i as f64) * s.into_inner())),
                    MalType::Float(f) => Ok(MalType::Float(f * s)),
                    _ => Err(MalError::InvalidToken),
                }
            },

            _ => Err(MalError::InvalidToken),
        }
    }

}

impl Sub for MalType {
    type Output = Result<MalType, MalError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            MalType::Int(s) => {
                match rhs {
                    MalType::Int(i) => Ok(MalType::Int(s - i)),
                    MalType::Float(f) => Ok(MalType::Float(OrderedFloat(s as f64 - f.into_inner()))),
                    _ => Err(MalError::InvalidToken),
                }
            }

            MalType::Float(s) => {
                match rhs {
                    MalType::Int(i) => Ok(MalType::Float(OrderedFloat(s.into_inner() - i as f64))),
                    MalType::Float(f) => Ok(MalType::Float(s + f)),
                    _ => Err(MalError::InvalidToken),
                }
            },

            _ => Err(MalError::InvalidToken),
        }
    }

}

impl Div for MalType {
    type Output = Result<MalType, MalError>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            MalType::Int(s) => {
                match rhs {
                    MalType::Int(i) => Ok(MalType::Int(s/i)),
                    MalType::Float(f) => Ok(MalType::Float(OrderedFloat(s as f64 / f.into_inner()))),
                    _ => Err(MalError::InvalidToken),
                }
            }

            MalType::Float(s) => {
                match rhs {
                    MalType::Int(i) => Ok(MalType::Float(OrderedFloat(s.into_inner() / i as f64))),
                    MalType::Float(f) => Ok(MalType::Float(s / f)),
                    _ => Err(MalError::InvalidToken),
                }
            },

            _ => Err(MalError::InvalidToken),
        }
    }

}
