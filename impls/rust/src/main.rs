mod reader;
mod printer;
mod types;
mod env;
mod ops;
mod eval;
use std::io::{self, Write};
use crate::{
    types:: MalError,
    printer::Printer,
    env::{MalEnv},
    eval::Eval,
    reader::Reader,
};



fn rep(input: String, env: &MalEnv) -> Result<String, MalError> {
    let read_ret = Reader::read(input)?;
    let eval_ret = Eval::eval(read_ret, env)?;
    Ok(Printer::print(eval_ret))
}

fn main() -> io::Result<()> {
    let env = env::init();
    loop {
        print!("user> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match rep(input.trim().to_string(), &env) {
            Ok(s) => println!("{s}"),
            Err(e) => println!("{}", e),
        }
    }
}
