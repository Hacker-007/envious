use crate::{error::Span, semantic_analyzer::types::Type};

use super::expression::Expression;

#[derive(Debug)]
pub struct Program<'a> {
    pub extern_declarations: Vec<ExternDeclaration<'a>>,
    pub functions: Vec<Function<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(
        extern_declarations: Vec<ExternDeclaration<'a>>,
        functions: Vec<Function<'a>>,
    ) -> Self {
        Self {
            extern_declarations,
            functions,
        }
    }
}

#[derive(Debug)]
pub struct Prototype<'a> {
    pub span: Span<'a>,
    pub name: usize,
    pub parameters: Vec<Parameter<'a>>,
    pub return_type: (Type, Span<'a>),
}

#[derive(Debug)]
pub struct ExternDeclaration<'a> {
    pub span: Span<'a>,
    pub name: usize,
    pub parameters: Vec<(Type, Span<'a>)>,
    pub return_type: (Type, Span<'a>),
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
    pub name: usize,
    pub ty: Type,
}

impl<'a> Parameter<'a> {
    pub fn new(span: Span<'a>, name: usize, ty: Type) -> Self {
        Self { span, name, ty }
    }
}
