use crate::{error::Span, semantic_analyzer::types::Type};

use super::expression::Expression;

#[derive(Debug)]
pub struct Program {
    functions: Vec<Function>,
}

impl Program {
    pub fn new(functions: Vec<Function>) -> Self {
        Self { functions }
    }
}

#[derive(Debug)]
pub struct Function {
    span: Span,
    name: usize,
    parameters: Vec<Parameter>,
    body: Expression,
}

impl Function {
    pub fn new(span: Span, name: usize, parameters: Vec<Parameter>, body: Expression) -> Self {
        Self {
            span,
            name,
            parameters,
            body,
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub span: Span,
    ty: Type,
    name: usize,
}

impl Parameter {
    pub fn new(span: Span, ty: Type, name: usize) -> Self {
        Self { span, ty, name }
    }
}
