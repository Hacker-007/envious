use crate::{
    ast::binding::{InfixOperator, PrefixOperator},
    dense::{DenseIndex, DenseVec},
    token::buffer::TokenIndex,
};

pub mod binding;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatementIndex(DenseIndex);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpressionIndex(DenseIndex);

#[derive(Debug, Default)]
pub(crate) struct AstBuilder {
    statements: DenseVec<Statement>,
    expressions: DenseVec<Expression>,
}

impl AstBuilder {
    pub(crate) fn allocate_expression(&mut self, expression: ExpressionIndex) -> StatementIndex {
        let idx = self.statements.push(Statement::Expression(expression));
        StatementIndex(idx)
    }

    pub(crate) fn allocate_let(
        &mut self,
        identifier: TokenIndex,
        value: ExpressionIndex,
    ) -> StatementIndex {
        let idx = self.statements.push(Statement::Let { identifier, value });
        StatementIndex(idx)
    }

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

    pub(crate) fn finish(self, roots: Vec<StatementIndex>) -> Ast {
        Ast {
            roots,
            statements: self.statements,
            expressions: self.expressions,
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    roots: Vec<StatementIndex>,
    statements: DenseVec<Statement>,
    expressions: DenseVec<Expression>,
}

impl Ast {
    pub fn roots(&self) -> &[StatementIndex] {
        &self.roots
    }

    pub fn get_statement(&self, statement: StatementIndex) -> &Statement {
        &self.statements[statement.0]
    }

    pub fn get_expression(&self, expression: ExpressionIndex) -> &Expression {
        &self.expressions[expression.0]
    }
}

#[derive(Debug)]
pub enum Statement {
    Expression(ExpressionIndex),
    Let {
        // TODO: update this to be an interned ID.
        identifier: TokenIndex,
        value: ExpressionIndex,
    },
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
