use std::fmt::{self, Display};

use crate::{
    ast::{Ast, Expression, ExpressionIndex, Literal},
    lex::token::buffer::{TokenIndex, TokenizedBuffer},
    source::{Source, Span},
};

pub struct PrettyPrinter<'a> {
    source: &'a Source,
    buffer: &'a TokenizedBuffer,
    ast: &'a Ast,
}

impl<'a> PrettyPrinter<'a> {
    pub fn new(source: &'a Source, buffer: &'a TokenizedBuffer, ast: &'a Ast) -> Self {
        Self {
            source,
            buffer,
            ast,
        }
    }

    fn fmt_expression(&self, idx: ExpressionIndex, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ast.expressions[idx.0] {
            Expression::Literal(Literal::Integer(token)) => {
                let span = self.span_at(token);
                write!(f, "{}", self.source.text(span))
            }
            Expression::BinaryOperation { lhs, operator, rhs } => {
                write!(f, "(")?;
                self.fmt_expression(*lhs, f)?;
                write!(f, " {} ", operator)?;
                self.fmt_expression(*rhs, f)?;
                write!(f, ")")
            }
            Expression::Error => write!(f, "<error>"),
        }
    }

    fn span_at(&self, token: &TokenIndex) -> Span {
        self.buffer
            .span_at(*token)
            .expect("token must exist in the tokenized buffer")
    }
}

impl<'a> Display for PrettyPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_expression(self.ast.root, f)
    }
}
