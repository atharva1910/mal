mod reader;
mod printer;
mod types;
mod env;
use std::io::{self, Write};
use reader::Reader;
use crate::{
    types::{MalType, MalError},
    printer::Printer,
    env::Env,
};

fn execute_func<F>(func: F)
where
     F: Fn()
{
    func()
}

fn read(input: String) -> Result<MalType, MalError> {
    Reader::read_str(input)
}

fn eval(input: MalType) -> Result<MalType, MalError> {
    match input {
        MalType::Atom(mat) => {
            match mat {
                types::MalAtomType::Sym(s) => {
                    execute_func(Env::map(&s)?);
                    todo!("call the function");
                },
                _ => panic!("atom not handled"),
            }
        },

        MalType::List(lmt) => {
            panic!("list not handled");
        },

        MalType::Vec(vmt) => {
            panic!("vec not handled");
        },

        MalType::Hash(hmt) => {
            panic!("hash not handled");
        },
    }
}

fn print(input: MalType) -> String {
    Printer::pr_str(input)
}

fn rep(input: String) -> Result<String, MalError> {
    let read_ret = read(input)?;
    let eval_ret = eval(read_ret)?;
    Ok(print(eval_ret))
}

fn main() -> io::Result<()> {
    loop {
        print!("user> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match rep(input.trim().to_string()) {
            Ok(s) => println!("{s}"),
            Err(e) => println!("{}", e),
        }
    }
}
