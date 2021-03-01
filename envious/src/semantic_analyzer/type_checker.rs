use std::collections::HashMap;

use crate::{
    error::{Error, Span},
    interner::Interner,
    parser::expression::{BinaryOperation, Expression, ExpressionKind, UnaryOperation},
};

use super::{caster::Caster, types::Type};

/// Struct that verifies the types of the expressions
/// and ensures that the types of the program are sound.
pub struct TypeChecker {
    vars: HashMap<usize, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    /// Analyzes the entire program given and ensures that
    /// each expression is valid.
    ///
    /// # Arguments
    /// * `interner` - The `Interner` used to store all string literals.
    /// * `expressions` - The `Expression`'s that constitute the program.
    pub fn analyze_program(
        &mut self,
        interner: &mut Interner<String>,
        program: &mut [Expression],
    ) -> Vec<Error> {
        let mut errors = vec![];
        for expression in program {
            if let Err(error) = self.analyze(interner, expression) {
                errors.push(error);
            }
        }

        errors
    }

    /// Analyzes the expression given and ensures that the
    /// resultant types of the subexpressions match.
    ///
    /// # Arguments
    /// * `interner` - The `Interner` used to store all string literals.
    /// * `expression` - The `Expression` to type check.
    pub fn analyze(
        &mut self,
        interner: &mut Interner<String>,
        expression: &mut Expression,
    ) -> Result<Type, Error> {
        match expression.1 {
            ExpressionKind::Int(_) => Ok(Type::Int),
            ExpressionKind::Float(_) => Ok(Type::Float),
            ExpressionKind::Boolean(_) => Ok(Type::Boolean),
            ExpressionKind::String(_) => Ok(Type::String),
            ExpressionKind::Identifier(id) => {
                if let Some(var_type) = self.vars.get(&id) {
                    Ok(var_type.clone())
                } else {
                    Err(Error::UndefinedVariable(expression.0.clone()))
                }
            }
            ExpressionKind::Unary {
                ref operation,
                expression: ref mut sub_expression,
            } => self.analyze_unary_expression(interner, &expression.0, operation, sub_expression),
            ExpressionKind::Binary {
                ref operation,
                ref mut left,
                ref mut right,
            } => self.analyze_binary_expression(interner, &expression.0, operation, left, right),
            ExpressionKind::If {
                ref mut condition,
                ref mut then_branch,
                ref mut else_branch,
            } => self.analyze_if_expression(interner, condition, then_branch, else_branch.as_mut()),
            ExpressionKind::Let {
                ref name,
                ref given_type,
                expression: ref mut sub_expression,
            } => self.analyze_let_expression(interner, name, given_type, sub_expression),
        }
    }

    /// Analyzes a unary expression for proper types.
    /// This function checks the operator and the operand
    /// to make sure that the operands can be applied to the
    /// given operand.
    ///
    /// # Arguments
    /// * `interner` - The `Interner` used to store all string literals.
    /// * `operation_span` - The span of the operation.
    /// * `operation` - The unary operation used.
    /// * `expression` - The `Expression` that the unary operation was applied to.
    fn analyze_unary_expression(
        &mut self,
        interner: &mut Interner<String>,
        operation_span: &Span,
        operation: &UnaryOperation,
        expression: &mut Expression,
    ) -> Result<Type, Error> {
        let expression_type = self.analyze(interner, expression)?;
        match operation {
            UnaryOperation::Plus => {
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

    /// Analyzes a binary expression for proper types.
    /// This function checks the operator and the operands
    /// to make sure that the operands can be applied to the
    /// given operands.
    ///
    /// # Arguments
    /// * `interner` - The `Interner` used to store all string literals.
    /// * `operation_span` - The span of the operation.
    /// * `operation` - The binary operation used.
    /// * `left` - The left `Expression` that the binary operation was applied to.
    /// * `right` - The right `Expression` that the binary operation was applied to.
    fn analyze_binary_expression(
        &mut self,
        interner: &mut Interner<String>,
        operation_span: &Span,
        operation: &BinaryOperation,
        left: &mut Expression,
        right: &mut Expression,
    ) -> Result<Type, Error> {
        let left_type = self.analyze(interner, left)?;
        let right_type = self.analyze(interner, right)?;
        match operation {
            BinaryOperation::Plus => match (&left_type, &right_type) {
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
            BinaryOperation::Minus => match (&left_type, &right_type) {
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
            BinaryOperation::Multiply => match (&left_type, &right_type) {
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
            BinaryOperation::Divide => match (&left_type, &right_type) {
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

    /// Analyzes an if expression for proper types.
    /// This function checks the condition and the two branches
    /// to make sure that the condition results in a boolean type
    /// and that the two branches match in type.
    ///
    /// # Arguments
    /// * `interner` - The `Interner` used to store all string literals.
    /// * `condition` - The `Expression` used as the condition to the if.
    /// * `then_branch` - The `Expression` used for the then branch of the if.
    /// * `else_branch` - The optional `Expression` used for the else branch of the if.
    fn analyze_if_expression(
        &mut self,
        interner: &mut Interner<String>,
        condition: &mut Expression,
        then_branch: &mut Expression,
        else_branch: Option<&mut Box<Expression>>,
    ) -> Result<Type, Error> {
        let condition_type = self.analyze(interner, condition)?;
        if condition_type != Type::Boolean {
            let error = Error::TypeMismatch {
                span: condition.0.clone(),
                expected_type: Type::Boolean,
                actual_type: condition_type,
            };

            return Err(error);
        }

        let then_type = self.analyze(interner, then_branch)?;
        if let Some(else_branch) = else_branch {
            let else_type = self.analyze(interner, else_branch)?;
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
            Ok(Type::Void)
        }
    }

    /// Analyzes a let expression for proper types.
    /// This function checks the given type of the let expression and
    /// the type of the resulting expression and makes sure
    /// that they are the same.
    ///
    /// # Arguments
    /// * `interner` - The `Interner` used to store all string literals.
    /// * `name` - The id of the identifier used for the name.
    /// * `given_type` - The `Type` assigned to the variable.
    /// * `expression` - The `Expression` or value of this variable.
    fn analyze_let_expression(
        &mut self,
        interner: &mut Interner<String>,
        name: &(Span, usize),
        given_type: &Option<Type>,
        expression: &mut Expression,
    ) -> Result<Type, Error> {
        let value_type = self.analyze(interner, expression)?;
        if let Some(given_type) = given_type {
            if given_type == &value_type {
                self.vars.insert(name.1, value_type.clone());
                Ok(value_type)
            } else {
                Err(Error::ConflictingType {
                    first_span: name.0.clone(),
                    first_type: given_type.clone(),
                    second_span: expression.0.clone(),
                    second_type: value_type,
                })
            }
        } else {
            self.vars.insert(name.1, value_type.clone());
            Ok(value_type)
        }
    }
}
