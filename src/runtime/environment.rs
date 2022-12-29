use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::values::{RuntimeError, RuntimeVal};

pub type Env = Rc<RefCell<Environment>>;

pub struct Environment {
    parent: Option<Env>,
    variables: HashMap<String, RuntimeVal>,
    constants: HashSet<String>,
    functions: HashMap<String, RuntimeVal>,
}

impl Environment {
    pub fn new(parent: Option<Env>) -> Environment {
        let has_parent = parent.is_some();

        let mut env = Environment {
            parent,
            variables: HashMap::new(),
            constants: HashSet::new(),
            functions: HashMap::new(),
        };

        if !has_parent {
            env.setup_builtins();
        }

        return env;
    }

    pub fn get_var(&self, name: &str) -> Result<RuntimeVal, RuntimeError> {
        match self.variables.get(name) {
            Some(val) => Ok(val.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get_var(name),
                None => Err(RuntimeError::UndefinedVariable(name.to_string())),
            },
        }
    }

    pub fn get_func(&self, name: &str) -> Result<RuntimeVal, RuntimeError> {
        match self.functions.get(name) {
            Some(val) => Ok(val.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get_func(name),
                None => Err(RuntimeError::UndefinedFunction(name.to_string())),
            },
        }
    }

    pub fn declare_func(&mut self, name: &str, value: RuntimeVal) -> Result<(), RuntimeError> {
        if self.variables.contains_key(name) {
            return Err(RuntimeError::FuncRedeclaration(name.to_string()));
        }

        self.functions.insert(name.to_string(), value);
        Ok(())
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
    }
}
