use crate::{
    types::{MalType, MalError},
    env::{MalEnv, self},
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
            "let*" => {
                let new_env = env::create_child(env);

                // Process the variable list
                let Some(var_def) = lmt.pop_front() else {
                    return Err(MalError::InvalidToken);
                };

                match var_def {
                    MalType::List(mut l) => {
                        while let Some(key) = l.pop_front() {
                            let Some(val) = l.pop_front() else {
                                return Err(MalError::EOF);
                            };

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

                let args = lmt.into_iter().map(|mt| Eval::eval(mt, env)).collect::<Result<Vec<MalType>, MalError>>();
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


    pub fn eval(input: MalType, env: &MalEnv) -> Result<MalType, MalError> {
        match input.clone() {
            MalType::List(_) => Eval::eval_list(input.clone(), env),
            MalType::Vec(_) => Eval::eval_vec(input.clone(), env),
            MalType::Hash(_) => Eval::eval_hash(input.clone(), env),
            MalType::Sym(_) => {
                if let Some(ret) = env::get(env, input.clone()) {
                    return Ok(ret);
                }
                println!("{} not found", input);
                return Err(MalError::InvalidToken);
            }

            _ =>  Ok(input)
        }
    }

}
