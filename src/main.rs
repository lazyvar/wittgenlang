use wittgenlang::{Wittgenlang};
use std::io::{self, Write};
use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Wittgenlang::new();
    
    if args.len() > 1 {
        // Read from file
        let filename = &args[1];
        let contents = fs::read_to_string(filename)
            .map_err(|e| format!("Error reading file: {}", e))?;
        
        match interpreter.evaluate(&contents) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        // Interactive mode
        println!("Wittgenlang REPL (Ctrl+D to exit)");
        
        loop {
            print!("> ");
            io::stdout().flush().map_err(|e| format!("IO error: {}", e))?;
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break, // Ctrl+D pressed
                Ok(_) => {
                    match interpreter.evaluate(input.trim()) {
                        Ok(result) => println!("{}", result),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                },
                Err(e) => eprintln!("Error reading input: {}", e),
            }
        }
    }
    
    Ok(())
} 