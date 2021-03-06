use crate::{error::{Error, Span}, parser::{ast::{Function, Parameter, Program}, expression::{
            Binary, BinaryOperation, Expression, ExpressionKind, Identifier, If, Let, Unary,
            UnaryOperation,
        }, typed_ast::{TypedParameter, TypedFunction, TypedProgram}, typed_expression::{TypedBinary, TypedExpression, TypedExpressionKind, TypedIdentifier, TypedIf, TypedLet, TypedUnary}}};

use super::types::Type;

pub trait TypeCheck<'a> {
    type Output;
    type Error;

    // TODO: Implement environment.
    fn check(self) -> Result<Self::Output, Self::Error>;
}

pub trait TypeCheckSpan<'a> {
    type Output;
    type Error;

    // TODO: Implement environment.
    fn check_span(self, span: Span<'a>) -> Result<Self::Output, Self::Error>;
}

impl<'a, T: TypeCheck<'a>> TypeCheck<'a> for Vec<T> {
    type Output = Vec<T::Output>;
    type Error = Vec<T::Error>;

    fn check(self) -> Result<Self::Output, Self::Error> {
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
    type Output = TypedProgram<'a>;
    type Error = Vec<Error<'a>>;

    fn check(self) -> Result<Self::Output, Self::Error> {
        Ok(
            TypedProgram {
                functions: self.functions.check()?,
            }
        )
    }
}

impl<'a> TypeCheck<'a> for Function<'a> {
    type Output = TypedFunction<'a>;
    type Error = Error<'a>;

    fn check(self) -> Result<Self::Output, Self::Error> {
        let mut typed_params = vec![];
        for parameter in self.parameters {
            if parameter.ty == Type::Void {
                return Err(Error::IllegalType(parameter.span));
            } else {
                typed_params.push(TypedParameter::new(parameter.span, parameter.ty, parameter.name));
            }
        }

        // TODO: Add parameters to environment.
        let typed_body = self.body.check()?;
        let return_type = get_type(&typed_body.1);
        let typed_function = TypedFunction::new(self.span, self.name, typed_params, typed_body, return_type);
        Ok(typed_function)
    }
}

impl<'a> TypeCheck<'a> for Parameter<'a> {
    type Output = TypedParameter<'a>;
    type Error = Error<'a>;

    fn check(self) -> Result<Self::Output, Self::Error> {
        Ok(TypedParameter::new(self.span, self.ty, self.name))
    }
}

impl<'a> TypeCheck<'a> for Expression<'a> {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check(self) -> Result<Self::Output, Self::Error> {
        match self.1 {
            ExpressionKind::Int(value) => Ok((self.0, TypedExpressionKind::Int(value))),
            ExpressionKind::Float(value) => Ok((self.0, TypedExpressionKind::Float(value))),
            ExpressionKind::Boolean(value) => Ok((self.0, TypedExpressionKind::Boolean(value))),
            ExpressionKind::String(id) => Ok((self.0, TypedExpressionKind::String(id))),
            ExpressionKind::Identifier(inner) => inner.check(),
            ExpressionKind::Unary(inner) => inner.check_span(self.0),
            ExpressionKind::Binary(inner) => inner.check_span(self.0),
            ExpressionKind::If(inner) => inner.check_span(self.0),
            ExpressionKind::Let(inner) => inner.check_span(self.0),
        }
    }
}

impl<'a> TypeCheck<'a> for Identifier {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check(self) -> Result<Self::Output, Self::Error> {
        // TODO: Check id of identifier from environment.
        todo!()
    }
}

impl<'a> TypeCheckSpan<'a> for Unary<'a> {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check_span(self, span: Span<'a>) -> Result<Self::Output, Self::Error> {
        let typed_expression = self.expression.check()?;
        let expression_type = get_type(&typed_expression.1);
        let operation_ty = match (self.operation, expression_type) {
            (UnaryOperation::Plus, Type::Int) => Some(Type::Int),
            (UnaryOperation::Plus, Type::Float) => Some(Type::Float),
            (UnaryOperation::Minus, Type::Int) => Some(Type::Int),
            (UnaryOperation::Minus, Type::Float) => Some(Type::Float),
            (UnaryOperation::Not, Type::Boolean) => Some(Type::Boolean),
            _ => None,
        };

        if let Some(operation_ty) = operation_ty {
            Ok((
                span,
                TypedExpressionKind::Unary(TypedUnary {
                    operation: self.operation,
                    expression: Box::new(typed_expression),
                    ty: operation_ty,
                })
            ))
        } else {
            let error = Error::UnsupportedOperation {
                operation_span: span,
                operands: vec![(typed_expression.0, expression_type)],
            };

            Err(error)
        }
    }
}

impl<'a> TypeCheckSpan<'a> for Binary<'a> {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check_span(self, span: Span<'a>) -> Result<Self::Output, Self::Error> {
        let typed_left = self.left.check()?;
        let typed_right = self.right.check()?;
        let left_type = get_type(&typed_left.1);
        let right_type = get_type(&typed_right.1);
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
            Ok((
                span,
                TypedExpressionKind::Binary(TypedBinary {
                    operation: self.operation,
                    left: Box::new(typed_left),
                    right: Box::new(typed_right),
                    ty: result_type,
                })
            ))
        } else {
            let error = Error::UnsupportedOperation {
                operation_span: span,
                operands: vec![(typed_left.0, left_type), (typed_right.0, right_type)],
            };

            Err(error)
        }
    }
}

impl<'a> TypeCheckSpan<'a> for If<'a> {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check_span(self, span: Span<'a>) -> Result<Self::Output, Self::Error> {
        let typed_condition = self.condition.check()?;
        let condition_type = get_type(&typed_condition.1);
        if condition_type != Type::Boolean {
            return Err(Error::TypeMismatch {
                span: typed_condition.0,
                expected_type: Type::Boolean,
                actual_type: condition_type,
            });
        }

        let typed_then = self.then_branch.check()?;
        let then_type = get_type(&typed_then.1);
        if let Some(else_branch) = self.else_branch {
            let typed_else = else_branch.check()?;
            let else_type = get_type(&typed_else.1);
            
            if then_type == else_type {
                Ok((
                    span,
                    TypedExpressionKind::If(TypedIf {
                        condition: Box::new(typed_condition),
                        then_branch: Box::new(typed_then),
                        else_branch: Some(Box::new(typed_else)),
                        ty: then_type,
                    })
                ))
            } else {
                Err(Error::ConflictingType {
                    first_span: typed_then.0,
                    first_type: then_type,
                    second_span: typed_else.0,
                    second_type: else_type,
                })
            }
        } else {
            Ok((
                span,
                TypedExpressionKind::If(TypedIf {
                    condition: Box::new(typed_condition),
                    then_branch: Box::new(typed_then),
                    else_branch: None,
                    ty: Type::Void,
                })
            ))
        }
    }
}

impl<'a> TypeCheckSpan<'a> for Let<'a> {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check_span(self, span: Span<'a>) -> Result<Self::Output, Self::Error> {
        let typed_expression = self.expression.check()?;
        let expression_type = get_type(&typed_expression.1);
        if let Some(given_type) = self.given_type {
            if expression_type != given_type {
                return Err(Error::ConflictingType {
                    first_span: self.name.0,
                    first_type: given_type,
                    second_span: typed_expression.0,
                    second_type: expression_type,
                });
            }
        }

        let typed_name = (
            self.name.0,
            TypedIdentifier {
                id: self.name.1.0,
                ty: expression_type,
            }
        );

        Ok((
            span,
            TypedExpressionKind::Let(TypedLet {
                name: typed_name,
                given_type: self.given_type,
                expression: Box::new(typed_expression),
                ty: expression_type,
            })
        ))
    }
}

fn get_type(typed_expression_kind: &TypedExpressionKind) -> Type {
    match typed_expression_kind {
        TypedExpressionKind::Int(_) => Type::Int,
        TypedExpressionKind::Float(_) => Type::Float,
        TypedExpressionKind::Boolean(_) => Type::Boolean,
        TypedExpressionKind::String(_) => Type::String,
        TypedExpressionKind::Identifier(ref inner) => inner.ty,
        TypedExpressionKind::Unary(ref inner) => inner.ty,
        TypedExpressionKind::Binary(ref inner) => inner.ty,
        TypedExpressionKind::If(ref inner) => inner.ty,
        TypedExpressionKind::Let(ref inner) => inner.ty,
    }
}