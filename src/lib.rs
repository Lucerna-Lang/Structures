#![feature(allocator_api)]

pub mod structs;
mod tokenizer;

pub use structs::DynFunc;
pub use tokenizer::{ParsedResult, parse_exp};
