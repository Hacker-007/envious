use crate::{
    error::Error,
    lexer::token::Token,
    parser::{expression::Expression, Parser},
};

/// Trait used by prefix parselets to parse different expressions.
pub trait PrefixParselet {
    /// This method parses the given token into a prefix expression.
    ///
    /// This method assumes that the token provided is the correct token for
    /// the given parselet. For example, the `IfParselet` expects that the
    /// token provided is the `If` token. If the proper token is not provided,
    /// then the parselet will panic.
    ///
    /// # Arguments
    /// * `token` - The token associated with the given prefix parselet.
    fn parse(
        &self,
        parser: &mut Parser<impl Iterator<Item = Token>>,
        token: Token,
    ) -> Result<Expression, Error>;
}
