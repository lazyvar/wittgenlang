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

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
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
            Stmt::Function { name, return_type: _, params: _, body: _ } => {
                // Store a placeholder value for now
                // In a full implementation, we would store the function definition
                self.environment.define(name, Value::Nil);
                Ok(Value::Nil)
            }
            Stmt::Value { name, type_name: _, initializer, mutable: _ } => {
                let value = self.evaluate(initializer)?;
                self.environment.define(name, value.clone());
                Ok(value)
            }
            Stmt::Change { name, value } => {
                let evaluated_value = self.evaluate(value)?;
                self.environment.assign(&name, evaluated_value.clone())?;
                Ok(evaluated_value)
            }
            Stmt::Produce(value) => {
                if let Some(expr) = value {
                    self.evaluate(expr)
                } else {
                    Ok(Value::Nil)
                }
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate(condition)?;
                if self.is_truthy(condition_value) {
                    let mut result = Value::Nil;
                    for stmt in then_branch {
                        result = self.execute(stmt)?;
                    }
                    Ok(result)
                } else if let Some(else_stmts) = else_branch {
                    let mut result = Value::Nil;
                    for stmt in else_stmts {
                        result = self.execute(stmt)?;
                    }
                    Ok(result)
                } else {
                    Ok(Value::Nil)
                }
            }
            Stmt::Unless { condition, body } => {
                let condition_value = self.evaluate(condition)?;
                if !self.is_truthy(condition_value) {
                    let mut result = Value::Nil;
                    for stmt in body {
                        result = self.execute(stmt)?;
                    }
                    Ok(result)
                } else {
                    Ok(Value::Nil)
                }
            }
            Stmt::While { condition, body } => {
                let mut result = Value::Nil;
                let condition = condition.clone();
                let body = body.clone();
                
                // Evaluate condition first
                let mut cond_result = self.evaluate(condition.clone())?;
                
                while self.is_truthy(cond_result) {
                    for stmt in body.clone() {
                        result = self.execute(stmt)?;
                    }
                    // Re-evaluate condition after each loop iteration
                    cond_result = self.evaluate(condition.clone())?;
                }
                Ok(result)
            }
            Stmt::Write(expr) => {
                let value = self.evaluate(expr)?;
                println!("{:?}", value);
                Ok(value)
            }
            // Add placeholder implementations for other statement types
            _ => Ok(Value::Nil),
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
                    Token::Is => Ok(Value::Boolean(self.is_equal(left_value, right_value))),
                    Token::EqualEqual => Ok(Value::Boolean(self.is_equal(left_value, right_value))),
                    Token::NotEqual => Ok(Value::Boolean(!self.is_equal(left_value, right_value))),
                    Token::Greater => self.compare_greater(left_value, right_value),
                    Token::Less => self.compare_less(left_value, right_value),
                    Token::GreaterEqual => self.compare_greater_equal(left_value, right_value),
                    Token::LessEqual => self.compare_less_equal(left_value, right_value),
                    _ => Err("Invalid binary operator.".to_string()),
                }
            }
            Expr::Grouping(expr) => self.evaluate(*expr),
            Expr::Literal(literal) => Ok(self.literal_to_value(literal)),
            Expr::Unary { operator, right } => {
                let right_value = self.evaluate(*right)?;
                match operator {
                    Token::Minus => self.unary_minus(right_value),
                    Token::ExclamationMark => Ok(Value::Boolean(!self.is_truthy(right_value))),
                    _ => Err("Invalid unary operator.".to_string()),
                }
            }
            Expr::Variable(name) => {
                self.environment
                    .get(&name)
                    .ok_or_else(|| format!("Undefined variable '{}'.", name))
            }
            Expr::FunctionCall { name, arguments, named_arguments: _ } => {
                // Basic function call implementation - only handle built-in functions
                if name == "print" || name == "write" {
                    if let Some(arg) = arguments.get(0) {
                        let arg = arg.clone();
                        let value = self.evaluate(arg)?;
                        println!("{:?}", value);
                        Ok(value)
                    } else {
                        Ok(Value::Nil)
                    }
                } else {
                    Err(format!("Function '{}' not implemented", name))
                }
            }
            // Add placeholder implementations for other expression types
            _ => Ok(Value::Nil),
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

    fn compare_greater(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn compare_less(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn compare_greater_equal(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn compare_less_equal(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
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
            Literal::Integer(i) => Value::Number(i as f64),
            Literal::String(s) => Value::String(s),
            Literal::Decision(b) => Value::Boolean(b),
            Literal::Nothing => Value::Nil,
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