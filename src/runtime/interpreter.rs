use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{atom::Atom, expr::Expr, program::Program, stmt::Stmt},
    lexer::TokenType,
};

use super::{
    environment::{Env, Environment},
    values::{RuntimeError, RuntimeVal},
};

pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate_program(
        &self,
        program: &Program,
        env: Env,
    ) -> Result<RuntimeVal, RuntimeError> {
        let mut result = RuntimeVal::Undefined;

        for stmt in &program.statements {
            result = self.evaluate(stmt, Rc::clone(&env))?;
        }

        Ok(result)
    }

    fn evaluate(&self, stmt: &Stmt, env: Env) -> Result<RuntimeVal, RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate_expr(expr, env),
            Stmt::VarDeclaration(name, is_const, expr) => {
                self.evaluate_var_declaration_stmt(name, is_const, expr, env)
            }
            Stmt::FuncDeclaration(name, parameters, body) => {
                self.evaluate_func_declaration_stmt(name, parameters, body, env)
            }
            Stmt::Block(stmts) => self.evaluate_block_stmt(stmts, env),
            Stmt::Return(expr) => self.evaluate_return_stmt(expr, env),
        }
    }

    fn evaluate_func_declaration_stmt(
        &self,
        name: &str,
        parameters: &Vec<String>,
        body: &Stmt,
        env: Env,
    ) -> Result<RuntimeVal, RuntimeError> {
        match body {
            Stmt::Block(value) => {
                let func = RuntimeVal::Func(
                    name.to_string(),
                    parameters.clone(),
                    Stmt::Block(value.clone()),
                );
                env.borrow_mut().declare_func(name, func.clone())?;
                return Ok(func);
            }
            _ => panic!("Invalid function body"),
        }
    }

    fn evaluate_return_stmt(&self, expr: &Expr, env: Env) -> Result<RuntimeVal, RuntimeError> {
        let val = self.evaluate_expr(expr, env)?;
        Ok(RuntimeVal::Return(Box::new(val)))
    }

    fn evaluate_block_stmt(&self, stmts: &[Stmt], env: Env) -> Result<RuntimeVal, RuntimeError> {
        let mut values = vec![];
        let mut return_val = Box::new(RuntimeVal::Undefined);
        let block_env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&env)))));

        for stmt in stmts {
            let val = self.evaluate(stmt, Rc::clone(&block_env))?;
            match val {
                RuntimeVal::Return(_) => {
                    return_val = Box::new(val);
                    break;
                }
                _ => values.push(val),
            }
        }

        Ok(RuntimeVal::Block(return_val))
    }

    fn evaluate_var_declaration_stmt(
        &self,
        name: &str,
        is_const: &bool,
        expr: &Expr,
        env: Env,
    ) -> Result<RuntimeVal, RuntimeError> {
        let val = self.evaluate_expr(expr, Rc::clone(&env))?;
        env.borrow_mut().declare_var(name, val, *is_const)?;
        Ok(RuntimeVal::Undefined)
    }

    fn evaluate_expr(&self, expr: &Expr, env: Env) -> Result<RuntimeVal, RuntimeError> {
        match expr {
            Expr::Binary(lhs, op, rhs) => self.evaluate_binary_expr(lhs, op, rhs, env),
            Expr::CallExpr(name, params) => self.evaluate_func_call_expr(name, params, env),
            Expr::Assignment(lhs, _, rhs) => self.evaluate_assignment_expr(lhs, rhs, env),
            Expr::Identifier(name) => self.evaluate_identifier(name, env),
            Expr::Literal(val) => self.evaluate_literal(val),
        }
    }

    fn evaluate_func_call_expr(
        &self,
        name: &str,
        params: &Vec<Expr>,
        env: Env,
    ) -> Result<RuntimeVal, RuntimeError> {
        let func = env.borrow().get_func(name)?;
        match func {
            RuntimeVal::Func(_, func_params, body) => {
                let block_env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&env)))));
                for (i, name) in func_params.iter().enumerate() {
                    let param_value = self.evaluate_expr(&params[i], Rc::clone(&block_env))?;
                    env.borrow_mut().declare_var(name, param_value, false)?;
                }

                self.evaluate(&body, block_env)
            }
            _ => panic!("Expected a function"),
        }
    }

    fn evaluate_assignment_expr(
        &self,
        lhs: &Expr,
        rhs: &Expr,
        env: Env,
    ) -> Result<RuntimeVal, RuntimeError> {
        match lhs {
            Expr::Identifier(name) => {
                let val = self.evaluate_expr(rhs, Rc::clone(&env))?;
                env.borrow_mut().assign_var(name, val)?;
                Ok(RuntimeVal::Undefined)
            }
            _ => Err(RuntimeError::InvalidOperandType),
        }
    }

    fn evaluate_identifier(&self, name: &str, env: Env) -> Result<RuntimeVal, RuntimeError> {
        env.borrow().get_var(name)
    }

    fn evaluate_binary_expr(
        &self,
        lhs: &Expr,
        op: &TokenType,
        rhs: &Expr,
        env: Env,
    ) -> Result<RuntimeVal, RuntimeError> {
        let left = self.evaluate_expr(lhs, Rc::clone(&env))?;
        let right = self.evaluate_expr(rhs, Rc::clone(&env))?;

        match left {
            RuntimeVal::Int(left) => match right {
                RuntimeVal::Int(right) => match op {
                    TokenType::Plus => Ok(RuntimeVal::Int(left + right)),
                    TokenType::Minus => Ok(RuntimeVal::Int(left - right)),
                    TokenType::Star => Ok(RuntimeVal::Int(left * right)),
                    TokenType::Slash => {
                        if right == 0 {
                            return Err(RuntimeError::DivisionByZero);
                        }
                        Ok(RuntimeVal::Int(left / right))
                    }
                    _ => Err(RuntimeError::InvalidOperandType),
                },
                RuntimeVal::Float(right) => match op {
                    TokenType::Plus => Ok(RuntimeVal::Float(left as f64 + right)),
                    TokenType::Minus => Ok(RuntimeVal::Float(left as f64 - right)),
                    TokenType::Star => Ok(RuntimeVal::Float(left as f64 * right)),
                    TokenType::Slash => {
                        if right == 0.0 {
                            return Err(RuntimeError::DivisionByZero);
                        }
                        Ok(RuntimeVal::Float(left as f64 / right))
                    }
                    _ => Err(RuntimeError::InvalidOperandType),
                },
                _ => Err(RuntimeError::InvalidOperandType),
            },
            RuntimeVal::Float(left) => match right {
                RuntimeVal::Int(right) => match op {
                    TokenType::Plus => Ok(RuntimeVal::Float(left + (right as f64))),
                    TokenType::Minus => Ok(RuntimeVal::Float(left - (right as f64))),
                    TokenType::Star => Ok(RuntimeVal::Float(left * (right as f64))),
                    TokenType::Slash => Ok(RuntimeVal::Float(left / (right as f64))),
                    _ => Err(RuntimeError::InvalidOperandType),
                },
                RuntimeVal::Float(right) => match op {
                    TokenType::Plus => Ok(RuntimeVal::Float(left + right)),
                    TokenType::Minus => Ok(RuntimeVal::Float(left - right)),
                    TokenType::Star => Ok(RuntimeVal::Float(left * right)),
                    TokenType::Slash => Ok(RuntimeVal::Float(left / right)),
                    _ => Err(RuntimeError::InvalidOperandType),
                },
                _ => Err(RuntimeError::InvalidOperandType),
            },
            RuntimeVal::String(left) => match right {
                RuntimeVal::String(right) => match op {
                    TokenType::Plus => Ok(RuntimeVal::String(left + &right)),
                    _ => Err(RuntimeError::InvalidOperandType),
                },
                _ => Err(RuntimeError::InvalidOperandType),
            },
            _ => unimplemented!("Binary expression not implemented"),
        }
    }

    fn evaluate_literal(&self, val: &Atom) -> Result<RuntimeVal, RuntimeError> {
        match val {
            Atom::Integer(num) => Ok(RuntimeVal::Int(*num)),
            Atom::Float(num) => Ok(RuntimeVal::Float(*num)),
            Atom::String(string) => Ok(RuntimeVal::String(string.clone())),
        }
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{parser::Parser, runtime::environment::Environment};

    use super::*;

    fn init(source: &str) -> Result<RuntimeVal, RuntimeError> {
        let mut parser = Parser::new(source);
        let program = parser.parse();

        let env = Rc::new(RefCell::new(Environment::new(None)));
        let runtime = Interpreter {};

        runtime.evaluate_program(&program, env)
    }

    #[test]
    fn additive_expr() {
        let result = init("1 + 1").expect("Failed to evaluate");
        assert_eq!(result, RuntimeVal::Int(2));
    }

    #[test]
    fn substractive_expr() {
        let result = init("1 - 2").expect("Failed to evaluate");
        assert_eq!(result, RuntimeVal::Int(-1));
    }

    #[test]
    fn multiplicative_expr() {
        let result = init("2 * 2").expect("Failed to evaluate");
        assert_eq!(result, RuntimeVal::Int(4));
    }

    #[test]
    fn division_expr() {
        let result = init("2 / 2").expect("Failed to evaluate");
        assert_eq!(result, RuntimeVal::Int(1));
    }

    #[test]
    fn division_by_zero_expr() {
        let result = init("2 / 0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RuntimeError::DivisionByZero);
    }

    #[test]
    fn operation_with_variables() {
        let result = init(
            "
        let t = 1 + 1;
        t * 2
        ",
        )
        .expect("Failed to evaluate");
        assert_eq!(result, RuntimeVal::Int(4));
    }

    #[test]
    fn block() {
        let result = init(
            "
        {
            let t = 1 + 1;
            t * 2

            return t;
        }
        ",
        )
        .expect("Failed to evaluate");
        assert_eq!(
            result,
            RuntimeVal::Block(Box::new(RuntimeVal::Return(Box::new(RuntimeVal::Int(2)))))
        );
    }
}
