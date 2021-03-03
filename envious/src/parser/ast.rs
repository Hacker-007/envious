use crate::{error::Span, semantic_analyzer::types::Type};

use super::expression::Expression;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new(functions: Vec<Function>) -> Self {
        Self { functions }
    }
}

#[derive(Debug)]
pub struct Function {
    pub span: Span,
    pub name: usize,
    pub parameters: Vec<Parameter>,
    pub body: Expression,
    pub return_type: Option<Type>,
}

impl Function {
    pub fn new(span: Span, name: usize, parameters: Vec<Parameter>, body: Expression) -> Self {
        Self {
            span,
            name,
            parameters,
            body,
            return_type: None,
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub span: Span,
    pub ty: Type,
    pub name: usize,
}

impl Parameter {
    pub fn new(span: Span, ty: Type, name: usize) -> Self {
        Self { span, ty, name }
    }
}
