use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{Expression, ExpressionKind, Identifier, Let},
        Parser,
    },
    semantic_analyzer::types::Type,
};

use super::prefix_parselet::PrefixParselet;

macro_rules! get {
    ($token: ident, $pattern: pat, $expression: expr) => {
        if let $pattern = $token.1 {
            $expression
        } else {
            unreachable!()
        };
    };
}

pub struct LetParselet;
impl<'a> PrefixParselet<'a> for LetParselet {
    fn parse(
        &self,
        parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let identifier = parser.expect(TokenKind::Identifier(0), token.0)?;
        let id = get!(identifier, TokenKind::Identifier(id), id);
        let (given_type, type_span) = {
            if let Some((_, TokenKind::Colon)) = parser.peek() {
                let (colon_span, _) = parser.consume(identifier.0)?;
                match parser.consume(colon_span)? {
                    (span, TokenKind::Void) => (Some(Type::Void), Some(span)),
                    (span, TokenKind::Int) => (Some(Type::Int), Some(span)),
                    (span, TokenKind::Float) => (Some(Type::Float), Some(span)),
                    (span, TokenKind::Boolean) => (Some(Type::Boolean), Some(span)),
                    (span, TokenKind::String) => (Some(Type::String), Some(span)),
                    (span, actual_kind) => {
                        return Err(Error::ExpectedKind {
                            span,
                            expected_kinds: vec![
                                TokenKind::Void,
                                TokenKind::Int,
                                TokenKind::Float,
                                TokenKind::Boolean,
                                TokenKind::String,
                            ],
                            actual_kind,
                        })
                    }
                }
            } else {
                (None, None)
            }
        };

        let last_span = if let Some(span) = type_span {
            span
        } else {
            identifier.0
        };

        let (equal_span, _) = parser.expect(TokenKind::EqualSign, last_span)?;
        let expression = parser.parse_expression(0, equal_span)?;

        Ok((
            token.0,
            ExpressionKind::Let(Let {
                name: (identifier.0, Identifier(id)),
                given_type,
                expression: Box::new(expression),
            }),
        ))
    }
}
