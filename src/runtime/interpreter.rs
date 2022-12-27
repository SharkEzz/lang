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
            _ => unimplemented!("Statement not implemented"),
        }
    }

    fn evaluate_expr(
        &self,
        expr: &Expr,
        env: &mut Environment,
    ) -> Result<RuntimeVal, RuntimeError> {
        match expr {
            Expr::Binary(lhs, op, rhs) => self.evaluate_binary_expr(lhs, op, rhs, env),
            Expr::Literal(val) => self.evaluate_literal(val),
            _ => unimplemented!("Expression not implemented"),
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
