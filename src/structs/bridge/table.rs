use crate::structs::DefaultTypes;
use std::collections::HashMap;
use std::collections::hash_map::Iter;

#[derive(Clone, Debug)]
pub struct TableImpl {
    data: HashMap<String, DefaultTypes>,
}


impl TableImpl {
    pub fn new() -> TableImpl {
        TableImpl { data: HashMap::new() }
    }
    pub fn get(&self, s2: &str) -> Option<DefaultTypes> {
        self.data.get(s2).cloned()
    }
    pub fn set(&mut self, k: String, v: DefaultTypes) {
        self.data.insert(k, v);
    }
    pub fn remove(&mut self, k: String) {
        self.data.remove(&k);
    }

    pub fn iter_data(&self) -> Iter<String, DefaultTypes> {
        self.data.iter()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
