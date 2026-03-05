use crate::types::MalType;

pub struct Printer {
    ret_string: String,
}

impl Printer{
    pub fn pr_str(input: MalType) -> String {
        let mut printer = Self {
            ret_string: String::new()
        };
        printer.gen_str(input);
        printer.ret_string
    }

    fn gen_str(&mut self, input: MalType) {
        match input {
            MalType::Atom(s) => {
                self.ret_string += &s.atom_to_string();
                self.ret_string += " ";
            }
            MalType::List(t) => {
                self.ret_string += "(";
                for mt in t {
                    self.gen_str(mt);
                }
                self.ret_string =
                    self.ret_string.trim().to_string() + ") ";
            }

            MalType::Vec(t) => {
                self.ret_string += "[";
                for mt in t {
                    self.gen_str(mt);
                }
                self.ret_string =
                    self.ret_string.trim().to_string() + "] ";
            }

            MalType::Hash(t) => {
                self.ret_string += "{";
                for (s, mt) in t.into_iter() {
                    self.gen_str(s);
                    self.gen_str(mt);
                }
                self.ret_string =
                    self.ret_string.trim().to_string() + "} ";
            }
        }
    }
}
