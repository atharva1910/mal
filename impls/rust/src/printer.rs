use crate::types::MalType;

pub struct Printer {
    ret_string: String,
}

impl Printer{
    pub fn pr_str(input: MalType) -> String {
        let mut printer = Self {
            ret_string: String::new()
        };

        let mut ret = String::new();
        printer.gen_str(input, &mut ret);
        ret
        //printer.gen_str(input);
            //printer.ret_string
    }

    fn gen_str(&mut self, input: MalType, ret: &mut String) {
        match input {
            MalType::Int(i) => ret.push_str(&i.to_string()),

            MalType::Float(f) => ret.push_str(&f.to_string()),

            MalType::Str(s) | MalType::Sym(s) => ret.push_str(&s),

            MalType::List(t) => {
                ret.push_str("(");

                for mt in t {
                    self.gen_str(mt, ret);
                }

                *ret =
                    ret.trim().to_string() + ")";
            }

            MalType::Vec(t) => {
                ret.push_str("[");
                for mt in t {
                    self.gen_str(mt, ret);
                }
                *ret =
                    ret.trim().to_string() + "]";
            }

            MalType::Hash(t) => {
                ret.push_str("{");
                for (s, mt) in t.into_iter() {
                    self.gen_str(s, ret);
                    self.gen_str(mt, ret);
                }
                *ret =
                    ret.trim().to_string() + "}";
            }

            MalType::Func(_) => {
                ret.push_str("Func");
            }
        }

        ret.push_str(" ");
    }
}
