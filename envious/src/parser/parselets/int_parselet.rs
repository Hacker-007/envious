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

pub struct IntParselet;
impl PrefixParselet for IntParselet {
    fn parse(
        &self,
        _: &mut Parser<impl Iterator<Item = Token>>,
        token: Token,
    ) -> Result<Expression, Error> {
        let value = get!(token, TokenKind::IntegerLiteral(value), value);
        Ok((token.0, ExpressionKind::Int(value)))
    }
}
