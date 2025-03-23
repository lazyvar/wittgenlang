use crate::lexer::Token;

#[derive(Debug, Clone)]
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
    FunctionCall {
        name: String,
        arguments: Vec<Expr>,
        named_arguments: Vec<(String, Expr)>,
    },
    TypeFunctionCall {
        object: Box<Expr>,
        function: String,
    },
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Record {
        type_name: String,
        fields: Vec<(String, Expr)>, 
    },
    TypeAnnotation {
        expr: Box<Expr>,
        type_name: String,
    },
    Lambda {
        params: Vec<(String, String)>, // (name, type)
        body: Box<Expr>,
    },
    AccessExpression {
        object: Box<Expr>,
        index: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    Integer(i64),
    String(String),
    Decision(bool),
    Nothing,
}

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(String),
    Custom(String),
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Union(Vec<Type>),
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Value {
        name: String,
        type_name: String,
        initializer: Expr,
        mutable: bool,
    },
    Function {
        name: String,
        return_type: String,
        params: Vec<(String, String)>, // (name, type)
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    Unless {
        condition: Expr,
        body: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        variable: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    Of {
        value: Expr,
        cases: Vec<(Vec<Expr>, Vec<Stmt>)>, // Multiple expressions can map to same branch
        default: Option<Vec<Stmt>>,
    },
    Change {
        name: String,
        value: Expr,
    },
    Break,
    Continue,
    Produce(Option<Expr>),
    ModuleDeclaration {
        name: String,
        body: Vec<Stmt>,
    },
    TypeDefinition {
        name: String,
        definition: TypeDefinition,
    },
    Import {
        module_path: Vec<String>,
        specific_imports: Vec<String>,
        alias: Option<String>,
    },
    Write(Expr),
}

#[derive(Debug, Clone)]
pub enum TypeDefinition {
    Alias(Type),
    Record {
        fields: Vec<(String, String)>, // field name, field type
    },
    Variant {
        variants: Vec<(String, Vec<(String, String)>)>, // variant name, fields
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
        if self.match_token(&Token::Module) {
            return self.module_declaration();
        }
        if self.match_token(&Token::By) {
            return self.function_declaration();
        }
        if self.match_token(&Token::See) {
            return self.type_definition();
        }
        if self.match_token(&Token::Import) {
            return self.import_declaration();
        }
        if self.match_token(&Token::ForNow) {
            return self.variable_declaration(true);
        }
        if self.match_token(&Token::TypePrefix) {
            // This is for handling type annotations 
            return self.type_annotation();
        }
        if self.match_token(&Token::Write) {
            return self.write_statement();
        }
        if let Token::Identifier(_) = self.peek() {
            // Look ahead to check if this is a value declaration with "is"
            let next_pos = self.current + 1;
            if next_pos < self.tokens.len() && 
               (matches!(self.tokens[next_pos], Token::TypePrefix | Token::NumberType | 
                         Token::TextType | Token::DecisionType | Token::NothingType |
                         Token::BlissType | Token::AnyType)) {
                return self.variable_declaration(false);
            }
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

        // Add a dummy return type for backward compatibility
        let return_type = "Any".to_string();

        self.consume(&Token::LeftParen, &format!("Expect '(' after {} name.", kind))?;
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err("Cannot have more than 255 parameters.".to_string());
                }
                if let Token::Identifier(param) = self.consume(&Token::Identifier("".to_string()), "Expect parameter name.")? {
                    // Make each parameter a tuple with a dummy type for compatibility
                    parameters.push((param, "Any".to_string()));
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
        Ok(Stmt::Function { name, return_type, params: parameters, body })
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = if let Token::Identifier(name) = self.peek() {
            self.advance();
            name
        } else {
            return Err("Expect variable name.".to_string());
        };

        // For now, use a dummy type
        let type_name = "Any".to_string();

        // For now, always require an initializer (no optionals)
        let initializer = self.expression()?;

        Ok(Stmt::Value {
            name,
            type_name, 
            initializer,
            mutable: false,
        })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&Token::If) {
            return self.if_statement();
        }
        if self.match_token(&Token::Unless) {
            return self.unless_statement();
        }
        if self.match_token(&Token::While) {
            return self.while_statement();
        }
        if self.match_token(&Token::For) {
            return self.for_statement();
        }
        if self.match_token(&Token::Of) {
            return self.of_statement();
        }
        if self.match_token(&Token::Change) {
            return self.change_statement();
        }
        if self.match_token(&Token::Break) {
            return Ok(Stmt::Break);
        }
        if self.match_token(&Token::Continue) {
            return Ok(Stmt::Continue);
        }
        if self.match_token(&Token::Produce) {
            return self.produce_statement();
        }
        
        // Handle expression statement
        let expr = self.expression()?;
        Ok(Stmt::Expression(expr))
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;
        
        self.consume(&Token::LeftBrace, "Expected '{' after if condition")?;
        
        let mut then_branch = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            then_branch.push(self.declaration()?);
        }
        
        self.consume(&Token::RightBrace, "Expected '}' after if block")?;
        
        let else_branch = if self.match_token(&Token::Else) {
            if self.match_token(&Token::If) {
                // Handle else-if as a nested if in the else branch
                let if_stmt = self.if_statement()?;
                Some(vec![if_stmt])
            } else {
                self.consume(&Token::LeftBrace, "Expected '{' after else")?;
                
                let mut else_stmts = Vec::new();
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    else_stmts.push(self.declaration()?);
                }
                
                self.consume(&Token::RightBrace, "Expected '}' after else block")?;
                
                Some(else_stmts)
            }
        } else {
            None
        };
        
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    fn unless_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;
        
        self.consume(&Token::LeftBrace, "Expected '{' after unless condition")?;
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
        }
        
        self.consume(&Token::RightBrace, "Expected '}' after unless block")?;
        
        Ok(Stmt::Unless {
            condition,
            body,
        })
    }
    
    fn while_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;
        
        self.consume(&Token::LeftBrace, "Expected '{' after while condition")?;
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
        }
        
        self.consume(&Token::RightBrace, "Expected '}' after while block")?;
        
        Ok(Stmt::While {
            condition,
            body,
        })
    }
    
    fn for_statement(&mut self) -> Result<Stmt, String> {
        let variable = if let Token::Identifier(name) = self.peek() {
            self.advance();
            name
        } else {
            return Err("Expected variable name after 'for'".to_string());
        };
        
        self.consume(&Token::In, "Expected 'in' after for loop variable")?;
        
        let iterable = self.expression()?;
        
        self.consume(&Token::LeftBrace, "Expected '{' after for loop iterable")?;
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
        }
        
        self.consume(&Token::RightBrace, "Expected '}' after for loop body")?;
        
        Ok(Stmt::For {
            variable,
            iterable,
            body,
        })
    }
    
    fn of_statement(&mut self) -> Result<Stmt, String> {
        // Parse the value to switch on
        let value = self.expression()?;
        
        self.consume(&Token::LeftBrace, "Expected '{' after switch value")?;
        
        let mut cases = Vec::new();
        let mut default = None;
        
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            if self.match_token(&Token::Otherwise) {
                // Handle default case
                self.consume(&Token::Arrow, "Expected '->' after 'otherwise'")?;
                
                let mut default_stmts = Vec::new();
                if self.match_token(&Token::LeftBrace) {
                    // Multiple statements in default case
                    while !self.check(&Token::RightBrace) && !self.is_at_end() {
                        default_stmts.push(self.declaration()?);
                    }
                    self.consume(&Token::RightBrace, "Expected '}' after default case block")?;
                } else {
                    // Single statement in default case
                    default_stmts.push(self.declaration()?);
                }
                
                default = Some(default_stmts);
            } else {
                // Handle regular case
                let mut case_values = Vec::new();
                
                // Parse case values (can be multiple separated by commas)
                loop {
                    case_values.push(self.expression()?);
                    
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
                
                self.consume(&Token::Arrow, "Expected '->' after case value")?;
                
                let mut case_stmts = Vec::new();
                if self.match_token(&Token::LeftBrace) {
                    // Multiple statements in case
                    while !self.check(&Token::RightBrace) && !self.is_at_end() {
                        case_stmts.push(self.declaration()?);
                    }
                    self.consume(&Token::RightBrace, "Expected '}' after case block")?;
                } else {
                    // Single statement in case
                    case_stmts.push(self.declaration()?);
                }
                
                cases.push((case_values, case_stmts));
            }
        }
        
        self.consume(&Token::RightBrace, "Expected '}' after switch statement")?;
        
        Ok(Stmt::Of {
            value,
            cases,
            default,
        })
    }
    
    fn produce_statement(&mut self) -> Result<Stmt, String> {
        let value = if self.check(&Token::RightBrace) {
            None
        } else {
            Some(self.expression()?)
        };
        
        Ok(Stmt::Produce(value))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.logic_or()?;
        
        if self.match_token(&Token::Is) {
            if let Expr::Variable(name) = expr {
                let value = self.assignment()?;
                // In Wittgenlang, this would be a variable declaration without a type,
                // but we'll handle it as an assignment for simplicity
                return Ok(Expr::Binary {
                    left: Box::new(Expr::Variable(name)),
                    operator: Token::Is,
                    right: Box::new(value),
                });
            } else {
                return Err("Invalid assignment target.".to_string());
            }
        }
        
        Ok(expr)
    }
    
    fn logic_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.logic_and()?;
        
        while self.match_token(&Token::Pipe) {
            let operator = self.previous();
            let right = self.logic_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn logic_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        
        while self.match_token(&Token::Ampersand) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        
        while self.match_any(&[Token::EqualEqual, Token::NotEqual, Token::Is]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        
        while self.match_any(&[
            Token::Less, Token::LessEqual, 
            Token::Greater, Token::GreaterEqual
        ]) {
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
        
        while self.match_any(&[Token::Plus, Token::Minus, Token::Ampersand]) {
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
        
        while self.match_any(&[
            Token::Star, Token::Slash, Token::IntegerDivide, 
            Token::Modulo, Token::Power
        ]) {
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
        if self.match_any(&[Token::Minus, Token::ExclamationMark]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        
        self.call()
    }
    
    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;
        
        loop {
            if self.match_token(&Token::LeftParen) {
                // Function call
                expr = self.finish_call(expr)?;
            } else if self.match_token(&Token::Apostrophe) {
                // Type function call: object'function
                if let Token::Identifier(function) = self.peek() {
                    self.advance();
                    expr = Expr::TypeFunctionCall {
                        object: Box::new(expr),
                        function,
                    };
                } else {
                    return Err("Expected function name after apostrophe".to_string());
                }
            } else if self.match_token(&Token::LeftBracket) {
                // List or map access: list[index]
                let index = self.expression()?;
                self.consume(&Token::RightBracket, "Expected ']' after index")?;
                
                expr = Expr::AccessExpression {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&Token::Dot) {
                // Method call with no arguments: object.method
                if let Token::Identifier(method) = self.peek() {
                    self.advance();
                    expr = Expr::FunctionCall {
                        name: method,
                        arguments: vec![expr],
                        named_arguments: Vec::new(),
                    };
                } else {
                    return Err("Expected method name after dot".to_string());
                }
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = Vec::new();
        let mut named_arguments = Vec::new();
        
        if !self.check(&Token::RightParen) {
            loop {
                if self.peek_ahead(1) == Some(&Token::Colon) {
                    // Named argument: param: value
                    if let Token::Identifier(name) = self.peek() {
                        self.advance(); // Consume parameter name
                        self.advance(); // Consume colon
                        let value = self.expression()?;
                        named_arguments.push((name, value));
                    } else {
                        return Err("Expected parameter name for named argument".to_string());
                    }
                } else {
                    // Positional argument
                    arguments.push(self.expression()?);
                }
                
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&Token::RightParen, "Expected ')' after arguments")?;
        
        let function_name = if let Expr::Variable(name) = callee {
            name
        } else {
            return Err("Expected function name".to_string());
        };
        
        Ok(Expr::FunctionCall {
            name: function_name,
            arguments,
            named_arguments,
        })
    }
    
    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&Token::Yes) {
            return Ok(Expr::Literal(Literal::Decision(true)));
        }
        if self.match_token(&Token::No) {
            return Ok(Expr::Literal(Literal::Decision(false)));
        }
        if self.match_token(&Token::Nothing) {
            return Ok(Expr::Literal(Literal::Nothing));
        }
        
        if let Token::Number(n) = self.peek() {
            self.advance();
            return Ok(Expr::Literal(Literal::Number(n)));
        }
        
        if let Token::String(s) = self.peek() {
            self.advance();
            return Ok(Expr::Literal(Literal::String(s)));
        }
        
        if let Token::MultilineString(s) = self.peek() {
            self.advance();
            return Ok(Expr::Literal(Literal::String(s)));
        }
        
        if let Token::Identifier(name) = self.peek() {
            self.advance();
            return Ok(Expr::Variable(name));
        }
        
        if self.match_token(&Token::LeftParen) {
            let expr = self.expression()?;
            self.consume(&Token::RightParen, "Expected ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        
        if self.match_token(&Token::LeftBracket) {
            // List literal
            let mut elements = Vec::new();
            
            if !self.check(&Token::RightBracket) {
                loop {
                    elements.push(self.expression()?);
                    
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            
            self.consume(&Token::RightBracket, "Expected ']' after list elements")?;
            return Ok(Expr::List(elements));
        }
        
        if self.match_token(&Token::LeftBrace) {
            // Map literal or record
            if let Some(Token::Identifier(_)) = self.peek_ahead(1) {
                if let Some(Token::Colon) = self.peek_ahead(2) {
                    // This is a map with string keys
                    let mut entries = Vec::new();
                    
                    if !self.check(&Token::RightBrace) {
                        loop {
                            let key = self.expression()?;
                            self.consume(&Token::Colon, "Expected ':' after map key")?;
                            let value = self.expression()?;
                            
                            entries.push((key, value));
                            
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                    }
                    
                    self.consume(&Token::RightBrace, "Expected '}' after map entries")?;
                    return Ok(Expr::Map(entries));
                }
            }
        }
        
        Err("Expected expression".to_string())
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

    fn module_declaration(&mut self) -> Result<Stmt, String> {
        let name = if let Token::Identifier(name) = self.peek() {
            self.advance();
            name
        } else {
            return Err("Expected module name".to_string());
        };

        self.consume(&Token::LeftBrace, "Expected '{' after module name")?;
        
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
        }
        
        self.consume(&Token::RightBrace, "Expected '}' after module body")?;
        
        Ok(Stmt::ModuleDeclaration { name, body })
    }

    fn function_declaration(&mut self) -> Result<Stmt, String> {
        // In Wittgenlang, functions are declared with:
        // function-name #ReturnType by { @param1 #Type1 ... body }
        
        // Get the function name and return type
        let name = if let Token::Identifier(name) = self.previous_token() {
            name
        } else {
            return Err("Expected function name before 'by'".to_string());
        };
        
        let return_type = if self.match_token(&Token::TypePrefix) {
            if let Token::Identifier(type_name) = self.peek() {
                self.advance();
                type_name
            } else {
                return Err("Expected return type after '#'".to_string());
            }
        } else if self.match_any(&[
            Token::NumberType, Token::TextType, Token::DecisionType, 
            Token::NothingType, Token::BlissType, Token::AnyType
        ]) {
            format!("{:?}", self.previous())
        } else {
            return Err("Expected return type before 'by'".to_string());
        };
        
        self.consume(&Token::LeftBrace, "Expected '{' after 'by'")?;
        
        // Parse parameters (starting with @)
        let mut params = Vec::new();
        while self.match_token(&Token::ExclamationMark) {
            // Skip comments inside function body
            self.skip_comment_line();
            continue;
        }
        
        while self.match_token(&Token::Ampersand) {
            let param_name = if let Token::Identifier(name) = self.peek() {
                self.advance();
                name
            } else {
                return Err("Expected parameter name after '@'".to_string());
            };
            
            let param_type = if self.match_token(&Token::TypePrefix) {
                if let Token::Identifier(type_name) = self.peek() {
                    self.advance();
                    type_name
                } else {
                    return Err("Expected parameter type after '#'".to_string());
                }
            } else if self.match_any(&[
                Token::NumberType, Token::TextType, Token::DecisionType, 
                Token::NothingType, Token::BlissType, Token::AnyType
            ]) {
                format!("{:?}", self.previous())
            } else {
                return Err("Expected parameter type".to_string());
            };
            
            params.push((param_name, param_type));
        }
        
        // Parse function body
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.declaration()?);
        }
        
        self.consume(&Token::RightBrace, "Expected '}' after function body")?;
        
        Ok(Stmt::Function {
            name,
            return_type,
            params,
            body,
        })
    }
    
    fn variable_declaration(&mut self, mutable: bool) -> Result<Stmt, String> {
        // Parse variable name
        let name = if let Token::Identifier(name) = self.peek() {
            self.advance();
            name
        } else {
            return Err("Expected variable name".to_string());
        };
        
        // Parse type annotation
        let type_name = if self.match_token(&Token::TypePrefix) {
            if let Token::Identifier(type_name) = self.peek() {
                self.advance();
                type_name
            } else {
                return Err("Expected type name after '#'".to_string());
            }
        } else if self.match_any(&[
            Token::NumberType, Token::TextType, Token::DecisionType, 
            Token::NothingType, Token::BlissType, Token::AnyType
        ]) {
            format!("{:?}", self.previous())
        } else {
            return Err("Expected type annotation for variable".to_string());
        };
        
        // Parse initializer (required in Wittgenlang)
        self.consume(&Token::Is, "Expected 'is' after type annotation")?;
        let initializer = self.expression()?;
        
        Ok(Stmt::Value {
            name,
            type_name,
            initializer,
            mutable,
        })
    }
    
    fn change_statement(&mut self) -> Result<Stmt, String> {
        // Parse "change variable to newValue"
        let name = if let Token::Identifier(name) = self.peek() {
            self.advance();
            name
        } else {
            return Err("Expected variable name after 'change'".to_string());
        };
        
        self.consume(&Token::To, "Expected 'to' after variable name in change statement")?;
        let value = self.expression()?;
        
        Ok(Stmt::Change { name, value })
    }
    
    fn write_statement(&mut self) -> Result<Stmt, String> {
        self.consume(&Token::LeftParen, "Expected '(' after 'write'")?;
        let expr = self.expression()?;
        self.consume(&Token::RightParen, "Expected ')' after write expression")?;
        
        Ok(Stmt::Write(expr))
    }
    
    fn type_definition(&mut self) -> Result<Stmt, String> {
        // Parse "#TypeName is ..." type definition
        self.consume(&Token::TypePrefix, "Expected '#' after 'see'")?;
        
        let type_name = if let Token::Identifier(name) = self.peek() {
            self.advance();
            name
        } else {
            return Err("Expected type name".to_string());
        };
        
        self.consume(&Token::Is, "Expected 'is' after type name")?;
        
        let definition = if self.match_token(&Token::TypePrefix) {
            // Alias type: #AliasType is #ExistingType
            if let Token::Identifier(target_type) = self.peek() {
                self.advance();
                TypeDefinition::Alias(Type::Primitive(target_type))
            } else {
                return Err("Expected type name after '#'".to_string());
            }
        } else if self.match_token(&Token::Record) {
            // Record type: #Person is record { name #Text, age #Number }
            self.consume(&Token::LeftBrace, "Expected '{' after 'record'")?;
            
            let mut fields = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                let field_name = if let Token::Identifier(name) = self.peek() {
                    self.advance();
                    name
                } else {
                    return Err("Expected field name".to_string());
                };
                
                let field_type = if self.match_token(&Token::TypePrefix) {
                    if let Token::Identifier(type_name) = self.peek() {
                        self.advance();
                        type_name
                    } else {
                        return Err("Expected field type after '#'".to_string());
                    }
                } else if self.match_any(&[
                    Token::NumberType, Token::TextType, Token::DecisionType, 
                    Token::NothingType, Token::BlissType, Token::AnyType
                ]) {
                    format!("{:?}", self.previous())
                } else {
                    return Err("Expected field type".to_string());
                };
                
                fields.push((field_name, field_type));
                
                if !self.check(&Token::RightBrace) {
                    self.consume(&Token::Comma, "Expected ',' between record fields")?;
                }
            }
            
            self.consume(&Token::RightBrace, "Expected '}' after record fields")?;
            
            TypeDefinition::Record { fields }
        } else if self.match_token(&Token::Variant) {
            // Variant type: #Shape is variant { Circle(radius #Number), Rectangle(width #Number, height #Number) }
            self.consume(&Token::LeftBrace, "Expected '{' after 'variant'")?;
            
            let mut variants = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                let variant_name = if let Token::Identifier(name) = self.peek() {
                    self.advance();
                    name
                } else {
                    return Err("Expected variant name".to_string());
                };
                
                let mut fields = Vec::new();
                if self.match_token(&Token::LeftParen) {
                    // Parse variant fields
                    if !self.check(&Token::RightParen) {
                        loop {
                            let field_name = if let Token::Identifier(name) = self.peek() {
                                self.advance();
                                name
                            } else {
                                return Err("Expected field name".to_string());
                            };
                            
                            let field_type = if self.match_token(&Token::TypePrefix) {
                                if let Token::Identifier(type_name) = self.peek() {
                                    self.advance();
                                    type_name
                                } else {
                                    return Err("Expected field type after '#'".to_string());
                                }
                            } else if self.match_any(&[
                                Token::NumberType, Token::TextType, Token::DecisionType, 
                                Token::NothingType, Token::BlissType, Token::AnyType
                            ]) {
                                format!("{:?}", self.previous())
                            } else {
                                return Err("Expected field type".to_string());
                            };
                            
                            fields.push((field_name, field_type));
                            
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                    }
                    
                    self.consume(&Token::RightParen, "Expected ')' after variant fields")?;
                }
                
                variants.push((variant_name, fields));
                
                if !self.check(&Token::RightBrace) {
                    self.consume(&Token::Comma, "Expected ',' between variant definitions")?;
                }
            }
            
            self.consume(&Token::RightBrace, "Expected '}' after variant definitions")?;
            
            TypeDefinition::Variant { variants }
        } else {
            return Err("Expected 'record', 'variant', or '#' after 'is' in type definition".to_string());
        };
        
        Ok(Stmt::TypeDefinition { name: type_name, definition })
    }
    
    fn import_declaration(&mut self) -> Result<Stmt, String> {
        // Parse import statements: "import Math" or "import { add, subtract } from Math"
        
        let mut specific_imports = Vec::new();
        
        if self.match_token(&Token::LeftBrace) {
            // Specific imports: import { add, subtract } from Math
            loop {
                if let Token::Identifier(name) = self.peek() {
                    self.advance();
                    specific_imports.push(name);
                } else {
                    return Err("Expected function name in import list".to_string());
                }
                
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
            
            self.consume(&Token::RightBrace, "Expected '}' after import list")?;
            self.consume(&Token::From, "Expected 'from' after import list")?;
        }
        
        // Parse module path
        let mut module_path = Vec::new();
        loop {
            if let Token::Identifier(name) = self.peek() {
                self.advance();
                module_path.push(name);
            } else {
                return Err("Expected module name".to_string());
            }
            
            if !self.match_token(&Token::Dot) {
                break;
            }
        }
        
        // Parse optional alias
        let alias = if self.match_token(&Token::As) {
            if let Token::Identifier(name) = self.peek() {
                self.advance();
                Some(name)
            } else {
                return Err("Expected alias name after 'as'".to_string());
            }
        } else {
            None
        };
        
        Ok(Stmt::Import {
            module_path,
            specific_imports,
            alias,
        })
    }
    
    fn skip_comment_line(&mut self) {
        while !self.is_at_end() && !self.check(&Token::RightBrace) {
            self.advance();
            if self.previous() == Token::ExclamationMark {
                break;
            }
        }
    }
    
    fn previous_token(&self) -> Token {
        if self.current > 0 {
            self.tokens[self.current - 2].clone()
        } else {
            Token::EOF
        }
    }
    
    fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        let index = self.current + offset;
        if index < self.tokens.len() {
            Some(&self.tokens[index])
        } else {
            None
        }
    }

    fn type_annotation(&mut self) -> Result<Stmt, String> {
        // This is a placeholder - implement according to your language grammar
        // For now, just parse it as a variable declaration
        self.variable_declaration(false)
    }
} 