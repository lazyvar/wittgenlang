use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable(String),
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Var {
        name: String,
        initializer: Option<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let lexer = crate::lexer::Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(&Token::Function) {
            return self.function("function");
        }
        if self.match_token(&Token::Let) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, String> {
        let name = if let Token::Identifier(name) = self.peek() {
            self.advance();
            name
        } else {
            return Err(format!("Expect {} name.", kind));
        };

        self.consume(&Token::LeftParen, &format!("Expect '(' after {} name.", kind))?;
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err("Cannot have more than 255 parameters.".to_string());
                }
                if let Token::Identifier(param) = self.consume(&Token::Identifier("".to_string()), "Expect parameter name.")? {
                    parameters.push(param);
                }
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        self.consume(&Token::RightParen, "Expect ')' after parameters.")?;
        self.consume(&Token::LeftBrace, &format!("Expect '{{' before {} body.", kind))?;
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
        }
        self.consume(&Token::RightBrace, "Expect '}' after body.")?;
        Ok(Stmt::Function { name, params: parameters, body })
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = if let Token::Identifier(name) = self.peek() {
            self.advance();
            name
        } else {
            return Err("Expect variable name.".to_string());
        };

        let initializer = if self.match_token(&Token::Equal) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(&Token::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&Token::Return) {
            return self.return_statement();
        }
        self.expression_statement()
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        let value = if !self.check(&Token::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(&Token::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(&Token::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_token(&Token::Equal) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_any(&[Token::Plus, Token::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_any(&[Token::Star, Token::Slash]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_any(&[Token::Bang, Token::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&Token::False) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }
        if self.match_token(&Token::True) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }
        if self.match_token(&Token::Nil) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if let Token::Number(n) = self.peek() {
            self.advance();
            return Ok(Expr::Literal(Literal::Number(n)));
        }
        if let Token::String(s) = self.peek() {
            self.advance();
            return Ok(Expr::Literal(Literal::String(s)));
        }
        if let Token::Identifier(name) = self.peek() {
            self.advance();
            return Ok(Expr::Variable(name));
        }
        if self.match_token(&Token::LeftParen) {
            let expr = self.expression()?;
            self.consume(&Token::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        Err("Expect expression.".to_string())
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_any(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek()) == std::mem::discriminant(token)
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::EOF)
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token: &Token, message: &str) -> Result<Token, String> {
        if self.check(token) {
            Ok(self.advance())
        } else {
            Err(message.to_string())
        }
    }
} 