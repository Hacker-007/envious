use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{Expression, ExpressionKind},
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
impl PrefixParselet for IdentifierParselet {
    fn parse(&self, _: &mut Parser, token: Token) -> Result<Expression, Error> {
        let name = get!(token, TokenKind::Identifier(value), value);
        Ok((token.0, ExpressionKind::Identifier(name)))
    }
}
