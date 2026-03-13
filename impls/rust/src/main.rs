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

fn execute_func<F>(func: F, args: &[MalType]) -> Result<MalType, MalError>
where
     F: Fn(&[MalType]) -> Result<MalType, MalError>
{
    func(args)
}

fn read(input: String) -> Result<MalType, MalError> {
    Reader::read_str(input)
}

fn eval(input: MalType, env: &Env) -> Result<MalType, MalError> {
    if let MalType::List(mut lmt) = input {
        let func = env.lookup(lmt.pop_front())?;
        let args = lmt.into_iter().map(|mt| eval(mt, env)).collect::<Result<Vec<MalType>, MalError>>();
        return execute_func(func, &args?);
    }
    Ok(input)
}

fn print(input: MalType) -> String {
    Printer::pr_str(input)
}

fn rep(input: String) -> Result<String, MalError> {
    let env = Env::init();
    let read_ret = read(input)?;
    let eval_ret = eval(read_ret, &env)?;
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
