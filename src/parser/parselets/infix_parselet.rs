use crate::{
    error::Error,
    lexer::token::Token,
    parser::{expression::Expression, Parser},
};

pub trait InfixParselet {
    fn parse(
        &self,
        parser: &mut Parser,
        left: Expression,
        token: Token,
    ) -> Result<Expression, Error>;

    fn get_precedence(&self) -> usize;
}
