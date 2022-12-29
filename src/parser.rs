use std::vec;

use crate::{
    ast::{atom::Atom, expr::Expr, program::Program, stmt::Stmt},
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
            if self.peek().kind == TokenType::Comment {
                self.eat(TokenType::Comment);
                continue;
            }

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
            TokenType::Func => self.parse_func_declaration(),
            TokenType::Return => self.parse_return_stmt(),
            TokenType::OpenBrace => self.parse_block_stmt(),
            TokenType::Print => self.parse_print_stmt(),
            _ => Stmt::Expression(self.parse_expression()),
        }
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_assignment_expr()
    }

    fn parse_print_stmt(&mut self) -> Stmt {
        self.eat(TokenType::Print);
        let expr = self.parse_expression();

        self.eat(TokenType::SemiColon);

        Stmt::Print(expr)
    }

    fn parse_block_stmt(&mut self) -> Stmt {
        self.eat(TokenType::OpenBrace);
        let mut statements: Vec<Stmt> = vec![];
        while self.peek().kind != TokenType::CloseBrace {
            statements.push(self.parse_statement());
        }
        self.eat(TokenType::CloseBrace);

        Stmt::Block(statements)
    }

    fn parse_return_stmt(&mut self) -> Stmt {
        self.eat(TokenType::Return);
        let expr = self.parse_expression();
        self.eat(TokenType::SemiColon);

        Stmt::Return(expr)
    }

    fn parse_func_declaration(&mut self) -> Stmt {
        self.eat(TokenType::Func);
        let identifier = self.eat(TokenType::Identifier).value;

        let mut parameters: Vec<String> = vec![];
        self.eat(TokenType::OpenParen);
        while self.peek().kind != TokenType::CloseParen {
            parameters.push(self.eat(TokenType::Identifier).value);
            if self.peek().kind == TokenType::Comma {
                self.eat(TokenType::Comma);
            }
        }
        self.eat(TokenType::CloseParen);

        let block = self.parse_block_stmt();

        Stmt::FuncDeclaration(identifier, parameters, Box::new(block))
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

        self.eat(TokenType::Colon);
        let typing = match self.eat(self.peek().kind).kind {
            TokenType::IntType => TokenType::Integer,
            TokenType::FloatType => TokenType::Float,
            TokenType::StringType => TokenType::String,
            TokenType::BoolType => TokenType::Bool,
            _ => panic!("Invalid variable type"),
        };

        self.eat(TokenType::Equal);
        let expr = self.parse_expression();

        self.eat(TokenType::SemiColon);

        Stmt::VarDeclaration(identifier.value, typing, is_const, expr)
    }

    fn parse_assignment_expr(&mut self) -> Expr {
        let left = self.parse_additive_expr();

        if self.peek().kind == TokenType::Equal {
            let op = self.advance();
            let right = self.parse_assignment_expr();
            self.eat(TokenType::SemiColon);

            return Expr::Assignment(Box::new(left), op.kind, Box::new(right));
        }

        left
    }

    fn parse_additive_expr(&mut self) -> Expr {
        let mut expr = self.parse_multiplicative_expr();

        while self.peek().kind == TokenType::Plus || self.peek().kind == TokenType::Minus {
            let op = self.advance();
            let right = self.parse_multiplicative_expr();

            expr = Expr::Binary(Box::new(expr), op.kind, Box::new(right));
        }

        expr
    }

    fn parse_multiplicative_expr(&mut self) -> Expr {
        let mut expr = self.parse_func_call_expr();

        while self.peek().kind == TokenType::Star || self.peek().kind == TokenType::Slash {
            let op = self.advance();
            let right = self.parse_func_call_expr();

            expr = Expr::Binary(Box::new(expr), op.kind, Box::new(right));
        }

        expr
    }

    fn parse_func_call_expr(&mut self) -> Expr {
        let primary = self.parse_primary_expr();
        match &primary {
            Expr::Identifier(name) => {
                if self.peek().kind != TokenType::OpenParen {
                    return primary;
                }

                self.eat(TokenType::OpenParen);
                let mut args: Vec<Expr> = vec![];
                while self.peek().kind != TokenType::CloseParen {
                    args.push(self.parse_expression());
                    if self.peek().kind == TokenType::Comma {
                        self.eat(TokenType::Comma);
                    }
                }
                self.eat(TokenType::CloseParen);

                Expr::CallExpr(name.clone(), args)
            }
            _ => primary,
        }
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
            TokenType::Bool => Expr::Literal(Atom::Bool(self.eat(TokenType::Bool).value == "true")),
            TokenType::OpenParen => {
                self.eat(TokenType::OpenParen);
                let expr = self.parse_expression();
                self.eat(TokenType::CloseParen);
                expr
            }
            TokenType::Identifier => Expr::Identifier(self.eat(TokenType::Identifier).value),
            _ => panic!("Parser error: unexpected token: {:?}", self.peek().kind),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_let_var_declaration() {
        let mut parser = Parser::new("let x: int = 1;");
        let ast = parser.parse();

        assert_eq!(
            ast.statements[0],
            Stmt::VarDeclaration(
                "x".to_string(),
                TokenType::Integer,
                false,
                Expr::Literal(Atom::Integer(1))
            )
        );
    }

    #[test]
    fn test_const_var_declaration() {
        let mut parser = Parser::new("const x: float = 1.1;");
        let ast = parser.parse();

        assert_eq!(
            ast.statements[0],
            Stmt::VarDeclaration(
                "x".to_string(),
                TokenType::Float,
                true,
                Expr::Literal(Atom::Float(1.1))
            )
        );
    }
}
