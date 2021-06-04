use crate::structs::DefaultTypes;

use std::fmt::Debug;
use std::collections::hash_map::Iter;

use super::bridge::TableImpl;

#[derive(Clone, Debug)]
pub struct Table {
    imp: TableImpl,
}

impl Table {
    pub fn new() -> Table {
        Table {
            imp: TableImpl::new(),
        }
    }
    pub fn iter_data(&self) -> Iter<String, DefaultTypes> {
        self.imp.iter_data()
    }

    pub fn raw_get(&self, s2: &str) -> Option<DefaultTypes> {
        self.imp.get(s2)
    }

    pub fn len(&self) -> usize {
        self.imp.len()
    }

    pub fn is_empty(&self) -> bool {
        self.imp.is_empty()
    }

    pub fn set(&mut self, k: String, v: DefaultTypes) {
        self.imp.set(k, v);
    }
}

impl Default for Table {
    fn default() -> Self {
        Table::new()
    }
}
