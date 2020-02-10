use crate::function::Function;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Arity {
    Unary,
    Binary
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Function(Function),
    Add(Arity),
    Substract(Arity),
    Multiply,
    Divide,
    LBracket,
    RBracket
}

impl Operator {
    pub fn from_str(buffer: &str) -> Result<Operator, Error> {
        match buffer {
            "+" => Ok(Operator::Add(Arity::Binary)),
            "-" => Ok(Operator::Substract(Arity::Binary)),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "(" => Ok(Operator::LBracket),
            ")" => Ok(Operator::RBracket),
            _ => Err(Error::new(ErrorKind::Other, format!("Invalid operator found: <{}>.", buffer)))
        }
    }
    
    pub fn precedence(&self) -> u8 {
        match *self {
            Operator::Add(Arity::Binary) => 3,
            Operator::Substract(Arity::Binary) => 3,
            Operator::Multiply => 4,
            Operator::Divide => 4,
            Operator::Function(_) => 5,
            Operator::Add(Arity::Unary) => 6,
            Operator::Substract(Arity::Unary) => 6,
            Operator::LBracket => 0,
            Operator::RBracket => 0,
        }
    }
}
