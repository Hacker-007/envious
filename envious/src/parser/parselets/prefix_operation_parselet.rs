use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{Expression, ExpressionKind, UnaryOperation},
        Parser,
    },
};

use super::prefix_parselet::PrefixParselet;

pub struct PrefixOperationParselet {
    precedence: usize,
}

impl PrefixOperationParselet {
    pub fn new<T: Into<usize>>(precedence: T) -> Self {
        Self {
            precedence: precedence.into(),
        }
    }
}

impl PrefixParselet for PrefixOperationParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expression, Error> {
        let operand = parser.parse_expression(self.precedence, &token.0)?;
        let kind = match &token.1 {
            TokenKind::Plus => return Ok(operand),
            TokenKind::Minus => ExpressionKind::Unary {
                operation: UnaryOperation::Minus,
                expression: Box::new(operand),
            },
            TokenKind::Not => ExpressionKind::Unary {
                operation: UnaryOperation::Not,
                expression: Box::new(operand),
            },
            _ => unreachable!(),
        };

        Ok((token.0, kind))
    }
}
