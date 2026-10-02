use std::num::{IntErrorKind, ParseIntError};

use crate::{
    ast::{
        binding::{InfixOperator, PrefixOperator},
        Ast, Expression, ExpressionIndex, Literal,
    },
    lex::token::buffer::{TokenIndex, TokenizedBuffer},
    source::{Source, Span},
};

#[derive(Debug)]
pub enum EvaluationError {
    InvalidInteger,
    IntegerOverflow,
    Unknown,
}

impl From<ParseIntError> for EvaluationError {
    fn from(error: ParseIntError) -> Self {
        match *error.kind() {
            IntErrorKind::Empty => Self::InvalidInteger,
            IntErrorKind::InvalidDigit => Self::InvalidInteger,
            IntErrorKind::PosOverflow => Self::IntegerOverflow,
            IntErrorKind::NegOverflow => Self::IntegerOverflow,
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

    pub fn evaluate(&self) -> Result<i64, EvaluationError> {
        self.evaluate_expression(self.ast.root())
    }

    fn evaluate_expression(&self, idx: ExpressionIndex) -> Result<i64, EvaluationError> {
        match self.ast.get(idx) {
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
                }
            }
            Expression::BinaryOperation { lhs, operator, rhs } => {
                let lhs = self.evaluate_expression(*lhs)?;
                let rhs = self.evaluate_expression(*rhs)?;
                match operator.0 {
                    InfixOperator::Plus => {
                        lhs.checked_add(rhs).ok_or(EvaluationError::IntegerOverflow)
                    }
                    InfixOperator::Minus => {
                        lhs.checked_sub(rhs).ok_or(EvaluationError::IntegerOverflow)
                    }
                    InfixOperator::Multiply => {
                        lhs.checked_mul(rhs).ok_or(EvaluationError::IntegerOverflow)
                    }
                    InfixOperator::Divide => {
                        lhs.checked_div(rhs).ok_or(EvaluationError::IntegerOverflow)
                    }
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
