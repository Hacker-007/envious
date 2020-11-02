//! The ExpressionKind enum maintains all of the different Expressions that could occur within the program.
//! Using an enum allows for easy extensibility.

use super::expression::Expression;
use crate::semantic_analyzer::types::Types;

#[derive(Debug)]
pub enum ExpressionKind {
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Identifier(String, Option<Types>),

    ParenthesizedExpression(Box<Expression>),
    InfixBinaryExpression(BinaryOperation, Box<Expression>, Box<Expression>),
    UnaryExpression(UnaryOperation, Box<Expression>),
    BinaryEqualityExpression(BinaryEqualityOperation, Box<Expression>, Box<Expression>),
    LetExpression(String, Types, Option<Box<Expression>>),
    FunctionCallExpression(String, Vec<Expression>),
    BlockExpression(Vec<Expression>),
    IfExpression(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
    DefineExpression(String, Vec<Parameter>, Box<Expression>, Types)
}

#[derive(Debug)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,

    Or,
    And,
}

#[derive(Debug)]
pub enum UnaryOperation {
    Positive,
    Negative,
    Not,
}

#[derive(Debug)]
pub enum BinaryEqualityOperation {
    Equals,
    NotEquals,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

#[derive(Debug)]
pub struct Parameter {
    pub pos: usize,
    pub name: String,
    pub expected_type: Types,
}

impl Parameter {
    pub fn new(pos: usize, name: String, expected_type: Types) -> Parameter {
        Parameter {
            pos,
            name,
            expected_type,
        }
    }
}