#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operand {
    Literal(f32),
    Variable(char)
}