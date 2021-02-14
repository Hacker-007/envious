use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{BinaryOperation, Expression, ExpressionKind},
        Parser,
    },
};

use super::infix_parselet::InfixParselet;

pub struct BinaryOperationParselet {
    precedence: usize,
    is_right_associative: bool,
}

impl BinaryOperationParselet {
    pub fn new<T: Into<usize>>(precedence: T, is_right_associative: bool) -> Self {
        Self {
            precedence: precedence.into(),
            is_right_associative,
        }
    }
}

impl InfixParselet for BinaryOperationParselet {
    fn parse(
        &self,
        parser: &mut Parser<impl Iterator<Item = Token>>,
        left: Expression,
        token: Token,
    ) -> Result<Expression, Error> {
        let right = parser.parse_expression(
            self.precedence - if self.is_right_associative { 1 } else { 0 },
            &token.0,
        )?;
        let kind = match &token.1 {
            TokenKind::Plus => ExpressionKind::Binary {
                operation: BinaryOperation::Plus,
                left: Box::new(left),
                right: Box::new(right),
            },
            TokenKind::Minus => ExpressionKind::Binary {
                operation: BinaryOperation::Minus,
                left: Box::new(left),
                right: Box::new(right),
            },
            TokenKind::Star => ExpressionKind::Binary {
                operation: BinaryOperation::Multiply,
                left: Box::new(left),
                right: Box::new(right),
            },
            TokenKind::Slash => ExpressionKind::Binary {
                operation: BinaryOperation::Divide,
                left: Box::new(left),
                right: Box::new(right),
            },
            _ => unimplemented!(),
        };

        Ok((token.0, kind))
    }

    fn get_precedence(&self) -> usize {
        self.precedence
    }
}
