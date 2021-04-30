use crate::{error::Error, lexer::token::{Token, TokenKind}, parser::{
        expression::{Expression, ExpressionKind},
        Parser,
    }};

use super::prefix_parselet::PrefixParselet;

pub struct ReturnParselet;
impl<'a> PrefixParselet<'a> for ReturnParselet {
    fn parse(
        &self,
        parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let mut expression = None;
        match parser.peek() {
            Some((_, TokenKind::SemiColon)) => {
                parser.consume(token.0)?;
            },
            Some(_) => expression = Some(Box::new(parser.parse_expression(0, token.0)?)),
            None => {}
        }

        let span = if let Some(expression) = &expression {
            token.0.combine(expression.0)
        } else {
            token.0
        };

        Ok((span, ExpressionKind::Return(expression)))
    }
}
