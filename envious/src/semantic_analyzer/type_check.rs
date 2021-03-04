use crate::{
    error::{Error, Span},
    parser::{
        ast::{Function, Parameter, Program},
        expression::{
            Binary, BinaryOperation, Expression, ExpressionKind, Identifier, If, Let, Unary,
            UnaryOperation,
        },
    },
};

use super::types::Type;

pub trait TypeCheck<'a> {
    type Output;
    type Error;

    // TODO: Implement environment.
    fn check(&mut self) -> Result<Self::Output, Self::Error>;
}

pub trait TypeCheckSpan<'a> {
    type Output;
    type Error;

    // TODO: Implement environment.
    fn check_span(&mut self, span: Span<'a>) -> Result<Self::Output, Self::Error>;
}

impl<'a, T: TypeCheck<'a>> TypeCheck<'a> for Vec<T> {
    type Output = Vec<T::Output>;
    type Error = Vec<T::Error>;

    fn check(&mut self) -> Result<Self::Output, Self::Error> {
        let mut results = vec![];
        let mut errors = vec![];
        for value in self {
            match value.check() {
                Ok(result) => results.push(result),
                Err(error) => errors.push(error),
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(results)
        }
    }
}

impl<'a> TypeCheck<'a> for Program<'a> {
    type Output = ();
    type Error = Vec<Error<'a>>;

    fn check(&mut self) -> Result<Self::Output, Self::Error> {
        self.functions.check().map(|_| ())
    }
}

impl<'a> TypeCheck<'a> for Function<'a> {
    type Output = Type;
    type Error = Error<'a>;

    fn check(&mut self) -> Result<Self::Output, Self::Error> {
        for parameter in &self.parameters {
            if parameter.ty == Type::Void {
                return Err(Error::IllegalType(parameter.span));
            }
        }

        // TODO: Add parameters to environment.
        let return_type = self.body.check()?;
        self.return_type = Some(return_type);
        Ok(return_type)
    }
}

impl<'a> TypeCheck<'a> for Parameter<'a> {
    type Output = Type;
    type Error = Error<'a>;

    fn check(&mut self) -> Result<Self::Output, Self::Error> {
        Ok(self.ty)
    }
}

impl<'a> TypeCheck<'a> for Expression<'a> {
    type Output = Type;
    type Error = Error<'a>;

    fn check(&mut self) -> Result<Self::Output, Self::Error> {
        match self.1 {
            ExpressionKind::Int(_) => Ok(Type::Int),
            ExpressionKind::Float(_) => Ok(Type::Float),
            ExpressionKind::Boolean(_) => Ok(Type::Boolean),
            ExpressionKind::String(_) => Ok(Type::String),
            ExpressionKind::Identifier(ref mut inner) => inner.check(),
            ExpressionKind::Unary(ref mut inner) => inner.check_span(self.0),
            ExpressionKind::Binary(ref mut inner) => inner.check_span(self.0),
            ExpressionKind::If(ref mut inner) => inner.check(),
            ExpressionKind::Let(ref mut inner) => inner.check(),
        }
    }
}

impl<'a> TypeCheck<'a> for Identifier {
    type Output = Type;
    type Error = Error<'a>;

    fn check(&mut self) -> Result<Self::Output, Self::Error> {
        // TODO: Check id of identifier from environment.
        todo!()
    }
}

impl<'a> TypeCheckSpan<'a> for Unary<'a> {
    type Output = Type;
    type Error = Error<'a>;

    fn check_span(&mut self, span: Span<'a>) -> Result<Self::Output, Self::Error> {
        let expression_type = self.expression.check()?;
        let valid_op = match self.operation {
            UnaryOperation::Plus => matches!(expression_type, Type::Int | Type::Float),
            UnaryOperation::Minus => matches!(expression_type, Type::Int | Type::Float),
            UnaryOperation::Not => matches!(expression_type, Type::Boolean),
        };

        if valid_op {
            Ok(expression_type)
        } else {
            let error = Error::UnsupportedOperation {
                operation_span: span,
                operands: vec![(self.expression.0, expression_type)],
            };

            Err(error)
        }
    }
}

impl<'a> TypeCheckSpan<'a> for Binary<'a> {
    type Output = Type;
    type Error = Error<'a>;

    fn check_span(&mut self, span: Span<'a>) -> Result<Self::Output, Self::Error> {
        let left_type = self.left.check()?;
        let right_type = self.right.check()?;
        let result_type = match (self.operation, left_type, right_type) {
            (BinaryOperation::Plus, Type::Int, Type::Int) => Some(Type::Int),
            (BinaryOperation::Plus, Type::Float, Type::Float) => Some(Type::Float),
            (BinaryOperation::Plus, Type::String, Type::String) => Some(Type::String),

            (BinaryOperation::Minus, Type::Int, Type::Int) => Some(Type::Int),
            (BinaryOperation::Minus, Type::Float, Type::Float) => Some(Type::Float),
            (BinaryOperation::Minus, Type::String, Type::String) => Some(Type::String),

            (BinaryOperation::Multiply, Type::Int, Type::Int) => Some(Type::Int),
            (BinaryOperation::Multiply, Type::Float, Type::Float) => Some(Type::Float),

            (BinaryOperation::Divide, Type::Int, Type::Int) => Some(Type::Int),
            (BinaryOperation::Divide, Type::Float, Type::Float) => Some(Type::Float),
            _ => None,
        };

        if let Some(result_type) = result_type {
            Ok(result_type)
        } else {
            let error = Error::UnsupportedOperation {
                operation_span: span,
                operands: vec![
                    (self.left.0, left_type),
                    (self.right.0, right_type),
                ],
            };

            Err(error)
        }
    }
}

impl<'a> TypeCheck<'a> for If<'a> {
    type Output = Type;
    type Error = Error<'a>;

    fn check(&mut self) -> Result<Self::Output, Self::Error> {
        let condition_type = self.condition.check()?;
        if condition_type != Type::Boolean {
            return Err(Error::TypeMismatch {
                span: self.condition.0,
                expected_type: Type::Boolean,
                actual_type: condition_type,
            });
        }

        if let Some(ref mut else_branch) = self.else_branch {
            let then_branch_type = self.then_branch.check()?;
            let else_branch_type = else_branch.check()?;

            if then_branch_type == else_branch_type {
                Ok(then_branch_type)
            } else {
                Err(Error::ConflictingType {
                    first_span: self.then_branch.0,
                    first_type: then_branch_type,
                    second_span: else_branch.0,
                    second_type: else_branch_type,
                })
            }
        } else {
            Ok(Type::Void)
        }
    }
}

impl<'a> TypeCheck<'a> for Let<'a> {
    type Output = Type;
    type Error = Error<'a>;

    fn check(&mut self) -> Result<Self::Output, Self::Error> {
        if let Some(given_type) = self.given_type {
            let actual_type = self.expression.check()?;
            if actual_type != given_type {
                return Err(Error::ConflictingType {
                    first_span: self.name.0,
                    first_type: given_type,
                    second_span: self.expression.0,
                    second_type: actual_type,
                });
            }
        }

        Ok(Type::Void)
    }
}
