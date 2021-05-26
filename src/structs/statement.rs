// Parser semi-types
use super::{Debug, DefaultTypes, Env};
use crate::parse_exp;

#[derive(Debug, Clone)]
pub struct Statement {
    pub raw: Vec<String>,
    pub(super) data: Option<String>,
    in_scope: bool,
    line: u32,
}

impl Statement {
    pub fn new(raw: Vec<String>, line: u32) -> Self {
        Statement {
            raw,
            data: None,
            in_scope: false,
            line,
        }
    }
    pub fn raw(&self) -> &Vec<String> {
        &self.raw
    }
    pub fn mut_raw(&mut self) -> &mut Vec<String> {
        &mut self.raw
    }
    pub fn is_finished(&self) -> bool {
        self.raw.last().unwrap_or(&String::from("")).ends_with(';') // TODO: HANDLE ERROR
    }
    pub fn is_function_end(&self) -> bool {
        if self.raw.len() < 2 {
            false
        } else {
            self.last() == *"}"
        }
    }
    pub fn is_function_decl(&self) -> bool {
        if self.raw.len() < 3 {
            false
        } else {
            (self.raw_get(1) == "->" || self.raw_get(1) == "=") && (self.raw_get(2) == "{")
        }
    }
    pub fn is_in_scope(&self) -> bool {
        self.in_scope
    }
    pub fn add_to_scope(&mut self) {
        self.in_scope = true;
    }
    pub fn is_scope_end(&self) -> bool {
        self.raw
            .get(self.raw.len() - 2)
            .unwrap_or(&String::from(""))
            .as_str()
            == "}"
    }
    pub fn raw_get(&self, i: usize) -> String {
        String::from(self.raw.get(i).unwrap_or(&String::from("")))
    }
    pub fn line_as_string(&self) -> String {
        self.line.to_string()
    }
    pub fn line(&self) -> u32 {
        self.line
    }
    pub fn first(&self) -> String {
        self.raw_get(0).replace("(", "")
    }
    pub fn last(&self) -> String {
        self.raw_get(self.raw.len() - 2)
    }
    pub fn is_raw_function_call(&self) -> bool {
        self.raw_get(0).ends_with('(') && self.raw_get(self.raw.len() - 2) == ")"
    }
    pub fn get_function_call_args_indexed(
        &self,
        env: &mut Env,
        s: &str,
    ) -> Result<Vec<DefaultTypes>, String> {
        let mut started = false;
        let mut dat = vec![];
        let mut in_nest = 0_isize;
        for raw in &self.raw().clone() {
            if started {
                if raw.ends_with('(') {
                    if in_nest == 0 {
                        dat.push(parse_exp(raw, env, self)?);
                    }
                    in_nest += 1;
                } else if raw.ends_with(')') {
                    in_nest -= 1;
                    if in_nest < 0 {
                        break;
                    }
                } else if in_nest == 0 {
                    dat.push(parse_exp(raw, env, self)?);
                }
            }
            if raw.starts_with(&s) {
                started = true;
            }
        }
        Ok(dat)
    }
    pub fn as_func(&self) -> Box<dyn Fn(&mut Env)> {
        let s = self.clone();
        match s.raw_get(1).as_str() {
            "->" | "=" => {
                let name = s.raw_get(0); // Necessarily exists since index 1 exists and whitespace characters were removed.
                Box::new(move |e2| {
                    let val = parse_exp(&s.raw_get(2), e2, &s.clone());
                    if let Ok(v) = val {
                        e2.set_variable(&name, v);
                    } else if let Err(err_msg) = val {
                        println!("{} - Line {}", err_msg, s.line());
                        e2.exit();
                    }
                })
            }
            _ => Box::new(move |mut e2| {
                let v = e2.get(&s.first());
                if let Some(DefaultTypes::Function(f)) = v {
                    let t2 = s.get_function_call_args_indexed(e2, &s.first());
                    match t2 {
                        Ok(call_args) => {
                            let _s = f.call(&mut e2, call_args);
                        }
                        Err(err_msg) => {
                            println!("{} - Line {}", err_msg, s.line());
                            e2.exit();
                        }
                    }
                }
            }),
        }
    }
}
