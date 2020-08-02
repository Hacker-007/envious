//! The Parser struct takes the tokens from the Lexer and constructs a tree structure called the AST.
//! This AST can then be converted in Dark code because that is compilation target of this language.
//!
//! The Parser must be the second thing that is called because it takes in the tokens from the lexer.
//!
//! # Example
//! ```
//! # fn run() -> Result<(), Error> {
//! let contents = "print(1)";
//! let tokens = Lexer::default().lex(contents)?;
//! let ast = Parser::new(tokens).parse()?;
//! # Ok(())
//! # }
//! ```

use crate::{
    ast::{
        expression::Expression,
        expression_kind::{
            BinaryEqualityOperation, BinaryOperation, ExpressionKind, UnaryOperation,
        },
    },
    errors::{error::Error, error_kind::ErrorKind},
    tokens::{token::Token, token_kind::TokenKind}, semantic_analyzer::{type_checker::TypeChecker, types::Types},
};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Parser {
    last_position: usize,
    tokens: VecDeque<(usize, TokenKind)>,
    identifier_mapping: HashMap<String, Types>,
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
                identifier_mapping: HashMap::new(),
        }
    }

    /// Resets the parser with the supplied tokens.
    ///
    /// # Arguments
    /// `tokens` - The new tokens to parse.
    pub fn with_tokens(&mut self, tokens: VecDeque<Token>) {
        self.last_position = tokens.front().map_or(0, |token| token.pos);
        self.tokens = tokens
            .into_iter()
            .map(|token| (token.pos, token.kind))
            .collect();
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
            Some((pos, TokenKind::If)) => {
                self.last_position = *pos;
                self.parse_if_expression()
            }
            Some((_, TokenKind::Identifier(_)))
                if self.tokens.get(1).map_or(false, |(_, kind)| {
                    matches!(kind, TokenKind::LeftParenthesis)
                }) =>
            {
                self.parse_function_call_expression()
            }
            Some((_, TokenKind::LeftCurlyBrace)) => self.parse_block_expression(),
            Some(_) => self.parse_equality_expression(),
            None => Err(Error::new(
                ErrorKind::Expected("An Expression".to_owned()),
                self.last_position,
            )),
        }
    }

    /// Parses a let expression. This may take different forms and so, all forms must be accounted for.
    fn parse_let_expression(&mut self) -> Result<Expression, Error> {
        let (pos, _) = self.tokens.pop_front().unwrap();
        match self.tokens.pop_front() {
            Some((ident_pos, TokenKind::Identifier(name))) => {
                let mut var_type = None;
                if let Some((_, TokenKind::Colon)) = self.tokens.front() {
                    self.tokens.pop_front();
                    match self.tokens.pop_front() {
                        Some((_, TokenKind::Int)) => var_type = Some(Types::Int),
                        Some((_, TokenKind::Float)) => var_type = Some(Types::Float),
                        Some((_, TokenKind::Boolean)) => var_type = Some(Types::Boolean),
                        Some((_, TokenKind::String)) => var_type = Some(Types::String),
                        Some((pos, kind)) => {
                            return Err(Error::new(
                                ErrorKind::TypeMismatch("A Type".to_owned(), kind.get_name()),
                                pos,
                            ))
                        }
                        None => {
                            return Err(Error::new(
                                ErrorKind::Expected("A Type".to_owned()),
                                self.last_position,
                            ))
                        }
                    }
                }

                if let Some(new_type) = var_type {
                    if let Some(defined_type) = self.identifier_mapping.get(&name) {
                        if defined_type != &Types::Any && &new_type != defined_type {
                            return Err(Error::new(ErrorKind::TypeMismatch((*defined_type).into(), new_type.into()), ident_pos))
                        }
                    }
                }

                if let Some((_, TokenKind::ColonEqualSign)) = self.tokens.front() {
                    self.tokens.pop_front();
                    let value = self.parse_expression()?;
                    if !self.identifier_mapping.contains_key(&name) {
                        self.identifier_mapping.insert(name.clone(), TypeChecker::check_types(&value)?.ok_or_else(|| Error::new(ErrorKind::Expected("A Non-Void Type".to_owned()), value.pos))?);
                    }

                    Ok(Expression::new(
                        ExpressionKind::LetExpression(name, var_type.unwrap_or(Types::Any), Some(Box::new(value))),
                        pos,
                    ))
                } else {
                    if !self.identifier_mapping.contains_key(&name) {
                        self.identifier_mapping.insert(name.clone(), var_type.unwrap_or(Types::Any));
                    }

                    Ok(Expression::new(
                        ExpressionKind::LetExpression(name, var_type.unwrap_or(Types::Any), None),
                        pos,
                    ))
                }
            }
            Some((pos, kind)) => Err(Error::new(
                ErrorKind::TypeMismatch("An Identifier".to_owned(), kind.get_name()),
                pos,
            )),
            None => Err(Error::new(
                ErrorKind::Expected("An Identifier".to_owned()),
                self.last_position,
            )),
        }
    }

    /// Parses a function call expression. This only has a single form to parse, so it is much cleaner.
    fn parse_function_call_expression(&mut self) -> Result<Expression, Error> {
        let (pos, function_name) =
            if let (pos, TokenKind::Identifier(function_name)) = self.tokens.pop_front().unwrap() {
                (pos, function_name)
            } else {
                unreachable!()
            };
        match self.tokens.pop_front() {
            Some((paren_pos, TokenKind::LeftParenthesis)) => {
                self.last_position = paren_pos;
                if let Some((_, TokenKind::RightParenthesis)) = self.tokens.front() {
                    self.tokens.pop_front();
                    return Ok(Expression::new(
                        ExpressionKind::FunctionCallExpression(function_name, vec![]),
                        pos,
                    ));
                }

                let value = self.parse_arguments()?;
                match self.tokens.pop_front() {
                    Some((_, TokenKind::RightParenthesis)) => Ok(Expression::new(
                        ExpressionKind::FunctionCallExpression(function_name, value),
                        pos,
                    )),
                    Some((pos, kind)) => Err(Error::new(
                        ErrorKind::TypeMismatch("A Right Parenthesis".to_owned(), kind.get_name()),
                        pos,
                    )),
                    None => Err(Error::new(
                        ErrorKind::Expected("A Right Parenthesis".to_owned()),
                        self.last_position,
                    )),
                }
            }
            Some((pos, kind)) => Err(Error::new(
                ErrorKind::TypeMismatch("A Left Parenthesis".to_owned(), kind.get_name()),
                pos,
            )),
            None => Err(Error::new(
                ErrorKind::Expected("A Left Parenthesis".to_owned()),
                self.last_position,
            )),
        }
    }

    // Parses a list of arguments. This is used by function call expressions.
    fn parse_arguments(&mut self) -> Result<Vec<Expression>, Error> {
        let mut arguments = vec![];
        arguments.push(self.parse_argument()?);
        while let Some((pos, TokenKind::Comma)) = self.tokens.front() {
            self.last_position = *pos;
            self.tokens.pop_front();
            arguments.push(self.parse_argument()?);
        }

        Ok(arguments)
    }

    // Parses a single argument. The implementation details of this function may change and it why the implementation has been decoupled.
    fn parse_argument(&mut self) -> Result<Expression, Error> {
        self.parse_expression()
    }

    fn parse_block_expression(&mut self) -> Result<Expression, Error> {
        let (pos, _) = self.tokens.pop_front().unwrap();
        self.last_position = pos;
        let mut expressions = vec![];
        let mut found_end_brace = false;
        while let Some((_, kind)) = self.tokens.front() {
            if matches!(kind, &TokenKind::RightCurlyBrace) {
                self.tokens.pop_front();
                found_end_brace = true;
                break;
            } else {
                expressions.push(self.parse_expression()?);
            }
        }

        if !found_end_brace {
            Err(Error::new(
                ErrorKind::Expected("A Right Curly Brace".to_owned()),
                self.last_position,
            ))
        } else {
            Ok(Expression::new(
                ExpressionKind::BlockExpression(expressions),
                pos,
            ))
        }
    }

    /// Parses an equality expression. This may take different forms but for now, it only has one form.
    fn parse_equality_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_term()?;
        while let Ok((pos, operation)) = self.parse_equality_operator() {
            self.tokens.pop_front();
            self.last_position = pos;
            let right = self.parse_term()?;
            left = Expression::new(
                ExpressionKind::BinaryEqualityExpression(
                    operation,
                    Box::new(left),
                    Box::new(right),
                ),
                pos,
            );
        }

        Ok(left)
    }

    /// Parses an equality operator. Currently, the only operator is =.
    fn parse_equality_operator(&mut self) -> Result<(usize, BinaryEqualityOperation), Error> {
        match self.tokens.front() {
            Some((pos, TokenKind::EqualSign)) => {
                self.last_position = *pos;
                Ok((*pos, BinaryEqualityOperation::Equals))
            }
            Some((pos, kind)) => Err(Error::new(
                ErrorKind::TypeMismatch("An Equal Sign".to_owned(), kind.get_name()),
                *pos,
            )),
            None => Err(Error::new(
                ErrorKind::Expected("An Equal Sign".to_owned()),
                self.last_position,
            )),
        }
    }

    /// Parses an if expression.
    fn parse_if_expression(&mut self) -> Result<Expression, Error> {
        let (pos, _) = self.tokens.pop_front().unwrap();
        let condition = self.parse_expression()?;
        let code = Box::new(self.parse_expression()?);
        Ok(Expression::new(
            ExpressionKind::IfExpression(Box::new(condition), code),
            pos,
        ))
    }

    /// Parses a term. A term is basically a factor + or - another factor.
    fn parse_term(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_factor()?;
        while let Ok((pos, operation)) = self.parse_term_operator() {
            self.tokens.pop_front();
            let right = self.parse_factor()?;
            left = Expression::new(
                ExpressionKind::InfixBinaryExpression(operation, Box::new(left), Box::new(right)),
                pos,
            );
        }

        Ok(left)
    }

    /// Parses the different term operators. These operators have a lower precedence than the factor operators.
    fn parse_term_operator(&mut self) -> Result<(usize, BinaryOperation), Error> {
        match self.tokens.front() {
            Some((pos, TokenKind::Plus)) => {
                self.last_position = *pos;
                Ok((*pos, BinaryOperation::Plus))
            }
            Some((pos, TokenKind::Minus)) => {
                self.last_position = *pos;
                Ok((*pos, BinaryOperation::Minus))
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
                ExpressionKind::InfixBinaryExpression(operation, Box::new(left), Box::new(right)),
                pos,
            );
        }

        Ok(left)
    }

    /// Parses a factor operator. These have higher priority than the term operators.
    fn parse_factor_operator(&mut self) -> Result<(usize, BinaryOperation), Error> {
        match self.tokens.front() {
            Some((pos, TokenKind::Star)) => {
                self.last_position = *pos;
                Ok((*pos, BinaryOperation::Multiply))
            }
            Some((pos, TokenKind::Slash)) => {
                self.last_position = *pos;
                Ok((*pos, BinaryOperation::Divide))
            }
            Some((pos, kind)) => Err(Error::new(
                ErrorKind::TypeMismatch("A Multiply Or Divide Operator".to_owned(), kind.get_name()),
                *pos,
            )),
            None => Err(Error::new(
                ErrorKind::Expected("A Multiply Or Divide Operator".to_owned()),
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
                let ident_type = self.identifier_mapping.get(&name).copied();
                Ok(Expression::new(ExpressionKind::Identifier(name, ident_type), pos))
            }
            Some((pos, TokenKind::Plus)) => {
                self.last_position = pos;
                let value = self.parse_primary()?;
                Ok(Expression::new(
                    ExpressionKind::UnaryExpression(UnaryOperation::Positive, Box::new(value)),
                    pos,
                ))
            }
            Some((pos, TokenKind::Minus)) => {
                self.last_position = pos;
                let value = self.parse_primary()?;
                Ok(Expression::new(
                    ExpressionKind::UnaryExpression(UnaryOperation::Negative, Box::new(value)),
                    pos,
                ))
            }
            Some((pos, TokenKind::LeftParenthesis)) => {
                self.last_position = pos;
                let expr = self.parse_expression()?;
                if let Some((_, TokenKind::RightParenthesis)) = self.tokens.front() {
                    self.last_position = pos;
                    self.tokens.pop_front();
                    Ok(Expression::new(
                        ExpressionKind::ParenthesizedExpression(Box::new(expr)),
                        pos
                    ))
                } else {
                    Err(Error::new(
                        ErrorKind::Expected("A Right Parenthesis".to_owned()),
                        self.last_position,
                    ))
                }
            }
            Some((pos, kind)) => Err(Error::new(
                ErrorKind::TypeMismatch(
                    "An Int, A Float, Or A Left Parenthesis".to_owned(),
                    kind.get_name(),
                ),
                pos,
            )),
            None => Err(Error::new(
                ErrorKind::Expected("An Int, A Float, Or A Left Parenthesis".to_owned()),
                self.last_position,
            )),
        }
    }
}
