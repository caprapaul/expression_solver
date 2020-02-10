use std::io::{Error, ErrorKind};
use std::sync::atomic::Ordering::AcqRel;
use crate::function::Function;
use crate::operator::Operator;
use crate::operand::Operand;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Operand(Operand),
    Operator(Operator)
}
