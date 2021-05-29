use crate::structs::{DefaultTypes, DefaultTypes::Function, Env, Statement};

/// # Errors
/// Will error if the string provided is none or if it is not transformable into a default type
pub fn parse_exp(ss: &str, env: &mut Env, sss: &Statement) -> Result<DefaultTypes, String> {
    let k;
    let t = String::from(ss);
    if ss.starts_with('"') && ss.ends_with('"') {
        k = Ok(String::from(ss));
    } else {
        let temp = ss.split('(').next();
        match temp {
            None => {
                k = Err("Attempting to parse nothing".to_string());
            }
            Some(val) => {
                k = Ok(val.parse::<String>().unwrap_or_else(|_| "".to_string()));
            }
        }
    }
    let s = k?;
    if s.parse::<i32>().is_ok() {
        Ok(DefaultTypes::Int(
            s.parse::<i32>().expect("Something odd happened here"),
        ))
    } else if s.starts_with('"') && s.ends_with('"') {
        Ok(DefaultTypes::Str(
            String::from(&s[1..s.len() - 1])
                .replace("|_", " ")
                .replace("|-", "\n")
                .replace(r"\/", r"/"),
        ))
    } else if s == "true" {
        Ok(DefaultTypes::Bool(true))
    } else if s == "false" {
        Ok(DefaultTypes::Bool(false))
    } else if env.contains(&s) {
        let dt = env.get(&s).expect("Something really odd happened"); // This should logically never panic since env.contains was true
        match dt {
            DefaultTypes::Function(f) => {
                if ss.ends_with('(') {
                    let args = get_args(&t, env, sss);
                    Ok(f.call(env, args?)[0].clone())
                } else {
                    Ok(Function(f))
                }
            }
            DefaultTypes::Table(tab) => Ok(DefaultTypes::Table(tab)),
            _ => Ok(dt),
        }
    } else {
        let err = format!("{}\"{}\"", "Could not parse expression: ", &t);
        Err(err)
    }
}

/// # Errors
pub fn get_args(s: &str, env: &mut Env, ss: &Statement) -> Result<Vec<DefaultTypes>, String> {
    ss.get_function_call_args_indexed(env, s)
}
