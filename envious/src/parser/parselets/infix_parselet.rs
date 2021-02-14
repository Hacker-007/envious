use crate::{
    error::Error,
    lexer::token::Token,
    parser::{expression::Expression, Parser},
};

pub trait InfixParselet {
    /// This method parses the given token and the expression into an
    /// infix expression.
    ///
    /// This method assumes that the token and the expression provided
    /// are the correct token and expression for the given parselet.
    /// For example, the `BinaryOperationParselet` expects that the
    /// token provided is a binary operation token and that the left expression is
    /// the first operand. If the proper token nor the proper expression are not provided,
    /// then the parselet will panic.
    ///
    /// # Arguments
    /// * `left` - The first expression that is already parsed.
    /// * `token` - The token associated with the given infix parselet.
    fn parse(
        &self,
        parser: &mut Parser<impl Iterator<Item = Token>>,
        left: Expression,
        token: Token,
    ) -> Result<Expression, Error>;

    /// This method gets the precedence of the infix parselet.
    /// This is used to determine whether to continue parsing the
    /// infix expression or whether to stop.
    fn get_precedence(&self) -> usize;
}
