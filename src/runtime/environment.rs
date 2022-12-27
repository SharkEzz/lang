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

        if !has_parent {
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

    pub fn declare_var(
        &mut self,
        name: &str,
        value: RuntimeVal,
        is_const: bool,
    ) -> Result<(), RuntimeError> {
        if self.variables.contains_key(name) {
            return Err(RuntimeError::VarRedeclaration(name.to_string()));
        }

        self.variables.insert(name.to_string(), value);
        if is_const {
            self.constants.insert(name.to_string());
        }

        Ok(())
    }

    pub fn assign_var(&mut self, name: &str, value: RuntimeVal) -> Result<(), RuntimeError> {
        let old_var = self.variables.remove(name);
        match old_var {
            None => return Err(RuntimeError::UndefinedVariable(name.to_string())),
            Some(_) => {
                if self.constants.contains(name) {
                    return Err(RuntimeError::ConstantReassignment(name.to_string()));
                }
                self.variables.insert(name.to_string(), value);
            }
        }

        Ok(())
    }

    fn setup_builtins(&mut self) {
        self.declare_var("true", RuntimeVal::Bool(true), true)
            .expect("Failed to initialize builtins");
        self.declare_var("false", RuntimeVal::Bool(false), true)
            .expect("Failed to initialize builtins");
        self.declare_var("null", RuntimeVal::Null, true)
            .expect("Failed to initialize builtins");
    }
}
