use crate::{error::Span, semantic_analyzer::types::Type};

use super::typed_expression::TypedExpression;

#[derive(Debug)]
pub struct TypedProgram<'a> {
    pub functions: Vec<TypedFunction<'a>>,
}

impl<'a> TypedProgram<'a> {
    pub fn new(functions: Vec<TypedFunction<'a>>) -> Self {
        Self { functions }
    }
}

#[derive(Debug)]
pub struct TypedFunction<'a> {
    pub span: Span<'a>,
    pub name: usize,
    pub parameters: Vec<TypedParameter<'a>>,
    pub body: TypedExpression<'a>,
    pub return_type: Type,
}

impl<'a> TypedFunction<'a> {
    pub fn new(
        span: Span<'a>,
        name: usize,
        parameters: Vec<TypedParameter<'a>>,
        body: TypedExpression<'a>,
        return_type: Type,
    ) -> Self {
        Self {
            span,
            name,
            parameters,
            body,
            return_type,
        }
    }
}

#[derive(Debug)]
pub struct TypedParameter<'a> {
    pub span: Span<'a>,
    pub ty: Type,
    pub name: usize,
}

impl<'a> TypedParameter<'a> {
    pub fn new(span: Span<'a>, ty: Type, name: usize) -> Self {
        Self { span, ty, name }
    }
}
