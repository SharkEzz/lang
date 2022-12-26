use std::collections::HashMap;

use super::values::RuntimeVal;

pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub variables: HashMap<String, RuntimeVal>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Environment {
        let should_setup_builtins = parent.is_none();

        let mut env = Environment {
            parent,
            variables: HashMap::new(),
        };

        if should_setup_builtins {
            env.setup_builtins();
        }

        return env;
    }

    pub fn get(&self, name: &str) -> Option<&RuntimeVal> {
        match self.variables.get(name) {
            Some(val) => Some(val),
            None => match &self.parent {
                Some(parent) => parent.get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, value: RuntimeVal) {
        if self.variables.contains_key(&name) {
            panic!("Variable {} already exists", name);
        }

        self.variables.insert(name, value);
    }

    fn setup_builtins(&mut self) {
        self.set("true".to_string(), RuntimeVal::Bool(true));
        self.set("false".to_string(), RuntimeVal::Bool(false));
        self.set("null".to_string(), RuntimeVal::Null);
    }
}
