use crate::{
    error::Error,
    lexer::token::Token,
    parser::{
        expression::{Expression, ExpressionKind, While},
        Parser,
    },
};

use super::prefix_parselet::PrefixParselet;

pub struct WhileParselet;
impl<'a> PrefixParselet<'a> for WhileParselet {
    fn parse(
        &self,
        parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let condition = parser.parse_expression(0, token.0)?;
        let expression = parser.parse_expression(0, condition.0)?;

        Ok((
            token.0.combine(expression.0),
            ExpressionKind::While(While {
                condition: Box::new(condition),
                expression: Box::new(expression),
            }),
        ))
    }
}
