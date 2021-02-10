use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{Expression, ExpressionKind},
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
impl PrefixParselet for LetParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expression, Error> {
        let identifier = parser.expect(TokenKind::Identifier(0))?;
        let id = get!(identifier, TokenKind::Identifier(id), id);
        let given_type = {
            if let Some((_, TokenKind::Colon)) = parser.peek() {
                parser.consume()?;
                match parser.consume()? {
                    (_, TokenKind::Any) => None,
                    (_, TokenKind::Void) => Some(Type::Void),
                    (_, TokenKind::Int) => Some(Type::Int),
                    (_, TokenKind::Float) => Some(Type::Float),
                    (_, TokenKind::Boolean) => Some(Type::Boolean),
                    (_, TokenKind::String) => Some(Type::String),
                    (span, actual_kind) => {
                        return Err(Error::ExpectedKind {
                            span,
                            expected_kinds: vec![
                                TokenKind::Any,
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
                None
            }
        };

        parser.expect(TokenKind::EqualSign)?;
        let expression = parser.parse_expression(0)?;

        Ok((
            token.0,
            ExpressionKind::Let {
                name: (identifier.0, id),
                given_type,
                expression: Box::new(expression),
            },
        ))
    }
}
