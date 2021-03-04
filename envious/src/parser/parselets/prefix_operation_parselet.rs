use crate::{
    error::Error,
    lexer::token::Token,
    parser::{
        expression::{Expression, ExpressionKind, Unary, UnaryOperation},
        Parser,
    },
};

use super::prefix_parselet::PrefixParselet;

pub struct PrefixOperationParselet {
    precedence: usize,
    operation: UnaryOperation,
}

impl PrefixOperationParselet {
    pub fn new<T: Into<usize>>(precedence: T, operation: UnaryOperation) -> Self {
        Self {
            precedence: precedence.into(),
            operation,
        }
    }
}

impl<'a> PrefixParselet<'a> for PrefixOperationParselet {
    fn parse(
        &self,
        parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let operand = parser.parse_expression(self.precedence, token.0)?;
        let kind = ExpressionKind::Unary(Unary {
            operation: self.operation,
            expression: Box::new(operand),
        });

        Ok((token.0, kind))
    }
}
