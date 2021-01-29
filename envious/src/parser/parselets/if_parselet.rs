use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{Expression, ExpressionKind},
        Parser,
    },
};

use super::prefix_parselet::PrefixParselet;

pub struct IfParselet;
impl PrefixParselet for IfParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expression, Error> {
        let condition = parser.parse_expression(0)?;
        parser.expect(TokenKind::Then)?;
        let then_branch = parser.parse_expression(0)?;
        let mut else_branch = None;
        if let Some((_, TokenKind::Else)) = parser.peek() {
            parser.consume().unwrap();
            else_branch = Some(Box::new(parser.parse_expression(0)?));
        }

        Ok((
            token.0,
            ExpressionKind::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch,
            },
        ))
    }
}
