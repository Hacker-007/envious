use crate::{
    error::Error,
    lexer::token::Token,
    parser::{
        expression::{Binary, BinaryOperation, Expression, ExpressionKind},
        Parser,
    },
};

use super::infix_parselet::InfixParselet;

pub struct BinaryOperationParselet {
    precedence: usize,
    operation: BinaryOperation,
    is_right_associative: bool,
}

impl BinaryOperationParselet {
    pub fn new<T: Into<usize>>(
        precedence: T,
        operation: BinaryOperation,
        is_right_associative: bool,
    ) -> Self {
        Self {
            precedence: precedence.into(),
            operation,
            is_right_associative,
        }
    }
}

impl<'a> InfixParselet<'a> for BinaryOperationParselet {
    fn parse(
        &self,
        parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        left: Expression<'a>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let right = parser.parse_expression(
            self.precedence - if self.is_right_associative { 1 } else { 0 },
            token.0,
        )?;
        let kind = ExpressionKind::Binary(Binary {
            operation: self.operation,
            left: Box::new(left),
            right: Box::new(right),
        });

        Ok((token.0, kind))
    }

    fn get_precedence(&self) -> usize {
        self.precedence
    }
}
