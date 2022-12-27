use crate::{
    ast::{atom::Atom, expr::Expr, program::Program, stmt::Stmt},
    lexer::TokenType,
};

use super::{
    environment::Environment,
    values::{RuntimeError, RuntimeVal},
};

pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate_program(
        &self,
        program: &Program,
        env: &mut Environment,
    ) -> Result<RuntimeVal, RuntimeError> {
        let mut result = RuntimeVal::Null;

        for stmt in &program.statements {
            result = self.evaluate(stmt, env)?;
        }

        Ok(result)
    }

    fn evaluate(&self, stmt: &Stmt, env: &mut Environment) -> Result<RuntimeVal, RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate_expr(expr, env),
            Stmt::VarDeclaration(name, is_const, expr) => {
                self.evaluate_var_declaration_stmt(name, is_const, expr, env)
            }
            _ => unimplemented!("Statement not implemented"),
        }
    }

    fn evaluate_var_declaration_stmt(
        &self,
        name: &str,
        is_const: &bool,
        expr: &Expr,
        env: &mut Environment,
    ) -> Result<RuntimeVal, RuntimeError> {
        let val = self.evaluate_expr(expr, env)?;
        env.declare_var(name, val, *is_const)?;
        Ok(RuntimeVal::Null)
    }

    fn evaluate_expr(
        &self,
        expr: &Expr,
        env: &mut Environment,
    ) -> Result<RuntimeVal, RuntimeError> {
        match expr {
            Expr::Binary(lhs, op, rhs) => self.evaluate_binary_expr(lhs, op, rhs, env),
            Expr::Assignment(lhs, _, rhs) => self.evaluate_assignment_expr(lhs, rhs, env),
            Expr::Identifier(name) => self.evaluate_identifier(name, env),
            Expr::Literal(val) => self.evaluate_literal(val),
            _ => unimplemented!("Expression not implemented"),
        }
    }

    fn evaluate_assignment_expr(
        &self,
        lhs: &Expr,
        rhs: &Expr,
        env: &mut Environment,
    ) -> Result<RuntimeVal, RuntimeError> {
        match lhs {
            Expr::Identifier(name) => {
                let val = self.evaluate_expr(rhs, env)?;
                env.assign_var(name, val)?;
                Ok(RuntimeVal::Null)
            }
            _ => Err(RuntimeError::InvalidOperandType),
        }
    }

    fn evaluate_identifier(
        &self,
        name: &str,
        env: &mut Environment,
    ) -> Result<RuntimeVal, RuntimeError> {
        match env.get(name) {
            Some(val) => Ok(val.to_owned()),
            None => Err(RuntimeError::UndefinedVariable(name.to_string())),
        }
    }

    fn evaluate_binary_expr(
        &self,
        lhs: &Expr,
        op: &TokenType,
        rhs: &Expr,
        env: &mut Environment,
    ) -> Result<RuntimeVal, RuntimeError> {
        let left = self.evaluate_expr(lhs, env)?;
        let right = self.evaluate_expr(rhs, env)?;

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
