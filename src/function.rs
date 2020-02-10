use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Function {
    Sin,
    Cos
}

impl Function {
    pub fn from_str(buffer: &str) -> Result<Function, Error> {
        match buffer {
            "sin" => Ok(Function::Sin),
            "cosin" => Ok(Function::Cos),
            _ => Err(Error::new(ErrorKind::Other, format!("Invalid function found: <{}>.", buffer)))
        }
    }
}
