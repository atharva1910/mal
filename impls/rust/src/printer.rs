use crate::types::MalType;

pub struct Printer {
    concat_string: String,
}

impl Printer{
    pub fn pr_str(input: MalType) -> String {
        let mut printer = Self {
            concat_string: String::new()
        };
        return printer.gen_str1(input);
        //printer.gen_str(input);
        //printer.concat_string
    }

    fn gen_str(&mut self, input: MalType) {
        match input {
            MalType::Atom(s) => {
                self.concat_string += s.as_str();
                self.concat_string += " ";
            }
            MalType::List(t) => {
                self.concat_string += "(";
                for mt in t {
                    self.gen_str(mt);
                }
                self.concat_string =
                    self.concat_string.trim().to_string() + ")";
            }
        }
    }

    fn gen_str1(&mut self, input: MalType) -> String{
        match input {
            MalType::Atom(s) => {
                return s + " ";
            }
            MalType::List(t) => {
                let mut ret: String = String::new();
                ret += "(";
                for mt in t {
                    ret += &self.gen_str1(mt);
                }
                return ret.trim().to_string() + ")";
            }
        }
    }
}
