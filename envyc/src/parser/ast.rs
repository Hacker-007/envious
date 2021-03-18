use crate::{error::Span, semantic_analyzer::types::Type};

use super::expression::Expression;

#[derive(Debug)]
pub struct Program<'a> {
    pub functions: Vec<Function<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(functions: Vec<Function<'a>>) -> Self {
        Self { functions }
    }
}

#[derive(Debug)]
pub struct Prototype<'a> {
    pub span: Span<'a>,
    pub name: usize,
    pub parameters: Vec<Parameter<'a>>,
    pub return_type: Option<Type>,
}

#[derive(Debug)]
pub struct Function<'a> {
    pub prototype: Prototype<'a>,
    pub body: Expression<'a>,
}

impl<'a> Function<'a> {
    pub fn new(prototype: Prototype<'a>, body: Expression<'a>) -> Self {
        Self { prototype, body }
    }
}

#[derive(Debug)]
pub struct Parameter<'a> {
    pub span: Span<'a>,
    pub ty: Type,
    pub name: usize,
}

impl<'a> Parameter<'a> {
    pub fn new(span: Span<'a>, ty: Type, name: usize) -> Self {
        Self { span, ty, name }
    }
}
