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
    imp : TableImpl,
}

#[derive(Clone, Debug)]
struct TableImpl {
    data: Vec<Pairs>,
}

impl TableImpl {
    fn new() -> TableImpl {
        TableImpl { data: Vec::new() }
    }
    fn get(&self, s2: &str) -> Option<DefaultTypes> {
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
    fn set(&mut self, k: String, v: DefaultTypes) {
        self.remove(DefaultTypes::Str(k.clone()));
        self.data.push(Pairs {
            key: DefaultTypes::Str(k),
            value: v,
        });
    }
    fn raw_push(&mut self, k: DefaultTypes, v: DefaultTypes) {
        self.data.push(
            Pairs {
                key: k,
                value: v,
            }
        )
    }
    fn remove(&mut self, value: DefaultTypes) -> Result<&'static str, &'static str> {
        let mut removed = false;
        for (i, x) in self.data.clone().iter().enumerate() {
            match (&value, x.key()) {
                (DefaultTypes::Str(s), DefaultTypes::Str(other_s)) => {
                    if s == other_s {
                        self.data.remove(i);
                        removed = true;
                        break;
                    }
                },
                _ => {}
            }
        }
        if removed {
            Ok("")
        } else {
            Err("")
        }
    }
        
    fn iter_data(&self) -> Iter<Pairs> {
        self.data.iter()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
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
        Table { imp: TableImpl::new() }
    }
    pub fn iter_data(&self) -> Iter<Pairs> {
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
