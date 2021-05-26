use super::Statements;
use crate::structs::{DefaultTypes, Env, Statement, Table};
use core::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

// Special lang types
pub type DynFunc = dyn Fn(&mut Env, Vec<DefaultTypes>) -> Vec<DefaultTypes>;

pub struct Function {
    data: Statements,
    func: Option<Rc<DynFunc>>,
    name: Option<String>,
}

impl Function {
    pub fn new(func: Rc<DynFunc>) -> Self {
        Function {
            data: vec![],
            func: Some(func),
            name: None,
        }
    }
    pub fn call(&self, env: &mut Env, vs: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
        let k = &self.func.as_ref().unwrap();
        (*k(env, vs)).to_vec()
    }
    pub fn from_raw(data: Statements) -> Self {
        Function {
            data,
            func: None,
            name: None,
        }
    }
    pub fn push_raw(&mut self, data: Statement) {
        self.data.push(data);
    }
    pub fn set_name(&mut self, s: String) {
        self.name = Some(s);
    }
    pub fn data(&self) -> &Statements {
        &self.data
    }
    pub fn name(&self) -> &String {
        let x = &self.name;
        &x.as_ref().unwrap()
    }
    pub fn parse_func(&mut self) {
        let mut k: Vec<Box<dyn Fn(&mut Env)>> = Vec::new();
        for sr in &self.data {
            k.push(sr.clone().as_func());
        }
        self.func = Some(Rc::new(
            move |e: &mut Env, v: Vec<DefaultTypes>| -> Vec<DefaultTypes> {
                let mut e3 = e;
                let mut args = Table::new();
                for (i, x) in v.iter().enumerate() {
                    if e3.exited() {
                        break;
                    }
                    args.set(i.to_string(), x.clone());
                }
                e3.set_variable("args", args.into());
                for s in &k {
                    if e3.exited() {
                        break;
                    }
                    s(&mut e3);
                }

                e3.return_val()
            },
        ));
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            data: self.data.clone(),
            func: self.func.clone(),
            name: None,
        }
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("data", &self.data)
            .finish()
    }
}
