use crate::parser::{Expr, Literal, Stmt};
use std::collections::HashMap;
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.values.get(name).cloned()
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<Value, String> {
        let mut last_value = Value::Nil;
        for statement in statements {
            last_value = self.execute(statement)?;
        }
        Ok(last_value)
    }

    fn execute(&mut self, stmt: Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::Function { name, params: _, body: _ } => {
                // Store a placeholder value for now
                // In a full implementation, we would store the function definition
                self.environment.define(name, Value::Nil);
                Ok(Value::Nil)
            }
            Stmt::Return(value) => {
                if let Some(expr) = value {
                    self.evaluate(expr)
                } else {
                    Ok(Value::Nil)
                }
            }
            Stmt::Var { name, initializer } => {
                let value = if let Some(expr) = initializer {
                    self.evaluate(expr)?
                } else {
                    Value::Nil
                };
                self.environment.define(name, value.clone());
                Ok(value)
            }
        }
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left_value = self.evaluate(*left)?;
                let right_value = self.evaluate(*right)?;
                match operator {
                    Token::Plus => self.binary_plus(left_value, right_value),
                    Token::Minus => self.binary_minus(left_value, right_value),
                    Token::Star => self.binary_multiply(left_value, right_value),
                    Token::Slash => self.binary_divide(left_value, right_value),
                    Token::Equal => Ok(Value::Boolean(self.is_equal(left_value, right_value))),
                    _ => Err("Invalid binary operator.".to_string()),
                }
            }
            Expr::Grouping(expr) => self.evaluate(*expr),
            Expr::Literal(literal) => Ok(self.literal_to_value(literal)),
            Expr::Unary { operator, right } => {
                let right_value = self.evaluate(*right)?;
                match operator {
                    Token::Minus => self.unary_minus(right_value),
                    Token::Bang => Ok(Value::Boolean(!self.is_truthy(right_value))),
                    _ => Err("Invalid unary operator.".to_string()),
                }
            }
            Expr::Variable(name) => {
                self.environment
                    .get(&name)
                    .ok_or_else(|| format!("Undefined variable '{}'.", name))
            }
        }
    }

    fn binary_plus(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
            _ => Err("Operands must be two numbers or two strings.".to_string()),
        }
    }

    fn binary_minus(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn binary_multiply(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn binary_divide(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                if r == 0.0 {
                    Err("Division by zero.".to_string())
                } else {
                    Ok(Value::Number(l / r))
                }
            }
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn unary_minus(&self, value: Value) -> Result<Value, String> {
        match value {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err("Operand must be a number.".to_string()),
        }
    }

    fn literal_to_value(&self, literal: Literal) -> Value {
        match literal {
            Literal::Number(n) => Value::Number(n),
            Literal::String(s) => Value::String(s),
            Literal::Boolean(b) => Value::Boolean(b),
            Literal::Nil => Value::Nil,
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Boolean(b) => b,
            Value::Nil => false,
            _ => true,
        }
    }

    fn is_equal(&self, left: Value, right: Value) -> bool {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Boolean(l), Value::Boolean(r)) => l == r,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
} 