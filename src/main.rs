use std::io;
use std::io::Error;

pub mod token;
pub mod tokenizer;
pub mod evaluator;
pub mod function;
pub mod operator;
pub mod operand;

fn main() {
    let mut expression = String::new();
    let mut operations: Vec<String> = Vec::new();
    println!("Input expression: ");
    
    io::stdin().read_line(&mut expression)
        .expect("Failed to read line");
    
    let result = match evaluator::evaluate(&expression, &mut operations) {
        Ok(value) => value,
        Err(e) => {
            println!("{:?}", e);
            return;
        },
    };
    
    for (i, operation) in operations.iter().enumerate() {
        println!("Step {}: {}", i+1, operation);
    }
    
    println!("Final result: {}", result);
}
