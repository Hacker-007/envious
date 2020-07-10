//! The ExpressionKind enum maintains all of the different Expressions that could occur within the program.
//! Using an enum allows for easy extensibility.

use super::expression::Expression;

#[derive(Debug)]
pub enum ExpressionKind {
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Identifier(String),

    InfixBinaryExpression(BinaryOperation, Box<Expression>, Box<Expression>),
    UnaryExpression(UnaryOperation, Box<Expression>),
    BinaryEqualityExpression(BinaryEqualityOperation, Box<Expression>, Box<Expression>),
    LetExpression(String, Type, Option<Box<Expression>>),
    FunctionCallExpression(String, Vec<Expression>),
    BlockExpression(Vec<Expression>),
    IfExpression(Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum Type {
    Unknown,

    Int,
    Float,
    Boolean,
    String,
}

#[derive(Debug)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum UnaryOperation {
    Positive,
    Negative,
}

#[derive(Debug)]
pub enum BinaryEqualityOperation {
    Equals,
}
