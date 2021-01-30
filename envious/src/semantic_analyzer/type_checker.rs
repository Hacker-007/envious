use crate::{
    error::{Error, Span},
    interner::Interner,
    parser::expression::{BinaryOperation, Expression, ExpressionKind, UnaryOperation},
};

use super::{caster::Caster, types::Type};

pub struct TypeChecker;

impl TypeChecker {
    pub fn analyze(
        interner: &mut Interner<String>,
        expression: &mut Expression,
    ) -> Result<Type, Error> {
        match expression.1 {
            ExpressionKind::Int(_) => Ok(Type::Int),
            ExpressionKind::Float(_) => Ok(Type::Float),
            ExpressionKind::Boolean(_) => Ok(Type::Boolean),
            ExpressionKind::String(_) => Ok(Type::String),
            ExpressionKind::Identifier(_) => {
                // This is temporary. This will be changed when I add variables.
                Ok(Type::String)
            }
            ExpressionKind::Unary {
                ref operation,
                expression: ref mut sub_expression,
            } => TypeChecker::analyze_unary_expression(
                interner,
                &expression.0,
                operation,
                sub_expression,
            ),
            ExpressionKind::Binary {
                ref operation,
                ref mut left,
                ref mut right,
            } => TypeChecker::analyze_binary_expression(
                interner,
                &expression.0,
                operation,
                left,
                right,
            ),
            ExpressionKind::If {
                ref mut condition,
                ref mut then_branch,
                ref mut else_branch,
            } => TypeChecker::analyze_if_expression(
                interner,
                condition,
                then_branch,
                else_branch.as_mut(),
            ),
        }
    }

    fn analyze_unary_expression(
        interner: &mut Interner<String>,
        operation_span: &Span,
        operation: &UnaryOperation,
        expression: &mut Expression,
    ) -> Result<Type, Error> {
        let expression_type = TypeChecker::analyze(interner, expression)?;
        match operation {
            UnaryOperation::Minus => {
                if matches!(expression_type, Type::Int | Type::Float) {
                    Ok(expression_type)
                } else {
                    let error = Error::UnsupportedOperation {
                        operation_span: operation_span.clone(),
                        operands: vec![(expression.0.clone(), expression_type)],
                    };

                    Err(error)
                }
            }
            UnaryOperation::Not => {
                if matches!(expression_type, Type::Boolean) {
                    Ok(expression_type)
                } else {
                    let error = Error::UnsupportedOperation {
                        operation_span: operation_span.clone(),
                        operands: vec![(expression.0.clone(), expression_type)],
                    };

                    Err(error)
                }
            }
        }
    }

    fn analyze_binary_expression(
        interner: &mut Interner<String>,
        operation_span: &Span,
        operation: &BinaryOperation,
        left: &mut Expression,
        right: &mut Expression,
    ) -> Result<Type, Error> {
        let left_type = TypeChecker::analyze(interner, left)?;
        let right_type = TypeChecker::analyze(interner, right)?;
        match operation {
            BinaryOperation::Plus => match (left_type, right_type) {
                (Type::Int, Type::Int) => Ok(Type::Int),
                (Type::Float, Type::Float) => Ok(Type::Float),
                (Type::String, Type::String) => Ok(Type::String),
                (Type::Int, Type::Float) => {
                    if let Some(error) = Caster::cast(interner, left, Type::Int, Type::Float) {
                        Err(error)
                    } else {
                        Ok(Type::Float)
                    }
                }
                (Type::Float, Type::Int) => {
                    if let Some(error) = Caster::cast(interner, right, Type::Int, Type::Float) {
                        Err(error)
                    } else {
                        Ok(Type::Float)
                    }
                }
                (Type::String, _) => {
                    if let Some(error) = Caster::cast(interner, right, right_type, Type::String) {
                        Err(error)
                    } else {
                        Ok(Type::String)
                    }
                }
                (_, Type::String) => {
                    if let Some(error) = Caster::cast(interner, left, left_type, Type::String) {
                        Err(error)
                    } else {
                        Ok(Type::String)
                    }
                }
                _ => {
                    let error = Error::UnsupportedOperation {
                        operation_span: operation_span.clone(),
                        operands: vec![(left.0.clone(), left_type), (right.0.clone(), right_type)],
                    };

                    Err(error)
                }
            },
            BinaryOperation::Minus => match (left_type, right_type) {
                (Type::Int, Type::Int) => Ok(Type::Int),
                (Type::Float, Type::Float) => Ok(Type::Float),
                (Type::String, Type::String) => Ok(Type::String),
                (Type::Int, Type::Float) => {
                    if let Some(error) = Caster::cast(interner, left, Type::Int, Type::Float) {
                        Err(error)
                    } else {
                        Ok(Type::Float)
                    }
                }
                (Type::Float, Type::Int) => {
                    if let Some(error) = Caster::cast(interner, right, Type::Int, Type::Float) {
                        Err(error)
                    } else {
                        Ok(Type::Float)
                    }
                }
                (Type::String, _) => {
                    if let Some(error) = Caster::cast(interner, right, right_type, Type::String) {
                        Err(error)
                    } else {
                        Ok(Type::String)
                    }
                }
                (_, Type::String) => {
                    if let Some(error) = Caster::cast(interner, left, left_type, Type::String) {
                        Err(error)
                    } else {
                        Ok(Type::String)
                    }
                }
                _ => {
                    let error = Error::UnsupportedOperation {
                        operation_span: operation_span.clone(),
                        operands: vec![(left.0.clone(), left_type), (right.0.clone(), right_type)],
                    };

                    Err(error)
                }
            },
            BinaryOperation::Multiply => match (left_type, right_type) {
                (Type::Int, Type::Int) => Ok(Type::Int),
                (Type::Float, Type::Float) => Ok(Type::Float),
                (Type::Int, Type::Float) => {
                    if let Some(error) = Caster::cast(interner, left, Type::Int, Type::Float) {
                        Err(error)
                    } else {
                        Ok(Type::Float)
                    }
                }
                (Type::Float, Type::Int) => {
                    if let Some(error) = Caster::cast(interner, right, Type::Int, Type::Float) {
                        Err(error)
                    } else {
                        Ok(Type::Float)
                    }
                }
                _ => {
                    let error = Error::UnsupportedOperation {
                        operation_span: operation_span.clone(),
                        operands: vec![(left.0.clone(), left_type), (right.0.clone(), right_type)],
                    };

                    Err(error)
                }
            },
            BinaryOperation::Divide => match (left_type, right_type) {
                (Type::Int, Type::Int) => Ok(Type::Int),
                (Type::Float, Type::Float) => Ok(Type::Float),
                (Type::Int, Type::Float) => {
                    if let Some(error) = Caster::cast(interner, left, Type::Int, Type::Float) {
                        Err(error)
                    } else {
                        Ok(Type::Float)
                    }
                }
                (Type::Float, Type::Int) => {
                    if let Some(error) = Caster::cast(interner, right, Type::Int, Type::Float) {
                        Err(error)
                    } else {
                        Ok(Type::Float)
                    }
                }
                _ => {
                    let error = Error::UnsupportedOperation {
                        operation_span: operation_span.clone(),
                        operands: vec![(left.0.clone(), left_type), (right.0.clone(), right_type)],
                    };

                    Err(error)
                }
            },
        }
    }

    fn analyze_if_expression(
        interner: &mut Interner<String>,
        condition: &mut Expression,
        then_branch: &mut Expression,
        else_branch: Option<&mut Box<Expression>>,
    ) -> Result<Type, Error> {
        let condition_type = TypeChecker::analyze(interner, condition)?;
        if condition_type != Type::Boolean {
            let error = Error::TypeMismatch {
                span: condition.0.clone(),
                expected_type: Type::Boolean,
                actual_type: condition_type,
            };

            return Err(error);
        }

        let then_type = TypeChecker::analyze(interner, then_branch)?;
        if let Some(else_branch) = else_branch {
            let else_type = TypeChecker::analyze(interner, else_branch)?;
            if then_type == else_type {
                Ok(then_type)
            } else {
                let error = Error::ConflictingType {
                    first_span: then_branch.0.clone(),
                    first_type: then_type,
                    second_span: else_branch.0.clone(),
                    second_type: else_type,
                };

                Err(error)
            }
        } else {
            Ok(then_type)
        }
    }
}
