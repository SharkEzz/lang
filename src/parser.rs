use crate::{
    ast::{
        atom::Atom,
        expr::Expr,
        program::{Program, Stmt},
    },
    lexer::{tokenizer::Tokenizer, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let tokenizer = Tokenizer::new(source);

        Parser {
            tokens: tokenizer.collect(),
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut statements: Vec<Stmt> = vec![];

        while self.tokens.len() > 0 {
            statements.push(self.parse_stmt());
        }

        Program { statements }
    }

    fn eat(&mut self, token_type: TokenType) -> Token {
        let token = self.tokens[0].clone();
        if token.kind != token_type {
            panic!("Unexpected token: {:?}", token);
        }

        self.tokens.remove(0);

        return token;
    }

    // fn peek(&self) -> Option<Token> {
    //     let tok = self.tokens.get(1);
    //     match tok {
    //         Some(t) => Some(t.clone()),
    //         None => None,
    //     }
    // }

    fn at(&self) -> Token {
        self.tokens[0].clone()
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.at().kind {
            _ => Stmt::Expression(self.parse_expr()),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> Expr {
        let left = self.parse_additive_expr();

        if self.at().kind == TokenType::Equal {
            self.eat(TokenType::Equal);
            let right = self.parse_assignment_expr();

            return Expr::Assignment(Box::new(left), Box::new(right));
        }

        return left;
    }

    fn parse_additive_expr(&mut self) -> Expr {
        let left = self.parse_multiplicative_expr();

        while self.at().kind == TokenType::Plus || self.at().kind == TokenType::Minus {
            let op = self.eat(self.at().kind);
            let right = self.parse_multiplicative_expr();

            return Expr::Binary(Box::new(left), Atom::String(op.value), Box::new(right));
        }

        return left;
    }

    fn parse_multiplicative_expr(&mut self) -> Expr {
        let left = self.parse_primary();

        while self.at().kind == TokenType::Star || self.at().kind == TokenType::Slash {
            let op = self.eat(self.at().kind);
            let right = self.parse_primary();

            return Expr::Binary(Box::new(left), Atom::String(op.value), Box::new(right));
        }

        return left;
    }

    fn parse_primary(&mut self) -> Expr {
        match self.at().kind {
            TokenType::String => Expr::Literal(Atom::String(self.eat(TokenType::String).value)),
            TokenType::Float => Expr::Literal(Atom::Float(
                self.eat(TokenType::Float)
                    .value
                    .parse()
                    .expect("Invalid float"),
            )),
            TokenType::Integer => Expr::Literal(Atom::Integer(
                self.eat(TokenType::Integer)
                    .value
                    .parse()
                    .expect("Invalid integer"),
            )),
            TokenType::OpenParen => {
                self.eat(TokenType::OpenParen);
                let expr = self.parse_expr();
                self.eat(TokenType::CloseParen);
                expr
            }
            _ => panic!("Parser error: unexpected token: {:#?}", self.at()),
        }
    }
}
