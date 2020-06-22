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

    InfixBinaryOperation(Operation, Box<Expression>, Box<Expression>),
    LetExpression(String, Type, Box<Expression>),
    PrintExpression(Box<Expression>),
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
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}