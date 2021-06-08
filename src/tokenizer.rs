use crate::structs::{DefaultTypes, DefaultTypes::Function, Env, Statement, Table};

#[derive(Clone)]
pub struct ParsedTable {
    table: DefaultTypes,
    key: String,
    nest: Vec<(String, Table)>,
    name: String,
}

pub enum ParsedResult {
    Table(ParsedTable),
    Normal(DefaultTypes),
    Error(String),
}

impl ParsedTable {
    pub(crate) fn value(&self) -> DefaultTypes {
        match &self.table {

            DefaultTypes::Table(t) => {
                t.raw_get(&self.key).expect("Invalid table")
            },
            _ => {
                panic!("Oh frick");
            }
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    fn raw(&self) -> Table {
        match &self.table {
            DefaultTypes::Table(t) => {t.clone()},
            _ => { panic!("Invalid Table") }
        }
    }
    pub fn set(&self, val: DefaultTypes) -> DefaultTypes {
        let mut last_t = self.raw();
        last_t.set(self.key.clone(), val);
        let nests = self.nest[0..self.nest.len()].iter().collect::<Vec<_>>();
        let elems: Vec<(&String, &Table)> = nests[1..nests.len()].iter().map(|(x, y)| (x, y)).collect();
        for (k, mut t) in elems {
            let mut newt = t.clone();
            newt.set(k.to_string(), DefaultTypes::Table(last_t.clone()));
            last_t = newt.clone();
        }
        DefaultTypes::Table(last_t)
    }
}
impl ParsedResult {
    pub(crate) fn as_type(&self) -> Result<DefaultTypes, String> {
        match self {
            ParsedResult::Normal(n) => { Ok(n.clone()) }
            ParsedResult::Table(tab) => {
                Ok(tab.value())
            }
            ParsedResult::Error(s) => {
                Err(s.clone())
            }
        }
    }
}

fn as_table(r: DefaultTypes) -> Table {
    if let DefaultTypes::Table(table) = r {
        table
    } else {
        panic!("Did not get a table");
    }
}

fn handle_table(ss: &str, env: &mut Env, sss: &Statement) -> Option<ParsedResult> {
    let t = ss.clone();
    let mut found = None;
    let sk = (t.split("(").next().unwrap()).to_string();
    let split = sk.split(".");
    let cc = env.get(split.clone().collect::<Vec<&str>>().get(0).unwrap()).expect("Could not find table");
    if let DefaultTypes::Table(mut current_t) = cc.clone() {
        let iterer = split.collect::<Vec<&str>>();
        let mut stuff = Vec::new();
        stuff.push((iterer[1].to_string(), current_t.clone()));
        let slice;
        slice = &iterer[1..iterer.len()-1];
        if slice.len()>0 {
            for (i, frag) in slice.iter().enumerate() {
                current_t = as_table(current_t.raw_get(frag).unwrap());
                stuff.push((frag.to_string(), current_t.clone()));
            }
        }
        stuff.reverse();
        let last_frag = &iterer.last().unwrap();
        let tab = ParsedTable {
            table: DefaultTypes::from(current_t),
            key: last_frag.to_string(),
            nest: stuff,
            name: iterer[0].parse().unwrap(),
        };
        found = Some(ParsedResult::Table(
            tab.clone()
        ));
        if t.ends_with('(') {
            if let DefaultTypes::Function(func) = tab.value() {
                let args = get_args(&t, env, sss);
                found = Some(ParsedResult::Normal(func.call(env, args.expect("Could not find args"))[0].clone()))
            }
        }
    } else {
        found = Some(ParsedResult::Error("Invalid stuff lol".parse().unwrap()))
    }
    found
}
pub fn parse_exp(ss: &str, env: &mut Env, sss: &Statement) -> ParsedResult {
    let k; // Wtf is this idk
    let t = String::from(ss);
    if ss == "[]" {
        ParsedResult::Normal(DefaultTypes::Table(Table::new()))
    } else if t.contains(".") && !ss.starts_with('"') {
        handle_table(&t, env, sss).expect("Attempted to index non existing table")
    } else {
        if ss.starts_with('"') && ss.ends_with('"') {
            k = Ok(String::from(ss));
        } else {
            let temp = ss.split('(').next();
            match temp {
                None => {
                    k = Err("Attempting to parse nothing".to_string());
                }
                Some(val) => {
                    k = Ok(val.parse::<String>().unwrap_or_else(|_| "".to_string()));
                }
            }
        }
        let s = k.expect("Woopsies");
        if s.parse::<i32>().is_ok() {
            ParsedResult::Normal(DefaultTypes::Int(
                s.parse::<i32>().expect("Something odd happened here"),
            ))
        } else if s.starts_with('"') && s.ends_with('"') {
            ParsedResult::Normal(DefaultTypes::Str(
                String::from(&s[1..s.len() - 1])
                    .replace("|_", " ")
                    .replace("|-", "\n")
                    .replace(r"\/", r"/"),
            ))
        } else if s == "true" {
            ParsedResult::Normal(DefaultTypes::Bool(true))
        } else if s == "false" {
            ParsedResult::Normal(DefaultTypes::Bool(false))
        } else if env.contains(&s) {
            let dt = env.get(&s).expect("Something really odd happened"); // This should logically never panic since env.contains was true
            match dt {
                DefaultTypes::Function(f) => {
                    if ss.ends_with('(') {
                        let args = get_args(&t, env, sss);
                        ParsedResult::Normal(f.call(env, args.expect("oop"))[0].clone())
                    } else {
                        ParsedResult::Normal(Function(f))
                    }
                }
                DefaultTypes::Table(tab) => ParsedResult::Normal(DefaultTypes::Table(tab)),
                _ => ParsedResult::Normal(dt),
            }
        } else {
            let err = format!("{}:{}", "Could not parse expression: ", &t);
            ParsedResult::Error(err)
        }
    }
}

/// # Errors
pub fn get_args(s: &str, env: &mut Env, ss: &Statement) -> Result<Vec<DefaultTypes>, String> {
    ss.get_function_call_args_indexed(env, s)
}
