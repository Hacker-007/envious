use crate::error::Span;

use super::{monotype::MonotypeRef, typed_expression::TypedExpression};

#[derive(Debug)]
pub struct TypedProgram<'a> {
    pub extern_declarations: Vec<TypedExternDeclaration<'a>>,
    pub functions: Vec<TypedFunction<'a>>,
}

impl<'a> TypedProgram<'a> {
    pub fn new(
        extern_declarations: Vec<TypedExternDeclaration<'a>>,
        functions: Vec<TypedFunction<'a>>,
    ) -> Self {
        Self {
            extern_declarations,
            functions,
        }
    }
}

#[derive(Debug)]
pub struct TypedPrototype<'a> {
    pub span: Span<'a>,
    pub name: usize,
    pub parameters: Vec<TypedParameter<'a>>,
    pub return_type: MonotypeRef,
}

impl<'a> TypedPrototype<'a> {
    pub fn new(
        span: Span<'a>,
        name: usize,
        parameters: Vec<TypedParameter<'a>>,
        return_type: MonotypeRef,
    ) -> Self {
        Self {
            span,
            name,
            parameters,
            return_type,
        }
    }
}

#[derive(Debug)]
pub struct TypedExternDeclaration<'a> {
    pub span: Span<'a>,
    pub name: usize,
    pub parameters: Vec<(MonotypeRef, Span<'a>)>,
    pub return_type: (MonotypeRef, Span<'a>),
}

#[derive(Debug)]
pub struct TypedFunction<'a> {
    pub prototype: TypedPrototype<'a>,
    pub body: TypedExpression<'a>,
}

impl<'a> TypedFunction<'a> {
    pub fn new(prototype: TypedPrototype<'a>, body: TypedExpression<'a>) -> Self {
        Self { prototype, body }
    }
}

#[derive(Debug)]
pub struct TypedParameter<'a> {
    pub span: Span<'a>,
    pub ty: MonotypeRef,
    pub name: usize,
}

impl<'a> TypedParameter<'a> {
    pub fn new(span: Span<'a>, ty: MonotypeRef, name: usize) -> Self {
        Self { span, ty, name }
    }
}
