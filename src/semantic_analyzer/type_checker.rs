//! The TypeChecker struct ensures that all of the types specified match up.
//! It relies on the Types enum to specify the types of the expressions.

use crate::{ast::{expression_kind::{BinaryOperation, ExpressionKind}, expression::Expression}, errors::{error_kind::ErrorKind, error::Error}};
use super::types::Types;

macro_rules! allowed_binary_types {
    ($operation: tt, $pos: expr, $left: ident, $right: ident, $(($first: pat, $second: pat => $result: path)),+) => {
        match ($left, $right) {
            $(
                ($first, $second) => Ok(Some($result)),
            )+
            _ => Err(Error::new(ErrorKind::UnsupportedOperation($operation.to_owned(), vec![$left.into(), $right.into()]), $pos)),
        }
    };
}

pub struct TypeChecker;

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker
    }

    pub fn perform_type_checking(&self, expressions: &[Expression]) -> Result<(), Error> {
        for expression in expressions {
            TypeChecker::check_types(expression)?;
        }
        
        Ok(())
    }

    pub fn check_types(expression: &Expression) -> Result<Option<Types>, Error> {
        match &expression.kind {
            ExpressionKind::Int(_) => Ok(Some(Types::Int)),
            ExpressionKind::Float(_) => Ok(Some(Types::Float)),
            ExpressionKind::Boolean(_) => Ok(Some(Types::Boolean)),
            ExpressionKind::String(_) => Ok(Some(Types::String)),
            ExpressionKind::Identifier(_, ident_type) => Ok(ident_type.clone()),
            ExpressionKind::InfixBinaryExpression(operation, left, right) => {
                match (TypeChecker::check_types(left)?, TypeChecker::check_types(right)?) {
                    (None, _) => Err(Error::new(ErrorKind::Expected("An Expression".to_owned()), left.pos)),
                    (_, None) => Err(Error::new(ErrorKind::Expected("An Expression".to_owned()), right.pos)),
                    (Some(left_type), Some(right_type)) => {
                        match operation {
                            BinaryOperation::Plus => {
                                allowed_binary_types!(
                                    "Plus",
                                    expression.pos,
                                    left_type,
                                    right_type,
                                    (Types::Int, Types::Int => Types::Int),
                                    (Types::Int, Types::Float => Types::Float),
                                    (Types::Float, Types::Int => Types::Float),
                                    (Types::Float, Types::Float => Types::Float),
                                    (_, Types::String => Types::String),
                                    (Types::String, _ => Types::String)
                                )
                            }
                            BinaryOperation::Minus => {
                                allowed_binary_types!(
                                    "Minus",
                                    expression.pos,
                                    left_type,
                                    right_type,
                                    (Types::Int, Types::Int => Types::Int),
                                    (Types::Int, Types::Float => Types::Float),
                                    (Types::Float, Types::Int => Types::Float),
                                    (Types::Float, Types::Float => Types::Float)
                                )
                            }
                            BinaryOperation::Multiply => {
                                allowed_binary_types!(
                                    "Multiply",
                                    expression.pos,
                                    left_type,
                                    right_type,
                                    (Types::Int, Types::Int => Types::Int),
                                    (Types::Int, Types::Float => Types::Float),
                                    (Types::Float, Types::Int => Types::Float),
                                    (Types::Float, Types::Float => Types::Float),
                                    (Types::Int, Types::String => Types::String),
                                    (Types::String, Types::Int => Types::String)
                                )
                            }
                            BinaryOperation::Divide => {
                                allowed_binary_types!(
                                    "Divide",
                                    expression.pos,
                                    left_type,
                                    right_type,
                                    (Types::Int, Types::Int => Types::Int),
                                    (Types::Int, Types::Float => Types::Float),
                                    (Types::Float, Types::Int => Types::Float),
                                    (Types::Float, Types::Float => Types::Float)
                                )
                            }
                        }
                    }
                }
            }
            ExpressionKind::UnaryExpression(_, sub_expression) => {
                match TypeChecker::check_types(sub_expression)? {
                    Some(Types::Int) => Ok(Some(Types::Int)),
                    Some(Types::Float) => Ok(Some(Types::Float)),
                    Some(evaluated_type) => Err(Error::new(ErrorKind::TypeMismatch("An Int Or A Float".to_owned(), evaluated_type.into()), sub_expression.pos)),
                    None => Err(Error::new(ErrorKind::Expected("An Int Or A Float".to_owned()), sub_expression.pos)),
                }
            }
            ExpressionKind::BinaryEqualityExpression(_, left, right) => {
                if TypeChecker::check_types(left)?.is_none() {
                    Err(Error::new(ErrorKind::Expected("An Expression".to_owned()), left.pos))
                } else if TypeChecker::check_types(right)?.is_none() {
                    Err(Error::new(ErrorKind::Expected("An Expression".to_owned()), right.pos))
                } else {
                    Ok(Some(Types::Boolean))
                }
            }
            ExpressionKind::ParenthesizedExpression(expression) => TypeChecker::check_types(expression),
            ExpressionKind::LetExpression(_, var_type, value) => {
                match (var_type, value) {
                    (defined_type, Some(expr)) => {
                        match TypeChecker::check_types(expr)? {
                            None => Err(Error::new(ErrorKind::Expected((*defined_type).into()), expr.pos)),
                            Some(actual_type) if *defined_type == Types::Any || actual_type == *defined_type => Ok(None),
                            Some(actual_type) => Err(Error::new(ErrorKind::TypeMismatch((*defined_type).into(), actual_type.into()), expr.pos)),
                        }
                    },
                    _ => Ok(None),
                }
            }
            ExpressionKind::BlockExpression(expressions) => {
                expressions.iter()
                .fold(
                    Ok(None),
                    |_, expression| TypeChecker::check_types(expression)
                )
            }
            ExpressionKind::IfExpression(condition, expression) => {
                match TypeChecker::check_types(condition)? {
                    None => Err(Error::new(ErrorKind::TypeMismatch(Types::Boolean.into(), Types::Void.into()), condition.pos)),
                    Some(condition_type) if condition_type != Types::Boolean => Err(Error::new(ErrorKind::TypeMismatch(Types::Boolean.into(), condition_type.into()), condition.pos)),
                    Some(_) => TypeChecker::check_types(expression),
                }
            }
            // ExpressionKind::FunctionCallExpression(_, _) => {}
            _ => todo!(),
        }
    }
}