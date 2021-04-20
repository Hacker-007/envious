use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{expression::Expression, Parser},
};

use super::prefix_parselet::PrefixParselet;

pub struct ParenthesisParselet;
impl<'a> PrefixParselet<'a> for ParenthesisParselet {
    fn parse(
        &self,
        parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let expression = parser.parse_expression(0, token.0)?;
        let (right_parenthesis_span, _) = parser.expect(TokenKind::RightParenthesis, expression.0)?;
        Ok((token.0.combine(right_parenthesis_span), expression.1))
    }
}
