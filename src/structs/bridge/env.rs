use crate::structs::{DefaultTypes};
use std::collections::HashMap;

#[derive(Clone)]
pub struct EnvImpl {
    vars: HashMap<String, DefaultTypes>,
    return_value: Option<Vec<DefaultTypes>>,
    is_exit: bool,
}

impl EnvImpl {
    pub fn new() -> Self {
        EnvImpl {
            vars: HashMap::new(),
            return_value: None,
            is_exit: false,
        }
    }
    pub fn contains(&self, v: &str) -> bool {
        self.vars.contains_key(&String::from(v))
    }
    pub fn add_variable(&mut self, name: &str, value: DefaultTypes) {
        self.vars.insert(name.into(), value);
    }
    pub fn remove(&mut self, name: &str) {
        self.vars.remove(name);
    }
    pub fn return_f(&mut self, v: Vec<DefaultTypes>) {
        self.return_value = Some(v);
    }
    pub fn get(&self, name: &str) -> Option<DefaultTypes> {
        self.vars.get(name).cloned()
    }
    pub fn exited(&self) -> bool {
        self.is_exit
    }
    pub fn exit(&mut self) {
        self.is_exit = true;
    }
    pub fn return_val(&self) -> Vec<DefaultTypes> {
        self.return_value.clone().unwrap_or_default()
    }
}
