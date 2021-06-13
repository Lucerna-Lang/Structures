// Interpreter Env
use super::{DefaultTypes};

use super::bridge::EnvImpl;

#[derive(Clone)]
pub struct Env {
    imp: EnvImpl,
}

impl Env {
    pub fn new() -> Self {
        Env {
            imp: EnvImpl::new(),
        }
    }
    pub fn contains(&self, v: &str) -> bool {
        self.imp.contains(v)
    }
    pub fn add_variable(&mut self, name: &str, value: DefaultTypes) {
        self.imp.add_variable(name, value)
    }
    pub fn remove(&mut self, name: &str) {
        self.imp.remove(name)
    }
    pub fn return_f(&mut self, v: Vec<DefaultTypes>) {
        self.imp.return_f(v)
    }
    pub fn set_variable(&mut self, name: &str, value: DefaultTypes) {
        self.imp.add_variable(name, value)
    }
    pub fn get(&self, name: &str) -> Option<DefaultTypes> {
        self.imp.get(name)
    }
    pub fn exited(&self) -> bool {
        self.imp.exited()
    }
    pub fn exit(&mut self, error_message: &str, line: u32) -> ! {
        self.imp.exit(format!("{} - Line {}", error_message, line))
    }
    pub fn cline(&self) -> u32 {self.imp.get_current_line()}
    pub fn return_val(&self) -> Vec<DefaultTypes> {
        self.imp.return_val()
    }
    pub fn setline(&mut self, ln: u32) {
        self.imp.setline(ln);
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::new()
    }
}
