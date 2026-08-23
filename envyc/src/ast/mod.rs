use std::fmt::Display;

use crate::{
    ast::binding::InfixOperator,
    dense::{DenseIndex, DenseVec},
    lex::token::buffer::TokenIndex,
};

pub mod binding;
pub mod printer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpressionIndex(DenseIndex);

#[derive(Debug, Default)]
pub(crate) struct AstBuilder {
    expressions: DenseVec<Expression>,
}

impl AstBuilder {
    pub(crate) fn allocate_literal(&mut self, literal: Literal) -> ExpressionIndex {
        let idx = self.expressions.push(Expression::Literal(literal));
        ExpressionIndex(idx)
    }

    pub(crate) fn allocate_binary(
        &mut self,
        (operator, token): (InfixOperator, TokenIndex),
        lhs: ExpressionIndex,
        rhs: ExpressionIndex,
    ) -> ExpressionIndex {
        let operator = match operator {
            InfixOperator::Plus => BinaryOperation::Plus(token),
        };

        let idx = self
            .expressions
            .push(Expression::BinaryOperation { lhs, operator, rhs });
        ExpressionIndex(idx)
    }

    pub(crate) fn allocate_error(&mut self) -> ExpressionIndex {
        let idx = self.expressions.push(Expression::Error);
        ExpressionIndex(idx)
    }

    pub(crate) fn finish(self, root: ExpressionIndex) -> Ast {
        Ast {
            root,
            expressions: self.expressions,
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    root: ExpressionIndex,
    expressions: DenseVec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    BinaryOperation {
        lhs: ExpressionIndex,
        operator: BinaryOperation,
        rhs: ExpressionIndex,
    },
    Error,
}

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    Integer(TokenIndex),
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperation {
    Plus(TokenIndex),
}

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperation::Plus(_) => write!(f, "+"),
        }
    }
}
