use crate::{context::symbol_interner::SymbolId, source::Spanned};

use super::ptr::P;

#[derive(Debug)]
pub enum Type {
    Int,
    Boolean,
}

pub type Identifier = Spanned<SymbolId>;

#[derive(Debug)]
pub enum ExpressionKind {
    Int(i64),
    Boolean(bool),
    Identifier(Identifier),
    Binary(Binary),
}

#[derive(Debug)]
pub struct Binary {
    _left: P<Spanned<ExpressionKind>>,
    _operation: Spanned<BinaryOperation>,
    _right: P<Spanned<ExpressionKind>>,
}

#[derive(Debug)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
}
