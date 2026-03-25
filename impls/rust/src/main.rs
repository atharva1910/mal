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
    match input.clone() {
        MalType::List(mut lmt) => {
            let Some(opr) = lmt.pop_front() else {
                return Ok(MalType::init_list());
            };

            let MalType::Sym(opr_type) = &opr else {
                return Err(MalError::InvalidToken);
            };


            match &opr_type[..] {
                "let*" => {
                    let new_env = env::create_child(env);

                    let Some(MalType::List(mut var_list)) = lmt.pop_front() else {
                        return Err(MalError::InvalidToken);
                    };


                    while let Some(var_def) = var_list.pop_front() {
                        match var_def {
                            MalType::List(mut l) => {
                                let Some(key) = l.pop_front() else {
                                    return Err(MalError::InvalidToken);
                                };

                                let Some(val) = l.pop_front() else {
                                    return Err(MalError::InvalidToken);
                                };

                                let val = eval(val, &new_env)?;
                                env::set(&new_env, key, val);
                            }

                            _ => return Err(MalError::InvalidToken),
                        }
                    };

                    return eval(input, &new_env);
                }

                "def!" => {
                    let Some(key) = lmt.pop_front() else {
                        return Err(MalError::InvalidToken);
                    };

                    let Some(val) = lmt.pop_front() else {
                        return Err(MalError::InvalidToken);
                    };

                    let val = eval(val, env)?;
                    println!("setting key: {:?} val {:?}", key, val);
                    env::set(env, key, val.clone());
                    return Ok(val);

                }

                _ => {
                    let Some(MalType::Func(func)) = env::get(env, opr) else {
                        return Err(MalError::InvalidToken);
                    };

                    let args = lmt.into_iter().map(|mt| eval(mt, env)).collect::<Result<Vec<MalType>, MalError>>();
                    return execute_func(func, &args?);
                }
            }
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

        MalType::Sym(s) => {
            println!("Get input {:?}", input.clone());
            if let Some(ret) = env::get(env, input.clone()) {
                return Ok(ret);
            }
            return Err(MalError::InvalidToken);
        }

        _ =>  Ok(input)
    }
}

fn print(input: MalType) -> String {
    Printer::pr_str(input)
}

fn rep(input: String, env: &MalEnv) -> Result<String, MalError> {
    let read_ret = read(input)?;
    let eval_ret = eval(read_ret, env)?;
    Ok(print(eval_ret))
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
