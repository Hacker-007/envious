use crate::{
    error::Error,
    lexer::token::Token,
    parser::{expression::Expression, Parser},
};

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expression, Error>;
}
