use std::io::{Error, ErrorKind};
use crate::token::Token;
use crate::operator::{Operator, Arity};
use crate::operand::Operand;
use crate::function::Function;

#[cfg(test)]
mod tests {
    use crate::tokenizer;
    
    #[test]
    fn test_tokenizer() {
        let expression = String::from("2*(sin(2)*-22)");
        let tokens = tokenizer::tokenize(&expression).unwrap();
        
        assert_eq!("[Operand(Literal(2.0)), Operator(Multiply), Operator(LBracket), Operator(Function(Sin)), Operator(LBracket), Operand(Literal(2.0)), Operator(RBracket), Operator(Multiply), Operator(Substract(Unary)), Operand(Literal(22.0)), Operator(RBracket)]",
                   format!("{:?}", tokens));
    }
    
}

#[derive(PartialEq)]
enum Character {
    None,
    Letter,
    Number,
    Operator,
    LBracket,
    RBracket,
}

fn get_char_type(c: char) -> Result<Character, Error> {
    match c {
        c if c.is_digit(10) || ".".contains(c) => Ok(Character::Number),
        c if c.is_alphabetic() => Ok(Character::Letter),
        c if "+-*/".contains(c) => Ok(Character::Operator),
        '(' => Ok(Character::LBracket),
        ')' => Ok(Character::RBracket),
        _ => Err(Error::new(ErrorKind::Other, format!("Invalid character found: <{}>.", c)))
    }
}

fn letter_buffer_to_tokens(buffer: &str, char_type: &Character) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    
    match char_type {
        Character::LBracket => {
            let token = Token::Operator(Operator::Function(Function::from_str(buffer)?));
            tokens.push(token);
        }
        _ => {
            for c in buffer.chars() {
                let token = Token::Operand(Operand::Variable(c));
                tokens.push(token);
            }
        }
    }
    return Ok(tokens);
}

fn number_buffer_to_token(buffer: &str) -> Token {
    let num: f32 = buffer.parse()
        .unwrap();
    return Token::Operand(Operand::Literal(num));
}

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

pub fn tokenize(expression: &str) -> Result<Vec<Token>, Error> {
    let expression = remove_whitespace(expression);
    let mut tokens: Vec<Token> = Vec::new();
    let mut letter_buffer = String::new();
    let mut number_buffer = String::new();
    let mut last_char_type = Character::None;
    
    let mut i: usize = 0;
    let mut c: char = '0';
    
    for (i, c) in expression.chars().enumerate() {
        let char_type = get_char_type(c)?;
        
        match (&last_char_type, &char_type) {
            (_, Character::Letter) => {
                letter_buffer = format!("{}{}", letter_buffer, c)
            },
            (Character::Letter, _) => {
                let letter_tokens = letter_buffer_to_tokens(&letter_buffer, &char_type)?;
                for token in letter_tokens {
                    tokens.push(token);
                }
                letter_buffer = String::new();
            },
            (_, Character::Number) => {
                number_buffer = format!("{}{}", number_buffer, c)
            },
            (Character::Number, _) => {
                tokens.push(number_buffer_to_token(&number_buffer));
                number_buffer = String::new();
            },
            (_, _) => {}
        }
        
        match &char_type {
            Character::Operator => {
                let mut token = Token::Operator(Operator::from_str(&(c.to_string()))?);
    
                // Handle unary operators
                if let Character::None | Character::LBracket | Character::Operator = last_char_type {
                    match token {
                        Token::Operator(Operator::Add(_)) => {
                            token = Token::Operator(Operator::Add(Arity::Unary));
                        },
                        Token::Operator(Operator::Substract(_)) => {
                            token = Token::Operator(Operator::Substract(Arity::Unary));
                        },
                        _ => return Err(Error::new(ErrorKind::Other, format!("Invalid operator usage on position <{}>.", i)))
                    }
                }
    
                tokens.push(token);
            },
            Character::LBracket => {
                let token = Token::Operator(Operator::LBracket);
                tokens.push(token);
            },
            Character::RBracket => {
                let token = Token::Operator(Operator::RBracket);
                tokens.push(token);
            }
            _ => {}
        }
        
        last_char_type = char_type;
    }
    
    match &last_char_type {
        Character::Letter=> {
            let letter_tokens = letter_buffer_to_tokens(&letter_buffer, &Character::None)?;
            for token in letter_tokens {
                tokens.push(token);
            }
            letter_buffer = String::new();
        },
        Character::Number => {
            tokens.push(number_buffer_to_token(&number_buffer));
            number_buffer = String::new();
        },
        Character::RBracket => {},
        _ => return Err(Error::new(ErrorKind::Other, format!("Invalid character <{}> at position <{}>.", c, i)))
    }
    
    return Ok(tokens);
}
