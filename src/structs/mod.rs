mod bridge;
mod env;
mod functions;
mod statement;
mod table;

pub use env::Env;
pub use functions::Function;
pub use bridge::DynFunc;
pub use statement::Statement;
use std::fmt::Debug;
pub use table::Table;

pub type Statements = Vec<Statement>;

// Default lang types
#[derive(Clone, Debug)]
pub enum DefaultTypes {
    Str(String),
    Int(i32),
    Bool(bool),
    Table(Table),
    Function(Function),
}

impl From<Table> for DefaultTypes {
    fn from(s: Table) -> Self {
        DefaultTypes::Table(s)
    }
}
