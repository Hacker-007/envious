use std::{iter::Peekable, vec::IntoIter};

use expression::Expression;

use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
};

use self::parselets::{
    infix_parselet::InfixParselet, precedence::Precedence, prefix_parselet::PrefixParselet,
    BinaryOperationParselet, BooleanParselet, FloatParselet, IdentifierParselet, IfParselet,
    IntParselet, PrefixOperationParselet, StringParselet,
};

pub mod expression;
pub mod parselets;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse_program(&mut self) -> (Vec<Expression>, Vec<Error>) {
        let mut expressions = vec![];
        let mut errors = vec![];
        while self.tokens.peek().is_some() {
            match self.parse_expression(0) {
                Ok(expression) => expressions.push(expression),
                Err(error) => errors.push(error),
            }
        }

        (expressions, errors)
    }

    fn parse_expression(&mut self, precedence: usize) -> Result<Expression, Error> {
        let token = self.consume()?;
        let mut left = self.parse_prefix(token)?;
        while precedence < self.get_precedence() {
            let token = self.consume()?;
            left = self.parse_infix(left, token)?;
        }

        Ok(left)
    }

    fn parse_prefix(&mut self, token: Token) -> Result<Expression, Error> {
        match token.1 {
            TokenKind::IntegerLiteral(_) => IntParselet.parse(self, token),
            TokenKind::FloatLiteral(_) => FloatParselet.parse(self, token),
            TokenKind::BooleanLiteral(_) => BooleanParselet.parse(self, token),
            TokenKind::StringLiteral(_) => StringParselet.parse(self, token),
            TokenKind::Identifier(_) => IdentifierParselet.parse(self, token),
            TokenKind::Plus | TokenKind::Minus | TokenKind::Not => {
                PrefixOperationParselet::new(Precedence::Unary).parse(self, token)
            }
            TokenKind::If => IfParselet.parse(self, token),
            _ => Err(Error::ExpectedPrefixExpression {
                span: token.0,
                found_kind: token.1,
            }),
        }
    }

    fn parse_infix(&mut self, left: Expression, token: Token) -> Result<Expression, Error> {
        match token.1 {
            TokenKind::Plus | TokenKind::Minus => {
                BinaryOperationParselet::new(Precedence::Addition, false).parse(self, left, token)
            }
            TokenKind::Star | TokenKind::Slash => {
                BinaryOperationParselet::new(Precedence::Multiplication, false)
                    .parse(self, left, token)
            }
            _ => unreachable!(),
        }
    }

    fn get_precedence(&mut self) -> usize {
        if let Some((_, kind)) = self.tokens.peek() {
            match kind {
                TokenKind::Plus | TokenKind::Minus => Precedence::Addition.into(),
                TokenKind::Star | TokenKind::Slash => Precedence::Multiplication.into(),
                _ => 0,
            }
        } else {
            0
        }
    }

    fn consume(&mut self) -> Result<Token, Error> {
        match self.tokens.next() {
            Some(token) => Ok(token),
            None => Err(Error::UnexpectedEndOfInput),
        }
    }

    fn expect(&mut self, expected_kind: TokenKind) -> Result<Token, Error> {
        let token = self.consume()?;
        if token.1 == expected_kind {
            Ok(token)
        } else {
            Err(Error::ExpectedKind {
                span: token.0,
                expected_kind,
                actual_kind: token.1,
            })
        }
    }
}
