//! The Parser struct takes the tokens from the Lexer and constructs a tree structure called the AST.
//! This AST can then be converted in Dark code because that is compilation target of this language.
//!
//! The Parser must be the second thing that is called because it takes in the tokens from the lexer.
//!
//! # Example
//! ```
//! # fn run() -> Result<(), Error> {
//! let contents = "print(1)";
//! let tokens = Lexer::new().lex(contents)?;
//! let ast = Parser::new(tokens).parse()?;
//! # Ok(())
//! # }
//! ```

use crate::{
    ast::{
        expression::Expression,
        expression_kind::{ExpressionKind, Operation, Type},
    },
    errors::{error::Error, error_kind::ErrorKind},
    tokens::{token::Token, token_kind::TokenKind},
};
use std::collections::VecDeque;

pub struct Parser {
    last_position: usize,
    tokens: VecDeque<(usize, TokenKind)>,
}

impl Parser {
    /// Constructs a new parser with the current position set to 1 (the first token).
    pub fn new(tokens: VecDeque<Token>) -> Parser {
        Parser {
            last_position: tokens.front().map_or(0, |token| token.pos),
            tokens: tokens
                .into_iter()
                .map(|token| (token.pos, token.kind))
                .collect(),
        }
    }

    /// Parse the tokens in the tokens field.
    /// This function returns a vector of all of the parsed expressions.
    /// The signature of this function might change to return a vector of errors.
    pub fn parse(&mut self) -> Result<Vec<Expression>, Error> {
        let mut expressions = vec![];
        while let Some(_) = self.tokens.front() {
            expressions.push(self.parse_expression()?);
        }

        Ok(expressions)
    }

    /// Parse a single expression.
    fn parse_expression(&mut self) -> Result<Expression, Error> {
        match self.tokens.front() {
            Some((pos, TokenKind::Let)) => {
                self.last_position = *pos;
                self.parse_let_expression()
            }
            Some((pos, TokenKind::BuiltInFunction(function_name))) => {
                self.last_position = *pos;
                match function_name.as_str() {
                    "print" => {
                        self.parse_print_expression()
                    },
                    _ => unreachable!()
                }
            }
            Some(_) => self.parse_term(),
            None => Err(Error::new(ErrorKind::Expected("An Expression".to_owned()), self.last_position))
        }
    }
    
    /// Parse a let expression. This may take different forms and so, all forms must be accounted for.
    fn parse_let_expression(&mut self) -> Result<Expression, Error> {
        let (pos, _) = self.tokens.pop_front().unwrap();
        match self.tokens.pop_front() {
            Some((_, TokenKind::Identifier(name))) => {
                let mut var_type = Type::Unknown;
                match self.tokens.front() {
                    Some((_, TokenKind::Colon)) => {
                        self.tokens.pop_front();
                        match self.tokens.pop_front() {
                            Some((_, TokenKind::Int)) => var_type = Type::Int,
                            Some((_, TokenKind::Float)) => var_type = Type::Float,
                            Some((_, TokenKind::Boolean)) => var_type = Type::Boolean,
                            Some((_, TokenKind::String)) => var_type = Type::String,
                            Some((pos, kind)) => return Err(Error::new(ErrorKind::TypeMismatch("A Type".to_owned(), kind.get_name()), pos)),
                            None => return Err(Error::new(ErrorKind::Expected("A Type".to_owned()), self.last_position)),
                        }
                    },
                    _ => {},
                }
                match self.tokens.pop_front() {
                    Some((_, TokenKind::EqualSign)) => {
                        let value = self.parse_expression()?;
                        Ok(Expression::new(ExpressionKind::LetExpression(name, var_type, Box::new(value)), pos))
                    }
                    Some((pos, kind)) => Err(Error::new(ErrorKind::TypeMismatch("An Equal Sign".to_owned(), kind.get_name()), pos)),
                    None => Err(Error::new(ErrorKind::Expected("An Equal Sign".to_owned()), self.last_position)),
                }
            }
            Some((pos, kind)) => Err(Error::new(ErrorKind::TypeMismatch("An Identifier".to_owned(), kind.get_name()), pos)),
            None => Err(Error::new(ErrorKind::Expected("An Identifier".to_owned()), self.last_position)),
        }
    }

    /// Parse a print expression. This only has a single form to parse, so it is much cleaner.
    fn parse_print_expression(&mut self) -> Result<Expression, Error> {
        let (pos, _) = self.tokens.pop_front().unwrap();
        match self.tokens.pop_front() {
            Some((_, TokenKind::LeftParenthesis)) => {
                let value = self.parse_expression()?;
                match self.tokens.pop_front() {
                    Some((_, TokenKind::RightParenthesis)) => Ok(Expression::new(ExpressionKind::PrintExpression(Box::new(value)), pos)),
                    Some((pos, kind)) => Err(Error::new(ErrorKind::TypeMismatch("A Right Parenthesis".to_owned(), kind.get_name()), pos)),
                    None => Err(Error::new(ErrorKind::Expected("A Right Parenthesis".to_owned()), self.last_position)),
                }
            }
            Some((pos, kind)) => Err(Error::new(ErrorKind::TypeMismatch("A Left Parenthesis".to_owned(), kind.get_name()), pos)),
            None => Err(Error::new(ErrorKind::Expected("A Left Parenthesis".to_owned()), self.last_position)),
        }
    }

    /// Parses a term. A term is basically a factor + or - another factor.
    fn parse_term(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_factor()?;
        while let Ok((pos, operation)) = self.parse_term_operator() {
            self.tokens.pop_front();
            let right = self.parse_factor()?;
            left = Expression::new(
                ExpressionKind::InfixBinaryOperation(operation, Box::new(left), Box::new(right)),
                pos,
            );
        }

        Ok(left)
    }

    /// This parses the different term operators. These operators have a lower precedence than the factor operators.
    fn parse_term_operator(&mut self) -> Result<(usize, Operation), Error> {
        match self.tokens.front() {
            Some((pos, TokenKind::Plus)) => {
                self.last_position = *pos;
                Ok((*pos, Operation::Add))
            },
            Some((pos, TokenKind::Minus)) => {
                self.last_position = *pos;
                Ok((*pos, Operation::Subtract))
            }
            Some((pos, kind)) => Err(Error::new(
                ErrorKind::TypeMismatch("A Plus Or Minus Operator".to_owned(), kind.get_name()),
                *pos,
            )),
            None => Err(Error::new(
                ErrorKind::Expected("A Plus Or Minus Operator".to_owned()),
                self.last_position,
            )),
        }
    }

    /// Parses a factor. A factor is basically a primary * or / by a primary.
    fn parse_factor(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_primary()?;
        while let Ok((pos, operation)) = self.parse_factor_operator() {
            self.tokens.pop_front();
            let right = self.parse_primary()?;
            left = Expression::new(
                ExpressionKind::InfixBinaryOperation(operation, Box::new(left), Box::new(right)),
                pos,
            );
        }

        Ok(left)
    }

    /// This parses a factor operator. These have higher priority than the term operators.
    fn parse_factor_operator(&mut self) -> Result<(usize, Operation), Error> {
        match self.tokens.front() {
            Some((pos, TokenKind::Star)) => {
                self.last_position = *pos;
                Ok((*pos, Operation::Multiply))
            },
            Some((pos, TokenKind::Slash)) => {
                self.last_position = *pos;
                Ok((*pos, Operation::Divide))
            },
            Some((pos, kind)) => Err(Error::new(
                ErrorKind::TypeMismatch("A Multiply Operator".to_owned(), kind.get_name()),
                *pos,
            )),
            None => Err(Error::new(
                ErrorKind::Expected("A Multiply Operator".to_owned()),
                self.last_position,
            )),
        }
    }

    /// Parses a primary expression. This can be a literal, or a parenthesized expression.
    fn parse_primary(&mut self) -> Result<Expression, Error> {
        match self.tokens.pop_front() {
            Some((pos, TokenKind::IntegerLiteral(value))) => {
                self.last_position = pos;
                Ok(Expression::new(ExpressionKind::Int(value), pos))
            }
            Some((pos, TokenKind::FloatLiteral(value))) => {
                self.last_position = pos;
                Ok(Expression::new(ExpressionKind::Float(value), pos))
            }
            Some((pos, TokenKind::BooleanLiteral(value))) => {
                self.last_position = pos;
                Ok(Expression::new(ExpressionKind::Boolean(value), pos))
            }
            Some((pos, TokenKind::StringLiteral(value))) => {
                self.last_position = pos;
                Ok(Expression::new(ExpressionKind::String(value), pos))
            }
            Some((pos, TokenKind::Identifier(name))) => {
                self.last_position = pos;
                Ok(Expression::new(ExpressionKind::Identifier(name), pos))
            }
            Some((pos, TokenKind::LeftParenthesis)) => {
                self.last_position = pos;
                let expr = self.parse_expression()?;
                if let Some((pos, TokenKind::RightParenthesis)) = self.tokens.front() {
                    self.last_position = *pos;
                    self.tokens.pop_front();
                    Ok(expr)
                } else {
                    Err(Error::new(ErrorKind::Expected("A Right Parenthesis".to_owned()), self.last_position))
                }
            }
            Some((pos, kind)) => {
                return Err(Error::new(
                    ErrorKind::TypeMismatch(
                        "An Int, A Float, Or A Left Parenthesis".to_owned(),
                        kind.get_name(),
                    ),
                    pos,
                ))
            }
            None => Err(Error::new(ErrorKind::Expected("An Int, A Float, Or A Left Parenthesis".to_owned()), self.last_position)),
        }
    }
}
