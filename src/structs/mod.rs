mod env;
mod functions;
mod statement;
mod table;

pub use env::Env;
pub use functions::{DynFunc, Function};
pub use statement::Statement;
use std::fmt::Debug;
pub use table::Table;

pub type Statements = Vec<Statement>;
pub type Variables = Vec<Variable>;

// Default lang types
#[derive(Clone, Debug)]
pub enum DefaultTypes {
    Str(String),
    Int(i32),
    Bool(bool),
    Table(Table),
    Function(Function),
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub value: DefaultTypes,
}

impl Variable {
    pub fn new(name: String, value: DefaultTypes) -> Self {
        Variable { name, value }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn value(&self) -> &DefaultTypes {
        &self.value
    }
    pub fn set_value(&mut self, v: DefaultTypes) {
        self.value = v;
    }
}

impl From<Table> for DefaultTypes {
    fn from(s: Table) -> Self {
        DefaultTypes::Table(s)
    }
}
