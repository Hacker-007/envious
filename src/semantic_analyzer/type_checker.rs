//! The TypeChecker struct ensures that all of the types specified match up.
//! It relies on the Types enum to specify the types of the expressions.

use crate::{ast::{expression_kind::ExpressionKind, expression::Expression}, errors::{error_kind::ErrorKind, error::Error}};
use super::types::Types;

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
            // ExpressionKind::InfixBinaryExpression(_, left, right) => {
            //     if TypeChecker::check_types(left)?.is_none() {
            //         Err(Error::new(ErrorKind::Expected("An Expression".to_owned()), left.pos))
            //     } else if TypeChecker::check_types(right)?.is_none() {
            //         Err(Error::new(ErrorKind::Expected("An Expression".to_owned()), right.pos))
            //     } else {
            //         Ok(Some(Types::Boolean))
            //     }
            // }
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
            _ => todo!(),
            // ExpressionKind::LetExpression(_, _, _) => {}
            // ExpressionKind::FunctionCallExpression(_, _) => {}
            // ExpressionKind::BlockExpression(_) => {}
            // ExpressionKind::IfExpression(_, _) => {}
        }
    }
}