use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{Expression, ExpressionKind, Identifier, Application},
        Parser,
    },
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

pub struct IdentifierParselet;
impl<'a> PrefixParselet<'a> for IdentifierParselet {
    fn parse(
        &self,
        parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let id = get!(token, TokenKind::Identifier(id), id);
        if let Some((_, TokenKind::LeftParenthesis)) = parser.peek() {
            let (left_parenthesis_span, _) = parser.consume(token.0)?;
            let mut parameters = Vec::new();
            if let Some((_, TokenKind::RightParenthesis)) = parser.peek() {
                parser.consume(token.0)?;
            } else {
                let mut last_span = left_parenthesis_span;
                loop {
                    let expression = parser.parse_expression(0, last_span)?;
                    parameters.push(expression);
                    
                    match parser.peek() {
                        Some((_, TokenKind::RightParenthesis)) => break,
                        Some((_, TokenKind::Comma)) => {
                            let (comma_span, _) = parser.consume(last_span)?;
                            last_span = comma_span;
                        }
                        Some((span, kind)) => return Err(Error::ExpectedKind {
                            span: *span,
                            expected_kinds: vec![
                                TokenKind::RightParenthesis,
                                TokenKind::Comma,
                            ],
                            actual_kind: *kind,
                        }),
                        None => return Err(Error::UnexpectedEndOfInput(last_span)),
                    }
                }

                parser.consume(token.0)?;
            }

            Ok((
                token.0,
                ExpressionKind::Application(Application {
                    function_name: (token.0, Identifier(id)),
                    parameters,
                })
            ))
        } else {
            Ok((token.0, ExpressionKind::Identifier(Identifier(id))))
        }
    }
}
