//! The TypeChecker struct ensures that all of the types specified match up.
//! It relies on the Types enum to specify the types of the expressions.

use crate::std::function::Function;
use std::collections::HashMap;
use crate::{ast::{expression_kind::{BinaryOperation, ExpressionKind}, expression::Expression}, errors::{error_kind::ErrorKind, error::Error}};
use super::types::Types;
use crate::std::standard_library::StandardLibrary;

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

pub struct TypeChecker {
    pub user_defined_functions: HashMap<String, Function>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            user_defined_functions: HashMap::new(),
        }
    }

    pub fn perform_type_checking(&mut self, expressions: &[Expression], standard_library: &StandardLibrary) -> Result<(), Error> {
        for expression in expressions {
            self.check_types(expression, standard_library)?;
        }
        
        Ok(())
    }

    pub fn check_types(&self, expression: &Expression, standard_library: &StandardLibrary) -> Result<Option<Types>, Error> {
        match &expression.kind {
            ExpressionKind::Int(_) => Ok(Some(Types::Int)),
            ExpressionKind::Float(_) => Ok(Some(Types::Float)),
            ExpressionKind::Boolean(_) => Ok(Some(Types::Boolean)),
            ExpressionKind::String(_) => Ok(Some(Types::String)),
            ExpressionKind::Identifier(_, ident_type) => Ok(ident_type.clone()),
            ExpressionKind::InfixBinaryExpression(operation, left, right) => {
                match (self.check_types(left, standard_library)?, self.check_types(right, standard_library)?) {
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
                match self.check_types(sub_expression, standard_library)? {
                    Some(Types::Int) => Ok(Some(Types::Int)),
                    Some(Types::Float) => Ok(Some(Types::Float)),
                    Some(evaluated_type) => Err(Error::new(ErrorKind::TypeMismatch("An Int Or A Float".to_owned(), evaluated_type.into()), sub_expression.pos)),
                    None => Err(Error::new(ErrorKind::Expected("An Int Or A Float".to_owned()), sub_expression.pos)),
                }
            }
            ExpressionKind::BinaryEqualityExpression(_, left, right) => {
                if self.check_types(left, standard_library)?.is_none() {
                    Err(Error::new(ErrorKind::Expected("An Expression".to_owned()), left.pos))
                } else if self.check_types(right, standard_library)?.is_none() {
                    Err(Error::new(ErrorKind::Expected("An Expression".to_owned()), right.pos))
                } else {
                    Ok(Some(Types::Boolean))
                }
            }
            ExpressionKind::ParenthesizedExpression(expression) => self.check_types(expression, standard_library),
            ExpressionKind::LetExpression(_, var_type, value) => {
                match (var_type, value) {
                    (defined_type, Some(expr)) => {
                        match self.check_types(expr, standard_library)? {
                            None => Err(Error::new(ErrorKind::Expected((*defined_type).into()), expr.pos)),
                            Some(actual_type) if *defined_type == Types::Any || actual_type == *defined_type => Ok(None),
                            Some(actual_type) => Err(Error::new(ErrorKind::TypeMismatch((*defined_type).into(), actual_type.into()), expr.pos)),
                        }
                    },
                    _ => Ok(None),
                }
            }
            ExpressionKind::FunctionCallExpression(name, parameters) => {
                let function = {
                    if let Some(function) = self.user_defined_functions.get(name) {
                        function
                    } else {
                        standard_library.get_function(expression.pos, &name)?
                    }
                };

                if function.number_of_args != parameters.len() {
                    Err(Error::new(ErrorKind::WrongNumberOfParameters, expression.pos))
                } else {
                    for (expected_type, parameter) in function.parameter_types.iter().zip(parameters.iter()) {
                        let parameter_type = self.check_types(parameter, standard_library)?.unwrap_or(Types::Void);
                        if *expected_type == Types::Any && parameter_type != Types::Void {
                            continue;
                        }

                        if parameter_type != *expected_type {
                            return Err(Error::new(ErrorKind::TypeMismatch((*expected_type).into(), parameter_type.into()), parameter.pos))
                        }
                    }
    
                    Ok(
                        match function.return_type {
                            Types::Void => None,
                            return_type => Some(return_type),
                        }
                    )
                }
            }
            ExpressionKind::BlockExpression(expressions) => {
                expressions.iter()
                .fold(
                    Ok(None),
                    |_, expression| Ok(self.check_types(expression, standard_library)?)
                )
            }
            ExpressionKind::IfExpression(condition, expression) => {
                match self.check_types(condition, standard_library)? {
                    None => Err(Error::new(ErrorKind::TypeMismatch(Types::Boolean.into(), Types::Void.into()), condition.pos)),
                    Some(condition_type) if condition_type != Types::Boolean => Err(Error::new(ErrorKind::TypeMismatch(Types::Boolean.into(), condition_type.into()), condition.pos)),
                    Some(_) => self.check_types(expression, standard_library),
                }
            }
            ExpressionKind::DefineExpression(_, _, expression, return_type) => {
                if let Some(returned_type) = self.check_types(expression, standard_library)? {
                    if return_type == &returned_type {
                        Ok(Some(returned_type))
                    } else {
                        Err(Error::new(
                            ErrorKind::TypeMismatch((*return_type).into(), returned_type.into()),
                            expression.pos,
                        ))
                    }
                } else {
                    if return_type == &Types::Void {
                        Ok(None)
                    } else {
                        Err(Error::new(
                            ErrorKind::TypeMismatch((*return_type).into(), Types::Void.into()),
                            expression.pos,
                        ))
                    }
                }
            },
        }
    }
}