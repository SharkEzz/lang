use std::vec;

use crate::{
    ast::{
        atom::Atom,
        expr::Expr,
        program::{Program, Stmt},
    },
    eof_token,
    lexer::{tokenizer::Tokenizer, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let tokenizer = Tokenizer::new(source);
        let mut tokens: Vec<Token> = tokenizer.collect();
        tokens.reverse();

        Parser { tokens }
    }

    pub fn parse(&mut self) -> Program {
        let mut statements: Vec<Stmt> = vec![];

        while self.peek().kind != TokenType::EOF {
            statements.push(self.parse_statement());
        }

        Program { statements }
    }

    fn peek(&self) -> Token {
        let tok = self.tokens.last().cloned();
        match tok {
            None => eof_token!(),
            Some(token) => token,
        }
    }

    fn eat(&mut self, token_type: TokenType) -> Token {
        let token = self.advance();
        if token.kind != token_type {
            panic!("Expected {:?}, got {:?}", token_type, token.kind);
        }

        return token;
    }

    fn advance(&mut self) -> Token {
        let current = self.peek().clone();

        let next = self.tokens.pop();
        match next {
            None => eof_token!(),
            Some(_new) => current,
        }
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.peek().kind {
            TokenType::Let => self.parse_var_declaration(),
            TokenType::Const => self.parse_var_declaration(),
            _ => Stmt::Expression(self.parse_expression()),
        }
    }

    fn parse_var_declaration(&mut self) -> Stmt {
        let is_const: bool;
        let var_type = self.eat(self.peek().kind);
        match var_type.kind {
            TokenType::Let => is_const = false,
            TokenType::Const => is_const = true,
            _ => panic!("Expected let or const, got {:?}", var_type.kind),
        };

        let identifier = self.eat(TokenType::Identifier);
        self.eat(TokenType::Equal);
        let expr = self.parse_expression();

        Stmt::VarDeclaration(identifier.value, is_const, expr)
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> Expr {
        let mut expr = self.parse_multiplicative_expr();

        while self.peek().kind == TokenType::Plus || self.peek().kind == TokenType::Minus {
            let op = self.advance();
            let right = self.parse_multiplicative_expr();

            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_multiplicative_expr(&mut self) -> Expr {
        let mut expr = self.parse_primary_expr();

        while self.peek().kind == TokenType::Star || self.peek().kind == TokenType::Slash {
            let op = self.advance();
            let right = self.parse_primary_expr();

            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_primary_expr(&mut self) -> Expr {
        match self.peek().kind {
            TokenType::Integer => Expr::Literal(Atom::Integer(
                self.eat(TokenType::Integer)
                    .value
                    .parse()
                    .expect("Parser error: expected integer"),
            )),
            TokenType::Float => Expr::Literal(Atom::Float(
                self.eat(TokenType::Float)
                    .value
                    .parse()
                    .expect("Parser error: expected float"),
            )),
            TokenType::String => Expr::Literal(Atom::String(self.eat(TokenType::String).value)),
            TokenType::OpenParen => {
                self.eat(TokenType::OpenParen);
                let expr = self.parse_expression();
                self.eat(TokenType::CloseParen);
                expr
            }
            _ => panic!("Parser error: unexpected token: {:?}", self.peek().kind),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_let_var_declaration() {
        let mut parser = Parser::new("let x = 1");
        let ast = parser.parse();

        assert_eq!(
            ast.statements[0],
            Stmt::VarDeclaration("x".to_string(), false, Expr::Literal(Atom::Integer(1)))
        );
    }

    #[test]
    fn test_const_var_declaration() {
        let mut parser = Parser::new("const x = 1");
        let ast = parser.parse();

        assert_eq!(
            ast.statements[0],
            Stmt::VarDeclaration("x".to_string(), true, Expr::Literal(Atom::Integer(1)))
        );
    }
}
