// Parser semi-types
use super::{Debug, DefaultTypes, Env};
use crate::structs::bridge::StatementImpl;

#[derive(Debug, Clone)]
pub struct Statement {
    imp: StatementImpl,
}

impl Statement {
    pub fn new(raw: Vec<String>, line: u32) -> Self {
        Statement {
            imp: StatementImpl::new(raw, line),
        }
    }
    pub fn with_imp(imp: StatementImpl) -> Self {
        Statement {
            imp,
        }
    }
    pub fn with_setter(mut self, setter: DefaultTypes) -> Self {
        self.imp.with_setter(setter);
        self
    }
    pub fn raw(&self) -> &Vec<String> {
        self.imp.raw()
    }
    pub fn mut_raw(&mut self) -> &mut Vec<String> {
        self.imp.mut_raw()
    }
    pub fn is_finished(&self) -> bool {
        self.imp.is_finished()
    }
    pub fn is_function_end(&self) -> bool {
        self.imp.is_function_end()
    }
    pub fn is_function_decl(&self) -> bool {
        self.imp.is_function_decl()
    }
    pub fn is_in_scope(&self) -> bool {
        self.imp.is_in_scope()
    }
    pub fn add_to_scope(&mut self) {
        self.imp.add_to_scope()
    }
    pub fn is_scope_end(&self) -> bool {
        self.imp.is_scope_end()
    }
    pub fn raw_get(&self, i: usize) -> String {
        self.imp.raw_get(i)
    }
    pub fn line_as_string(&self) -> String {
        self.imp.line_as_string()
    }
    pub fn line(&self) -> u32 {
        self.imp.line()
    }
    pub fn first(&self) -> String {
        self.imp.first()
    }
    pub fn last(&self) -> String {
        self.imp.last()
    }
    pub fn is_raw_function_call(&self) -> bool {
        self.imp.is_raw_function_call()
    }
    pub fn get_function_call_args_indexed(
        &self,
        env: &mut Env,
        s: &str,
    ) -> Result<Vec<DefaultTypes>, String> {
        self.imp.get_function_call_args_indexed(env, s)
    }
    pub fn as_func(&self) -> Box<dyn Fn(&mut Env)> {
        self.imp.as_func()
    }
}
