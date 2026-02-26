mod reader;
mod printer;
mod types;
use std::io::{self, Write};
use reader::Reader;
use crate::{
    types::{MalType, MalError},
    printer::Printer,
};

fn READ(input: String) -> Result<MalType, MalError> {
    Reader::read_str(input)
}

fn EVAL(input: MalType) -> MalType {
    input
}

fn PRINT(input: MalType) -> String {
    Printer::pr_str(input)
}

fn rep(input: String) -> Result<String, MalError> {
    let read_ret = READ(input)?;
    let eval_ret = EVAL(read_ret);
    let print_ret = PRINT(eval_ret);
    Ok(print_ret)
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
