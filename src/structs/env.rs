// Interpreter Env
use super::{DefaultTypes, Variables};
use crate::structs::Variable;

#[derive(Clone)]
pub struct Env {
    pub vars: Variables,
    names: Vec<String>,
    return_value: Option<Vec<DefaultTypes>>,
    is_exit: bool,
}

impl Env {
    pub fn new() -> Self {
        Env {
            vars: Vec::new(),
            names: Vec::new(),
            return_value: None,
            is_exit: false,
        }
    }
    pub fn contains(&self, v: &str) -> bool {
        self.names.contains(&String::from(v))
    }
    pub fn add_variable(&mut self, name: &str, value: DefaultTypes) {
        self.vars.push(Variable::new(String::from(name), value));
        self.names.push(String::from(name));
    }
    pub fn remove(&mut self, name: &str) {
        let mut i = 0;
        let check = |s| s == name;
        while i != self.names.len() {
            let r = self.names[i].clone();
            if check(r) {
                self.names.remove(i);
            } else {
                i += 1;
            }
        }
    }
    pub fn return_f(&mut self, v: Vec<DefaultTypes>) {
        self.return_value = Some(v);
    }
    pub fn set_variable(&mut self, name: &str, value: DefaultTypes) {
        self.add_variable(&*name, value);
    }
    pub fn get(&self, name: &str) -> Option<DefaultTypes> {
        let mut found = None;
        for x in self.vars.clone() {
            if x.name() == name {
                found = Some(x.value);
            }
        }
        found
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

    pub fn var(&self) -> &Variables {
        &self.vars
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::new()
    }
}
