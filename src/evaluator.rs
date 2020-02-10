use crate::tokenizer::tokenize;
use crate::token::Token;
use std::io::{Error, ErrorKind};
use crate::operator::{Operator, Arity};
use crate::function::Function;
use crate::operand::Operand;

#[cfg(test)]
mod tests {
    use crate::tokenizer;
    use crate::evaluator::{infix_to_rpn, evaluate};
    
    #[test]
    fn test_infix_to_rpn() {
        let expression = String::from("2*(sin(2)*-22)");
        let tokens = tokenizer::tokenize(&expression).unwrap();
        let tokens = infix_to_rpn(tokens).unwrap();
        
        assert_eq!("[Operand(Literal(2.0)), Operand(Literal(2.0)), Operator(Function(Sin)), Operand(Literal(22.0)), Operator(Substract(Unary)), Operator(Multiply), Operator(Multiply)]",
                   format!("{:?}", tokens));
        
    }
    
    #[test]
    fn test_evaluate() {
        let mut operations = Vec::new();
        assert_eq!(-40.009087, evaluate("2*(sin(2)*-22)", &mut operations).unwrap());
        assert_eq!(-40.0, evaluate("2* (2-22)", &mut operations).unwrap());
        assert_eq!(0.0, evaluate("2*(123 + 22 / 4 + 1) * 0", &mut operations).unwrap());
        assert_eq!(5.0, evaluate("2+1*3", &mut operations).unwrap());
    }
}



fn infix_to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, Error>{
    let mut operators = Vec::new();
    let mut rpn = Vec::new();
    
    for token in tokens {
        match token {
            Token::Operand(_) => rpn.push(token),
            Token::Operator(Operator::LBracket) => operators.push(Operator::LBracket),
            Token::Operator(Operator::RBracket) => {
                let mut popped = Token::Operator(Operator::RBracket);
                while popped != Token::Operator(Operator::LBracket) {
                    popped = match operators.pop() {
                        None => return Err(Error::new(ErrorKind::Other, format!("Invalid expression: too many closing brackets."))),
                        Some(x) => Token::Operator(x),
                    };
                    
                    if popped != Token::Operator(Operator::LBracket) {
                        rpn.push(popped);
                    }
                }
            },
            Token::Operator(operator) => {
                let mut popped = Token::Operator(Operator::RBracket);
                while !operators.is_empty() && (*operators.last().unwrap()).precedence() >= operator.precedence() {
                    popped = match operators.pop() {
                        None => return Err(Error::new(ErrorKind::Other, format!("Invalid expression."))),
                        Some(x) => Token::Operator(x),
                    };
                    
                    rpn.push(popped);
                }
                operators.push(operator);
            }
        }
    }
    
    while !operators.is_empty() {
        let popped = match operators.pop() {
            None => return Err(Error::new(ErrorKind::Other, format!("Invalid expression."))),
            Some(x) => Token::Operator(x),
        };
        
        if let Token::Operator(Operator::LBracket) = popped {
            return Err(Error::new(ErrorKind::Other, format!("Invalid expression: too many opening brackets.")));
        }
    
        rpn.push(popped);
    }
    
    return Ok(rpn);
}

fn calculate(left: f32, right: f32, operator: Operator, operations: &mut Vec<String>) -> f32 {
    match operator {
        Operator::Function(function) => match function {
            Function::Sin => {
                let res = right.sin();
                operations.push(format!("sin({}) = {}", right, res));
                res
            },
            Function::Cos => {
                let res = right.cos();
                operations.push(format!("cos({}) = {}", right, res));
                res
            },
        },
        Operator::Add(arity) => {
            let res = left + right;
            
            if let Arity::Binary = arity {
                operations.push(format!("{} + {} = {}", left, right, res));
            }
    
    
            res
        },
        Operator::Substract(arity) => {
            let res = left - right;
    
            if let Arity::Binary = arity {
                operations.push(format!("{} - {} = {}", left, right, res));
            }
            
            res
        },
        Operator::Multiply => {
            let res = left * right;
            
            operations.push(format!("{} * {} = {}", left, right, res));
    
            res
        },
        Operator::Divide => {
            let res = left / right;
    
            operations.push(format!("{} / {} = {}", left, right, res));
    
            res
        },
        _ => 0.0
    }
}

pub fn evaluate(expression: &str, operations: &mut Vec<String>) -> Result<f32, Error> {
    let tokens = tokenize(expression)?;
    let tokens = infix_to_rpn(tokens)?;
    let mut values = Vec::new();
    
    for token in tokens {
        match token {
            Token::Operator(operator) => {
                let right = match values.pop() {
                    None => return Err(Error::new(ErrorKind::Other, format!("Invalid expression: not enough operands."))),
                    Some(x) => x,
                };
                
                let left: f32 = match operator {
                    Operator::Function(_) => 0.0,
                    Operator::Add(Arity::Unary) =>  0.0,
                    Operator::Substract(Arity::Unary) => 0.0,
                    _ => match values.pop() {
                        None => return Err(Error::new(ErrorKind::Other, format!("Invalid expression: not enough operands."))),
                        Some(x) => x,
                    }
                };
                
                values.push(calculate(left, right, operator, operations));
            },
            Token::Operand(Operand::Literal(value)) => values.push(value),
            _ => return Err(Error::new(ErrorKind::Other, format!("Variables not implemented.")))
        }
    }
    
    if values.len() > 1 {
        return Err(Error::new(ErrorKind::Other, format!("Invalid expression: too many opening brackets?")));
    }
    
    let result = match values.pop() {
        None => return Err(Error::new(ErrorKind::Other, format!("Invalid expression: not enough operands."))),
        Some(x) => x,
    };
    
    return Ok(result);
}
