use std::collections::{HashMap, HashSet};

use super::values::{RuntimeError, RuntimeVal};

pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeVal>,
    constants: HashSet<String>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Environment {
        let has_parent = parent.is_some();

        let mut env = Environment {
            parent,
            variables: HashMap::new(),
            constants: HashSet::new(),
        };

        if has_parent {
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

    pub fn set(
        &mut self,
        name: String,
        value: RuntimeVal,
        is_const: bool,
    ) -> Result<(), RuntimeError> {
        if is_const && self.constants.contains(&name) {
            return Err(RuntimeError::CannotRedefineConstant(format!(
                "Cannot redeclare constant variable: {}",
                name
            )));
        }

        self.variables.insert(name, value);
        Ok(())
    }

    fn setup_builtins(&mut self) {
        self.set("true".to_string(), RuntimeVal::Bool(true), true)
            .expect("Failed to initialize builtins");
        self.set("false".to_string(), RuntimeVal::Bool(false), true)
            .expect("Failed to initialize builtins");
        self.set("null".to_string(), RuntimeVal::Null, true)
            .expect("Failed to initialize builtins");
    }
}
