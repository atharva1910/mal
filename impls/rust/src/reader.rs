use regex::Regex;
use crate::types::{MalType, MalError};

pub struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    pub fn read_str(input: String) -> Result<MalType, MalError> {
        let mut reader = Self {
            tokens: Reader::tokenize(input),
            pos: 0
        };
        reader.read_from()
    }
}

impl Reader {
    fn next(&mut self) -> Result<String, MalError> {
        if self.pos >= self.tokens.len() {
            return Err(MalError::ParsingError);
        }

        let ret = self.tokens[self.pos].clone();
        self.pos += 1;
        Ok(ret)
    }

    fn peek(&self) -> Result<String, MalError> {
        if self.pos >= self.tokens.len() {
            return Err(MalError::ParsingError);
        }
        Ok(self.tokens[self.pos].clone())
    }

    fn read_from(&mut self) -> Result<MalType, MalError> {
        match self.peek()?.as_str() {
            "(" => return Ok(self.read_list()?),
            _ => return Ok(self.read_atom()?),

        }
    }

    fn read_list(&mut self) -> Result<MalType, MalError> {
        if self.peek()?.as_str() != "(" {
            return Err(MalError::ParsingError);
        } else {
            self.next()?;
        }

        let mut ret: MalType = MalType::init_list();
        loop {
            if self.peek()?.as_str() == ")" {
                self.next()?;
                break;
            } else {
                ret.push(self.read_from()?);
            }
        }

        Ok(ret)
    }

    fn is_valid_atom(&self) -> Result<bool, MalError> {
        let atom = self.peek()?;

        if (atom.len() == 1 && ("+-/*").contains(&atom)) ||
            atom.parse::<f64>().is_ok() ||
            atom.parse::<u64>().is_ok() {
            return Ok(true);
        }

        Ok(false)
    }

    fn read_atom(&mut self) -> Result<MalType, MalError> {
        if self.is_valid_atom()? {
            Ok(MalType::init_atom(self.next()?))
        } else {
            Err(MalError::InvalidToken)
        }
    }


    fn tokenize(input: String) -> Vec<String> {
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap()
            .find_iter(&input)
            .map(|x| x.as_str().trim().to_string())
            .collect::<Vec<String>>()
    }
}
