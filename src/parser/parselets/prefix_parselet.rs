use crate::{error::Error, lexer::token::{Token, TokenKind}, parser::{Parser, expression::{Expression, ExpressionKind, UnaryOperation}}};

macro_rules! get {
    ($token: ident, $pattern: pat, $expression: expr) => {
        if let $pattern = $token.1 {
            $expression
        } else {
            unreachable!()
        };
    };
}

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expression, Error>;
}

pub struct IntParselet;
impl PrefixParselet for IntParselet {
    fn parse(&self, _: &mut Parser, token: Token) -> Result<Expression, Error> {
        let value = get!(token, TokenKind::IntegerLiteral(value), value);
        Ok((token.0, ExpressionKind::Int(value)))
    }
}

pub struct FloatParselet;
impl PrefixParselet for FloatParselet {
    fn parse(&self, _: &mut Parser, token: Token) -> Result<Expression, Error> {
        let value = get!(token, TokenKind::FloatLiteral(value), value);
        Ok((token.0, ExpressionKind::Float(value)))
    }
}

pub struct BooleanParselet;
impl PrefixParselet for BooleanParselet {
    fn parse(&self, _: &mut Parser, token: Token) -> Result<Expression, Error> {
        let value = get!(token, TokenKind::BooleanLiteral(value), value);
        Ok((token.0, ExpressionKind::Boolean(value)))
    }
}

pub struct StringParselet;
impl PrefixParselet for StringParselet {
    fn parse(&self, _: &mut Parser, token: Token) -> Result<Expression, Error> {
        let value = get!(token, TokenKind::StringLiteral(value), value);
        Ok((token.0, ExpressionKind::String(value)))
    }
}

pub struct IdentifierParselet;
impl PrefixParselet for IdentifierParselet {
    fn parse(&self, _: &mut Parser, token: Token) -> Result<Expression, Error> {
        let name = get!(token, TokenKind::Identifier(value), value);
        Ok((token.0, ExpressionKind::Identifier(name)))
    }
}

pub struct PrefixOperationParselet {
    precedence: usize,
}

impl PrefixOperationParselet {
    pub fn new<T: Into<usize>>(precedence: T) -> Self {
        Self {
            precedence: precedence.into(),
        }
    }
}

impl PrefixParselet for PrefixOperationParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expression, Error> {
        let operand = parser.parse_expression(self.precedence)?;
        let kind = match &token.1 {
            TokenKind::Plus => return Ok(operand),
            TokenKind::Minus => ExpressionKind::Unary { operation: UnaryOperation::Minus, expression: Box::new(operand) },
            TokenKind::Not => ExpressionKind::Unary{ operation: UnaryOperation::Not, expression: Box::new(operand) },
            _ => unreachable!(),
        };

        Ok((token.0, kind))
    }
}

pub struct IfParselet;
impl PrefixParselet for IfParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expression, Error> {
        let condition = parser.parse_expression(0)?;
        parser.expect(TokenKind::Then)?;
        let then_branch = parser.parse_expression(0)?;
        parser.expect(TokenKind::Else)?;
        let else_branch = parser.parse_expression(0)?;
        Ok((token.0, ExpressionKind::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }))
    }
}