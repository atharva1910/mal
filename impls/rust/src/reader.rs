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
            "(" => {
                let mut ret = MalType::init_list();
                self.read_list_type("(", ")", &mut ret)?;
                Ok(ret)
            },

            "[" => {
                let mut ret = MalType::init_vec();
                self.read_list_type("[", "]", &mut ret)?;
                Ok(ret)
            },

            "{" => {
                let mut ret = MalType::init_dict();
                //self.read_list_type("{", "}", &mut ret)?;
                self.read_dict(&mut ret)?;
                Ok(ret)
            },

            "'" | "`" | "~" | "~@" => {
                let mut ret: MalType = MalType::init_list();
                ret.push(MalType::init_atom(self.next()?)?);
                ret.push(self.read_from()?);
                Ok(ret)
            }

            _ => Ok(self.read_atom()?),
        }
    }

    fn read_dict(&mut self, list_type: &mut MalType) -> Result<(), MalError> {
        if self.next()?.as_str() != "{" {
            return Err(MalError::ParsingError);
        }

        loop {
            if self.peek()?.as_str() == "}" {
                self.next()?;
                break;
            } else {
                let key = self.read_from()?;
                let value = self.read_from()?;
                list_type.insert(key, value);
            }
        }

        Ok(())
    }

    fn read_list_type(&mut self, start: &str, end: &str, list_type: &mut MalType) -> Result<(), MalError> {
        if self.next()?.as_str() != start {
            return Err(MalError::ParsingError);
        }

        loop {
            if self.peek()?.as_str() == end {
                self.next()?;
                break;
            } else {
                list_type.push(self.read_from()?);
            }
        }

        Ok(())
    }

    fn read_atom(&mut self) -> Result<MalType, MalError> {
        Ok(MalType::init_atom(self.next()?)?)
    }


    fn tokenize(input: String) -> Vec<String> {
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap()
            .find_iter(&input)
            .map(|x| {
                x.as_str().trim().trim_matches(',').trim().to_string()
            }) .collect::<Vec<String>>()
    }
}
