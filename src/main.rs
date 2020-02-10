use std::io;

pub mod token;
pub mod tokenizer;
pub mod evaluator;
pub mod function;
pub mod operator;
pub mod operand;

fn main() {
    let mut expression = String::new();
    println!("Input expression: ");
    
    io::stdin().read_line(&mut expression)
        .expect("Failed to read line");
    
    let result = evaluator::evaluate(&expression)
        .unwrap();
    
    println!("Result: {}", result);
}
