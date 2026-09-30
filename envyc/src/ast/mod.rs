use std::fmt::Display;

use crate::{
    ast::binding::{InfixOperator, PrefixOperator},
    dense::{DenseIndex, DenseVec},
    lex::token::buffer::TokenIndex,
};

pub mod binding;

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

    pub(crate) fn allocate_unary(
        &mut self,
        (operator, token): (PrefixOperator, TokenIndex),
        operand: ExpressionIndex,
    ) -> ExpressionIndex {
        let idx = self.expressions.push(Expression::UnaryOperation {
            operator: UnaryOperation(operator, token),
            operand,
        });

        ExpressionIndex(idx)
    }

    pub(crate) fn allocate_binary(
        &mut self,
        (operator, token): (InfixOperator, TokenIndex),
        lhs: ExpressionIndex,
        rhs: ExpressionIndex,
    ) -> ExpressionIndex {
        let idx = self.expressions.push(Expression::BinaryOperation {
            lhs,
            operator: BinaryOperation(operator, token),
            rhs,
        });

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

impl Ast {
    pub fn root(&self) -> ExpressionIndex {
        self.root
    }

    pub fn get(&self, expression: ExpressionIndex) -> &Expression {
        &self.expressions[expression.0]
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    UnaryOperation {
        operator: UnaryOperation,
        operand: ExpressionIndex,
    },
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
pub struct UnaryOperation(pub(crate) PrefixOperator, TokenIndex);

#[derive(Debug, Clone, Copy)]
pub struct BinaryOperation(pub(crate) InfixOperator, TokenIndex);
