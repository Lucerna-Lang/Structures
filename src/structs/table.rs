use crate::structs::DefaultTypes;

use std::fmt::Debug;
use std::slice::Iter;

#[derive(Clone, Debug)]
pub struct Pairs {
    key: DefaultTypes,
    value: DefaultTypes,
}

#[derive(Clone, Debug)]
pub struct Table {
    data: Vec<Pairs>,
}

impl Pairs {
    pub fn key(&self) -> &DefaultTypes {
        &self.key
    }

    pub fn value(&self) -> &DefaultTypes {
        &self.value
    }
}

impl Table {
    pub fn new() -> Table {
        Table { data: vec![] }
    }
    pub fn iter_data(&self) -> Iter<Pairs> {
        self.data.iter()
    }

    pub fn raw_get(&self, s2: &str) -> Option<DefaultTypes> {
        let mut r = None;
        for x in &self.data {
            if let DefaultTypes::Str(s1) = x.key.clone() {
                if s1 == s2 {
                    r = Some(x.value.clone());
                }
            }
        }
        r
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn set(&mut self, s2: String, v: DefaultTypes) {
        for (i, x) in self.data.clone().iter().enumerate() {
            if let DefaultTypes::Str(s1) = x.key.clone() {
                if s1 == s2 {
                    self.data.remove(i);
                }
            }
        }
        self.data.push(Pairs {
            key: DefaultTypes::Str(s2),
            value: v,
        });
    }
}

impl Default for Table {
    fn default() -> Self {
        Table::new()
    }
}
