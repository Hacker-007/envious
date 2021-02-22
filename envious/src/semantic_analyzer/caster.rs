use crate::{
    error::Error,
    interner::Interner,
    parser::expression::{Expression, ExpressionKind},
};

use super::types::Type;

macro_rules! internal_cast {
    ($kind: ident, $pattern: pat, $expression: expr) => {
        if let $pattern = $kind {
            $expression
        } else {
            unreachable!()
        };
    };
}

/// Struct that handles implicit casting between two types.
pub struct Caster;

impl Caster {
    /// Casts the given expression from the starting_type to the
    /// final_type. This operation modifies the given expression
    /// in-place as opposed to creating a new expression.
    ///
    /// # Arguments
    /// * `interner` - The `Interner` used to store all string literals.
    /// * `expression` - The `Expression` to cast.
    /// * `starting_type` - The starting type of the `Expression`.
    /// * `final_type` - The final type of the `Expression`.
    pub fn cast(
        interner: &mut Interner<String>,
        (span, kind): &mut Expression,
        starting_type: Type,
        final_type: Type,
    ) -> Option<Error> {
        if starting_type == final_type {
            return None;
        }

        match starting_type {
            Type::Int => {
                let new_expression = match final_type {
                    Type::Float => {
                        internal_cast!(
                            kind,
                            ExpressionKind::Int(value),
                            ExpressionKind::Float(*value as f64)
                        )
                    }
                    Type::Boolean => {
                        internal_cast!(
                            kind,
                            ExpressionKind::Int(value),
                            ExpressionKind::Boolean(*value != 0)
                        )
                    }
                    Type::String => {
                        internal_cast!(
                            kind,
                            ExpressionKind::Int(value),
                            ExpressionKind::String(interner.insert(format!("{}", value)))
                        )
                    }
                    _ => unreachable!(),
                };

                *kind = new_expression;
            }
            Type::Float => {
                let new_expression = match final_type {
                    Type::Boolean => {
                        internal_cast!(
                            kind,
                            ExpressionKind::Float(value),
                            ExpressionKind::Boolean(*value != 0.0)
                        )
                    }
                    Type::String => {
                        internal_cast!(
                            kind,
                            ExpressionKind::Float(value),
                            ExpressionKind::String(interner.insert(format!("{}", value)))
                        )
                    }
                    Type::Int => {
                        return Some(Error::IllegalCast {
                            span: span.clone(),
                            from_type: starting_type,
                            to_type: final_type,
                        })
                    }
                    _ => unreachable!(),
                };

                *kind = new_expression;
            }
            Type::Boolean => {
                let new_expression = match final_type {
                    Type::String => {
                        internal_cast!(
                            kind,
                            ExpressionKind::Boolean(value),
                            ExpressionKind::String(interner.insert(format!("{}", value)))
                        )
                    }
                    Type::Int | Type::Float => {
                        return Some(Error::IllegalCast {
                            span: span.clone(),
                            from_type: starting_type,
                            to_type: final_type,
                        })
                    }
                    _ => unreachable!(),
                };

                *kind = new_expression;
            }
            Type::String => {}
            Type::Void => {
                return Some(Error::IllegalCast {
                    span: span.clone(),
                    from_type: Type::Void,
                    to_type: final_type,
                })
            }
            Type::Function { .. } => {
                return Some(Error::IllegalCast {
                    span: span.clone(),
                    from_type: starting_type.clone(),
                    to_type: final_type
                })
            }
        }

        None
    }
}
