use std::{iter::Peekable, mem};

use expression::{BinaryOperation, Expression, UnaryOperation};
use parselets::LetParselet;

use crate::{
    error::{Error, Span},
    lexer::token::{Token, TokenKind},
    semantic_analyzer::types::Type,
};

use self::{
    ast::{Function, Parameter, Program},
    parselets::{
        infix_parselet::InfixParselet, precedence::Precedence, prefix_parselet::PrefixParselet,
        BinaryOperationParselet, BooleanParselet, FloatParselet, IdentifierParselet, IfParselet,
        IntParselet, PrefixOperationParselet, StringParselet,
    },
};

pub mod ast;
pub mod expression;
pub mod parselets;

/// Struct that transforms the vector of tokens into a vector of expressions.
///
/// The `Parser` uses a mixture of the Pratt parsing technique and the
/// recursive descent algorithm. It achieves this through the mini parsers
/// called parselets.
pub struct Parser<T: Iterator<Item = Token>> {
    tokens: Peekable<T>,
}

impl<T: Iterator<Item = Token>> Parser<T> {
    pub fn new(tokens: Peekable<T>) -> Self {
        Self { tokens }
    }

    /// Walks through the tokens and constructs a program, or a vector
    /// of functions.
    pub fn parse(&mut self) -> Result<Program, Vec<Error>> {
        let mut functions = vec![];
        let mut errors = vec![];
        while let Some((_, _)) = self.tokens.peek() {
            let dummy_span = Span::new(String::new(), 1, 1, 1, 1);
            match self.parse_function(&dummy_span) {
                Ok(function) => functions.push(function),
                Err(error) => errors.push(error),
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Program::new(functions))
        }
    }

    fn parse_function(&mut self, span: &Span) -> Result<Function, Error> {
        let (define_span, _) = self.expect(TokenKind::Define, span)?;
        if let (function_name_span, TokenKind::Identifier(id)) =
            self.expect(TokenKind::Identifier(0), &define_span)?
        {
            let (left_paren_span, _) =
                self.expect(TokenKind::LeftParenthesis, &function_name_span)?;
            let parameters = self.parse_parameters()?;
            let last_span = parameters
                .iter()
                .last()
                .map_or(&left_paren_span, |param| &param.span);
            let (right_paren_span, _) = self.expect(TokenKind::RightParenthesis, last_span)?;
            let (eq_span, _) = self.expect(TokenKind::EqualSign, &right_paren_span)?;
            let body = self.parse_expression(0, &eq_span)?;
            Ok(Function::new(function_name_span, id, parameters, body))
        } else {
            unreachable!()
        }
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, Error> {
        let mut parameters = vec![];
        while let Some((_, kind)) = self.tokens.peek() {
            if kind == &TokenKind::RightParenthesis {
                break;
            }

            let (param_span, kind) = self.tokens.next().unwrap();
            let id = match kind {
                TokenKind::Identifier(id) => id,
                _ => {
                    return Err(Error::ExpectedKind {
                        span: param_span,
                        expected_kinds: vec![TokenKind::Identifier(0)],
                        actual_kind: kind,
                    })
                }
            };

            let (colon_span, _) = self.expect(TokenKind::Colon, &param_span)?;
            let (type_span, kind) = self.consume(&colon_span)?;
            let ty = match kind {
                TokenKind::Void => Type::Void,
                TokenKind::Int => Type::Int,
                TokenKind::Float => Type::Float,
                TokenKind::Boolean => Type::Boolean,
                TokenKind::String => Type::String,
                _ => {
                    return Err(Error::ExpectedKind {
                        span: type_span,
                        expected_kinds: vec![
                            TokenKind::Void,
                            TokenKind::Int,
                            TokenKind::Float,
                            TokenKind::Boolean,
                            TokenKind::String,
                        ],
                        actual_kind: kind,
                    })
                }
            };

            parameters.push(Parameter::new(param_span, ty, id));
            if let Some((_, TokenKind::Comma)) = self.tokens.peek() {
                self.tokens.next();
            } else {
                break;
            }
        }

        Ok(parameters)
    }

    /// Parses a single expression. This function follows the Pratt parsing technique
    /// to handle operator precedence and infix operations.
    ///
    /// # Arguments
    /// * `precendence` - The current precedence to use when evaluating expressions.
    /// * `span` - The `Span` of the current token.
    fn parse_expression(&mut self, precedence: usize, span: &Span) -> Result<Expression, Error> {
        let token = self.consume(span)?;
        let mut left = self.parse_prefix(token)?;
        while precedence < self.get_precedence() {
            let token = self.consume(&left.0)?;
            left = self.parse_infix(left, token)?;
        }

        Ok(left)
    }

    /// Parses a prefix expression by analyzing the type of the token.
    /// This function looks through the kind of the token to determine which
    /// parselet to use. Then, it runs the parselet and returns the result of the execution.
    ///
    /// # Arguments
    /// * `token` - The token to parse into a prefix expression.
    fn parse_prefix(&mut self, token: Token) -> Result<Expression, Error> {
        match token.1 {
            TokenKind::IntegerLiteral(_) => IntParselet.parse(self, token),
            TokenKind::FloatLiteral(_) => FloatParselet.parse(self, token),
            TokenKind::BooleanLiteral(_) => BooleanParselet.parse(self, token),
            TokenKind::StringLiteral(_) => StringParselet.parse(self, token),
            TokenKind::Identifier(_) => IdentifierParselet.parse(self, token),
            TokenKind::Plus => {
                PrefixOperationParselet::new(Precedence::Unary, UnaryOperation::Plus)
                    .parse(self, token)
            }
            TokenKind::Minus => {
                PrefixOperationParselet::new(Precedence::Unary, UnaryOperation::Minus)
                    .parse(self, token)
            }
            TokenKind::Not => PrefixOperationParselet::new(Precedence::Unary, UnaryOperation::Not)
                .parse(self, token),
            TokenKind::If => IfParselet.parse(self, token),
            TokenKind::Let => LetParselet.parse(self, token),
            _ => Err(Error::ExpectedPrefixExpression {
                span: token.0,
                found_kind: token.1,
            }),
        }
    }

    /// Parses an infix expression by analyzing the type of the token.
    /// This function looks through the kind of the token to determine which
    /// parselet to use. Then, it runs the parselet and returns the result of the execution.
    ///
    /// # Arguments
    /// * `left` - The first part of the infix expression that was already parsed.
    /// * `token` - The token to parse into a prefix expression.
    fn parse_infix(&mut self, left: Expression, token: Token) -> Result<Expression, Error> {
        match token.1 {
            TokenKind::Plus => {
                BinaryOperationParselet::new(Precedence::Addition, BinaryOperation::Plus, false)
                    .parse(self, left, token)
            }
            TokenKind::Minus => {
                BinaryOperationParselet::new(Precedence::Addition, BinaryOperation::Minus, false)
                    .parse(self, left, token)
            }
            TokenKind::Star => BinaryOperationParselet::new(
                Precedence::Multiplication,
                BinaryOperation::Multiply,
                false,
            )
            .parse(self, left, token),
            TokenKind::Slash => BinaryOperationParselet::new(
                Precedence::Multiplication,
                BinaryOperation::Divide,
                false,
            )
            .parse(self, left, token),
            _ => unreachable!(),
        }
    }

    /// Analyzes the type of the next token without consuming i
    /// and then returns the precedence associated with the token.
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

    /// Returns an immutable reference to the next token
    /// without consuming it.
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    /// Consumes the next token in the `token` iterator.
    /// This function may result in an error if there are no
    /// more token remaining, but one was requested.
    ///
    /// # Arguments
    /// `span` - The `Span` of the previous token.
    fn consume(&mut self, span: &Span) -> Result<Token, Error> {
        match self.tokens.next() {
            Some(token) => Ok(token),
            None => Err(Error::UnexpectedEndOfInput(span.clone())),
        }
    }

    /// Consumes the next token and then verifies that the kind of the
    /// token matches the expected_kind. This function results in an error
    /// if there are no more tokens remaining or if the kind of the current token
    /// does not match the expected_kind.
    ///
    /// # Arguments
    /// * `expected_kind` - The kind expected of the next token.
    /// `span` - The `Span` of the previous token.
    fn expect(&mut self, expected_kind: TokenKind, span: &Span) -> Result<Token, Error> {
        let token = self.consume(span)?;

        if mem::discriminant(&token.1) == mem::discriminant(&expected_kind) {
            Ok(token)
        } else {
            Err(Error::ExpectedKind {
                span: token.0,
                expected_kinds: vec![expected_kind],
                actual_kind: token.1,
            })
        }
    }
}
