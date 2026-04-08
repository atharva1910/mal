use crate::{
    types::{MalType, MalError},
    env::{MalEnv, self},
    printer::Printer,
};

pub struct Eval{}


impl Eval {
    fn execute_func<F>(func: F, args: &[MalType]) -> Result<MalType, MalError>
    where
        F: Fn(&[MalType]) -> Result<MalType, MalError>
    {
        func(args)
    }

    fn eval_list(input: MalType, env: &MalEnv) -> Result<MalType, MalError> {
        let MalType::List(mut lmt) = input else {
            panic!("Why are we here");
        };

        let Some(opr) = lmt.pop_front() else {
            return Ok(MalType::init_list());
        };

        let MalType::Sym(opr_type) = &opr else {
            return Err(MalError::InvalidToken);
        };


        match &opr_type[..] {
            "list?" => {
                let Some(MalType::List(_)) = lmt.pop_front() else {
                    return Ok(MalType::create_bool(false)?);
                };

                Ok(MalType::create_bool(true)?)
            }

            "do" => {
                let mut ret: MalType = MalType::init_list();
                while let Some(mt) = lmt.pop_front()  {
                    ret = Eval::eval(mt, env)?;
                }

                Ok(ret)
            }

            "let*" => {
                let new_env = env::create_child(env);

                let Some(var_def) = lmt.pop_front() else {
                    return Err(MalError::InvalidToken);
                };

                match var_def {
                    MalType::List(mut l) => {
                        while let Some(key) = l.pop_front() {
                            let Some(val) = l.pop_front() else {
                                return Err(MalError::EOF);
                            };

                            println!("let* Eval:");
                            Printer::print(val.clone());
                            let val = Eval::eval(val, &new_env)?;
                            env::set(&new_env, key, val);
                        }
                    }

                    MalType::Vec(v) => {
                        for c in v.chunks_exact(2) {
                            let key = c[0].clone();
                            let val = Eval::eval(c[1].clone(), &new_env)?;
                            env::set(&new_env, key, val);

                        }

                        if !v.chunks_exact(2).remainder().is_empty() {
                            return Err(MalError::EOF);
                        }
                    }

                    _ => panic!("What"),
                }

                let mut ret: Result<MalType, MalError> = Ok(MalType::init_list());
                while let Some(mt) = lmt.pop_front() {
                    ret = Eval::eval(mt, &new_env);
                }
                ret
            }

            "def!" => {
                let Some(key) = lmt.pop_front() else {
                    return Err(MalError::InvalidToken);
                };

                let Some(val) = lmt.pop_front() else {
                    return Err(MalError::InvalidToken);
                };

                let val = Eval::eval(val, env)?;
                env::set(env, key, val.clone());
                return Ok(val);

            }

            _ => {
                let Some(MalType::Func(func)) = env::get(env, opr.clone()) else {
                    println!("{} not found", opr);
                    return Err(MalError::InvalidToken);
                };

                let args: Result<Vec<MalType>, MalError> = lmt.into_iter().map(|mt| Eval::eval(mt, env)).collect();
                return Eval::execute_func(func, &args?);
            }
        }
    }

    fn eval_vec(input: MalType, env: &MalEnv) -> Result<MalType, MalError> {
        let MalType::Vec(lmt) = input else {
            panic!("Why are we here");
        };

        let mut ret = MalType::init_vec();
        for mt in lmt {
            ret.push(Eval::eval(mt, env)?);
        }
        Ok(ret)
    }

    fn eval_hash(input: MalType, env: &MalEnv) -> Result<MalType, MalError> {
        let MalType::Hash(hash) = input else {
            panic!("Why are we here");
        };

        let mut ret = MalType::init_dict();
        for (k, v) in hash.into_iter() {
            ret.insert(k, Eval::eval(v, env)?);
        }

        Ok(ret)
    }

    fn eval_sym(input: MalType, env: &MalEnv) -> Result<MalType, MalError> {
        if let Some(ret) = env::get(env, input) {
            return Ok(ret);
        }
        return Err(MalError::InvalidToken);
    }

    fn print_eval(input: &MalType, env: &MalEnv) -> Result<(), MalError> {
        let MalType::Bool(b) = MalType::to_bool(env::get(env, MalType::init_sym("DEBUG-EVAL")).as_ref())? else {
            panic!("HUH?");
        };

        if b {
            println!("EVAL: {}", Printer::print(input.clone()));
        }
    }

    pub fn eval(input: MalType, env: &MalEnv) -> Result<MalType, MalError> {
        match &input {
            MalType::List(_) => Eval::eval_list(input, env),
            MalType::Vec(_) => Eval::eval_vec(input, env),
            MalType::Hash(_) => Eval::eval_hash(input, env),
            MalType::Sym(_) => Eval::eval_sym(input, env),
            _ =>  Ok(input)
        }
    }

}
