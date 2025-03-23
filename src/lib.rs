mod utils;
mod lexer;
mod parser;
mod evaluator;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
use crate::parser::Parser;
use crate::evaluator::Interpreter;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Wittgenlang {
    interpreter: Interpreter,
}

// Implementation for all targets
impl Wittgenlang {
    pub fn new() -> Self {
        #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
        utils::set_panic_hook();
        
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn evaluate(&mut self, input: &str) -> Result<String, String> {
        let mut parser = Parser::new(input);
        let statements = parser.parse()?;
        let result = self.interpreter.interpret(statements)?;
        Ok(format!("{:?}", result))
    }
}

// Wasm-specific implementations
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[wasm_bindgen]
impl Wittgenlang {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new()
    }

    #[wasm_bindgen]
    pub fn evaluate_wasm(&mut self, input: &str) -> Result<String, String> {
        self.evaluate(input)
    }
}

// When the `wee_alloc`