use crate::source::{Span, Spanned};

use super::expression::{Identifier, Type};

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub define_keyword: Span,
    pub name: Identifier,
    pub left_parenthesis: Span,
    pub parameters: Vec<Parameter>,
    pub right_parenthesis: Span,
    pub equal_sign: Span,
    // body: Expression,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: Identifier,
    pub colon: Span,
    pub ty: Spanned<Type>,
    pub trailing_comma: Option<Span>,
}