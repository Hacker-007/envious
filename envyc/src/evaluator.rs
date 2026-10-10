use std::{
    convert::TryInto,
    num::{IntErrorKind, ParseIntError},
    ops::{BitAnd, BitOr, BitXor},
};

use crate::{
    ast::{
        binding::{InfixOperator, PrefixOperator},
        Ast, Expression, ExpressionIndex, Literal, Statement, StatementIndex,
    },
    source::{Source, Span},
    token::buffer::{TokenIndex, TokenizedBuffer},
};

#[derive(Debug)]
pub enum EvaluationError {
    InvalidInteger,
    IntegerUnderflow,
    IntegerOverflow,
    InvalidShift,
    Unknown,
}

impl From<ParseIntError> for EvaluationError {
    fn from(error: ParseIntError) -> Self {
        match *error.kind() {
            IntErrorKind::Empty => Self::InvalidInteger,
            IntErrorKind::InvalidDigit => Self::InvalidInteger,
            IntErrorKind::NegOverflow => Self::IntegerUnderflow,
            IntErrorKind::PosOverflow => Self::IntegerOverflow,
            IntErrorKind::Zero => unreachable!("parsing into `i64` implies zeros are always valid"),
            _ => Self::Unknown,
        }
    }
}

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

    pub fn evaluate(&mut self) -> Result<i64, EvaluationError> {
        let mut result = 0;
        for &statement in self.ast.roots() {
            result = self.evaluate_statement(statement)?;
        }

        Ok(result)
    }

    fn evaluate_statement(&mut self, idx: StatementIndex) -> Result<i64, EvaluationError> {
        match *self.ast.get_statement(idx) {
            Statement::Expression(idx) => self.evaluate_expression(idx),
            Statement::Let { value, .. } => {
                let value = self.evaluate_expression(value)?;
                println!("found let with value `{value}`");
                Ok(0)
            }
        }
    }

    fn evaluate_expression(&self, idx: ExpressionIndex) -> Result<i64, EvaluationError> {
        match self.ast.get_expression(idx) {
            Expression::Literal(Literal::Integer(token)) => {
                let span = self.span_at(token);
                self.source
                    .slice(span)
                    .parse()
                    .map_err(|_| EvaluationError::InvalidInteger)
            }
            Expression::UnaryOperation { operator, operand } => {
                let operand = self.evaluate_expression(*operand)?;
                match operator.0 {
                    PrefixOperator::Negate => Ok(-operand),
                    PrefixOperator::BitwiseComplement => Ok(!operand),
                }
            }
            Expression::BinaryOperation { lhs, operator, rhs } => {
                let lhs = self.evaluate_expression(*lhs)?;
                let rhs = self.evaluate_expression(*rhs)?;
                match operator.0 {
                    InfixOperator::Multiply => {
                        lhs.checked_mul(rhs).ok_or(EvaluationError::IntegerOverflow)
                    }
                    InfixOperator::Divide => {
                        lhs.checked_div(rhs).ok_or(EvaluationError::IntegerOverflow)
                    }
                    InfixOperator::Plus => {
                        lhs.checked_add(rhs).ok_or(EvaluationError::IntegerOverflow)
                    }
                    InfixOperator::Minus => lhs
                        .checked_sub(rhs)
                        .ok_or(EvaluationError::IntegerUnderflow),
                    InfixOperator::LeftShift => {
                        let shift = rhs.try_into().map_err(|_| EvaluationError::InvalidShift)?;
                        lhs.checked_shl(shift)
                            .ok_or(EvaluationError::IntegerOverflow)
                    }
                    InfixOperator::RightShift => {
                        let shift = rhs.try_into().map_err(|_| EvaluationError::InvalidShift)?;
                        lhs.checked_shr(shift)
                            .ok_or(EvaluationError::IntegerUnderflow)
                    }
                    InfixOperator::BitwiseAnd => Ok(lhs.bitand(rhs)),
                    InfixOperator::BitwiseXor => Ok(lhs.bitxor(rhs)),
                    InfixOperator::BitwiseOr => Ok(lhs.bitor(rhs)),
                }
            }
            Expression::Error => Ok(0),
        }
    }

    fn span_at(&self, token: &TokenIndex) -> Span {
        self.buffer
            .span_at(*token)
            .expect("token must exist in the tokenized buffer")
    }
}
