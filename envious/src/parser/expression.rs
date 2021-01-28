use crate::span::Span;

pub type Expression = (Span, ExpressionKind);

#[derive(Debug)]
pub enum ExpressionKind {
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(usize),
    Identifier(usize),
    Unary {
        operation: UnaryOperation,
        expression: Box<Expression>,
    },
    Binary {
        operation: BinaryOperation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },
}

#[derive(Debug)]
pub enum UnaryOperation {
    Minus,
    Not,
}

#[derive(Debug)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
}
