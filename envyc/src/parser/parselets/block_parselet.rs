use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{Expression, ExpressionKind},
        Parser,
    },
};

use super::prefix_parselet::PrefixParselet;

pub struct BlockParselet;
impl<'a> PrefixParselet<'a> for BlockParselet {
    fn parse(
        &self,
        parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let mut expressions = vec![];
        let mut last_span = token.0;
        loop {
            let expression = parser.parse_expression(0, last_span)?;
            last_span = expression.0;
            expressions.push(expression);
            if let Some((_, TokenKind::RightCurlyBrace)) = parser.peek() {
                last_span = parser.consume(last_span)?.0;
                break;
            }
        }

        Ok((token.0.combine(last_span), ExpressionKind::Block(expressions)))
    }
}
