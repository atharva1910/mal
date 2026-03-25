use crate::types::MalType;

pub struct Printer {
}

impl Printer{
    pub fn print(input: MalType) -> String {
        Printer::pr_str(input)
    }

    fn pr_str(input: MalType) -> String {
        let mut ret = String::new();
        Printer::gen_str(input, &mut ret);
        ret
    }

    fn gen_str(input: MalType, ret: &mut String) {
        match input {
            MalType::Int(i) => ret.push_str(&i.to_string()),

            MalType::Float(f) => ret.push_str(&f.to_string()),

            MalType::Str(s) | MalType::Sym(s) => ret.push_str(&s),

            MalType::List(t) => {
                ret.push_str("(");

                for mt in t {
                    Printer::gen_str(mt, ret);
                }

                *ret =
                    ret.trim().to_string() + ")";
            }

            MalType::Vec(t) => {
                ret.push_str("[");
                for mt in t {
                    Printer::gen_str(mt, ret);
                }
                *ret =
                    ret.trim().to_string() + "]";
            }

            MalType::Hash(t) => {
                ret.push_str("{");
                for (s, mt) in t.into_iter() {
                    Printer::gen_str(s, ret);
                    Printer::gen_str(mt, ret);
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
