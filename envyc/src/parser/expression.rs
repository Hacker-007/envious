use crate::{error::Span, semantic_analyzer::types::Type};

/// Represents an expression that is generated by the `Parser`.
/// Each expression consists of a span (the location information of the expression)
/// and the kind of the expression.
pub type Expression<'a> = (Span<'a>, ExpressionKind<'a>);

/// Enum that details the different types of expressions that can be produced
/// by the `Expression`. The `ExpressionKind` should strive to only store types that
/// are small in nature and any other types (i.e. String) should be stored in the
/// `Interner`.
#[derive(Debug)]
pub enum ExpressionKind<'a> {
    Int(i64),
    Float(f64),
    Boolean(bool),
    // The actual value for both the `String` and the `Identifier` are
    // stored in the `Interner` to reduce redundency in values. Instead,
    // the id's are stored in the variant.
    String(usize),
    Identifier(Identifier),
    Unary(Unary<'a>),
    Binary(Binary<'a>),
    If(If<'a>),
    Let(Let<'a>),
    Block(Vec<Expression<'a>>),
}

#[derive(Debug)]
pub struct Identifier(pub usize);

#[derive(Debug)]
pub struct Unary<'a> {
    pub operation: UnaryOperation,
    pub expression: Box<Expression<'a>>,
}

#[derive(Debug)]
pub struct Binary<'a> {
    pub operation: BinaryOperation,
    pub left: Box<Expression<'a>>,
    pub right: Box<Expression<'a>>,
}

#[derive(Debug)]
pub struct If<'a> {
    pub condition: Box<Expression<'a>>,
    pub then_branch: Box<Expression<'a>>,
    pub else_branch: Option<Box<Expression<'a>>>,
}

#[derive(Debug)]
pub struct Let<'a> {
    pub name: (Span<'a>, Identifier),
    pub given_type: Option<Type>,
    pub expression: Box<Expression<'a>>,
}

/// Enum that details the different unary operations
/// that can be applied to any expression.
/// Note that this enum should not contain any subexpressions.
/// It should exist to only describe the operations possible.
#[derive(Debug, Clone, Copy)]
pub enum UnaryOperation {
    Plus,
    Minus,
    Not,
}

/// Enum that details the different binary operations
/// that can be applied to any expression.
/// Note that this enum should not contain any subexpressions.
/// It should exist to only describe the operations possible.
#[derive(Debug, Clone, Copy)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
}