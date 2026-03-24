mod reader;
mod printer;
mod types;
mod env;
mod ops;
use std::io::{self, Write};
use reader::Reader;
use crate::{
    types::{MalType, MalError},
    printer::Printer,
    env::{MalEnv, Env},
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

fn eval(input: MalType, env: &MalEnv) -> Result<MalType, MalError> {
    match input {
        MalType::List(mut lmt) => {
            let Some(opr) = lmt.pop_front() else {
                return Ok(MalType::init_list());
            };

            let MalType::Sym(opr_type) = &opr else {
                return Err(MalError::InvalidToken);
            };


            match &opr_type[..] {
                "let*" => {
                    let Some(key) = lmt.pop_front() else {
                        return Err(MalError::InvalidToken);
                    };

                    let Some(val) = lmt.pop_front() else {
                        return Err(MalError::InvalidToken);
                    };

                    let val = eval(val, env)?;
                    let new_env = env::create_child(env, key, val);
                }

                "def!" => {
                    let Some(key) = lmt.pop_front() else {
                        return Err(MalError::InvalidToken);
                    };

                    let Some(val) = lmt.pop_front() else {
                        return Err(MalError::InvalidToken);
                    };

                    let val = eval(val, env)?;
                    env::set(env, key, val);
                }

                _ => {
                    let Some(MalType::Func(func)) = env::get(env, opr) else {
                        return Err(MalError::InvalidToken);
                    };

                    let args = lmt.into_iter().map(|mt| eval(mt, env)).collect::<Result<Vec<MalType>, MalError>>();
                    return execute_func(func, &args?);
                }
            }

            Ok(MalType::init_list())
        }

        MalType::Vec(lmt) => {
            let mut ret = MalType::init_vec();
            for mt in lmt {
                ret.push(eval(mt, env)?);
            }
            Ok(ret)
        }

        MalType::Hash(hash) => {
            let mut ret = MalType::init_dict();
            for (k, v) in hash.into_iter() {
                ret.insert(eval(k, env)?,
                           eval(v, env)?);
            }
            Ok(ret)
        }

        _ =>  Ok(input)
    }
}

fn print(input: MalType) -> String {
    Printer::pr_str(input)
}

fn rep(input: String) -> Result<String, MalError> {
    let env = env::init();
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
