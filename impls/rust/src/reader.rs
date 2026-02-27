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
            return Err(MalError::EOF);
        }

        let ret = self.tokens[self.pos].clone();
        self.pos += 1;
        Ok(ret)
    }

    fn peek(&self) -> Result<String, MalError> {
        if self.pos >= self.tokens.len() {
            return Err(MalError::EOF);
        }
        Ok(self.tokens[self.pos].clone())
    }

    fn read_from(&mut self) -> Result<MalType, MalError> {
        match self.peek()?.as_str() {
            "(" => Ok(self.read_list("(", ")")?),
            "[" => Ok(self.read_list("[", "]")?),
            "{" => Ok(self.read_list("{", "}")?),
            _ => Ok(self.read_atom()?),

        }
    }

    fn read_list(&mut self, start: &str, end: &str) -> Result<MalType, MalError> {
        if self.peek()?.as_str() != start {
            return Err(MalError::ParsingError);
        } else {
            self.next()?;
        }

        let mut ret: MalType = MalType::init_list();
        loop {
            if self.peek()?.as_str() == end {
                self.next()?;
                break;
            } else {
                ret.push(self.read_from()?);
            }
        }

        Ok(ret)
    }

    fn read_atom(&mut self) -> Result<MalType, MalError> {
        Ok(MalType::init_atom(self.next()?))
    }


    fn tokenize(input: String) -> Vec<String> {
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap()
            .find_iter(&input)
            .map(|x| {
                x.as_str().trim().trim_matches(',').trim().to_string()
            }) .collect::<Vec<String>>()
    }
}
