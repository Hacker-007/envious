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

pub struct Caster;

impl Caster {
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
        }

        None
    }
}
