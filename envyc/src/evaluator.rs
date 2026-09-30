use std::fmt::{self, Display};

use crate::{
    ast::{
        binding::{InfixOperator, PrefixOperator},
        Ast, Expression, ExpressionIndex, Literal,
    },
    lex::token::buffer::{TokenIndex, TokenizedBuffer},
    source::{Source, Span},
};

pub struct Evaluator<'a> {
    source: &'a Source,
    buffer: &'a TokenizedBuffer,
    ast: &'a Ast,
}

impl<'a> Evaluator<'a> {
    pub fn new(source: &'a Source, buffer: &'a TokenizedBuffer, ast: &'a Ast) -> Self {
        Self {
            source,
            buffer,
            ast,
        }
    }

    pub fn evaluate(&self) -> Result<i64, ()> {
        self.evaluate_expression(self.ast.root())
    }

    fn evaluate_expression(&self, idx: ExpressionIndex) -> Result<i64, ()> {
        match self.ast.get(idx) {
            Expression::Literal(Literal::Integer(token)) => {
                let span = self.span_at(token);
                self.source.text(span).parse().map_err(|_| ())
            }
            Expression::UnaryOperation { operator, operand } => {
                let operand = self.evaluate_expression(*operand)?;
                match operator.0 {
                    PrefixOperator::Negate => operand.checked_neg().ok_or(()),
                }
            }
            Expression::BinaryOperation { lhs, operator, rhs } => {
                let lhs = self.evaluate_expression(*lhs)?;
                let rhs = self.evaluate_expression(*rhs)?;
                match operator.0 {
                    InfixOperator::Plus => lhs.checked_add(rhs).ok_or(()),
                    InfixOperator::Minus => lhs.checked_sub(rhs).ok_or(()),
                    InfixOperator::Multiply => lhs.checked_mul(rhs).ok_or(()),
                    InfixOperator::Divide => lhs.checked_div(rhs).ok_or(()),
                }
            }
            Expression::Error => Err(()),
        }
    }

    fn span_at(&self, token: &TokenIndex) -> Span {
        self.buffer
            .span_at(*token)
            .expect("token must exist in the tokenized buffer")
    }
}
