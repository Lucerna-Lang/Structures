use super::Statements;
use crate::structs::{DefaultTypes, Env, Statement};
use std::fmt::{Debug};
use std::rc::Rc;
use super::bridge::FunctionImpl;
use crate::structs::bridge::DynFunc;

#[derive(Debug, Clone)]
pub struct Function {
    imp: FunctionImpl,
}

impl Function {
    pub fn new(func: Rc<DynFunc>) -> Self {
        Function {
            imp: FunctionImpl::new(func),
        }
    }
    pub fn call(&self, env: &mut Env, vs: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
        self.imp.call(env, vs)
    }
    pub fn from_raw(data: Statements) -> Self {
        Function {
            imp: FunctionImpl::from_raw(data),
        }
    }
    pub fn push_raw(&mut self, data: Statement) {
        self.imp.push_raw(data)
    }
    pub fn set_name(&mut self, s: String) {
        self.imp.set_name(s)
    }
    pub fn data(&self) -> &Statements {
        self.imp.data()
    }
    pub fn name(&self) -> &String {
        self.imp.name()
    }
    pub fn parse_func(&mut self) {
        self.imp.parse_func()
    }
}
