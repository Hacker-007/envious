use std::rc::Rc;

use crate::{
    parser::expression::{Application, Binary, Expression, ExpressionKind, If, Let, Unary, While},
    type_inference::typed_expression::{
        TypedApplication, TypedBinary, TypedIdentifier, TypedIf, TypedLet, TypedUnary, TypedWhile,
    },
};

use super::{
    context::Context,
    monotype::{Monotype, MonotypeRef},
    typed_expression::{TypedExpression, TypedExpressionKind},
};

#[derive(Debug)]
pub struct TypeInferer {
    next_unknown: usize,
    context: Context,
}

impl<'a> TypeInferer {
    pub fn new() -> Self {
        Self {
            next_unknown: 0,
            context: Context::new(),
        }
    }

    pub fn annotate_expression(&mut self, expression: &Expression<'a>) -> TypedExpression<'a> {
        match &expression.1 {
            &ExpressionKind::Int(value) => (expression.0, TypedExpressionKind::Int(value)),
            &ExpressionKind::Float(value) => (expression.0, TypedExpressionKind::Float(value)),
            &ExpressionKind::Boolean(value) => (expression.0, TypedExpressionKind::Boolean(value)),
            &ExpressionKind::Char(value) => (expression.0, TypedExpressionKind::Char(value)),
            ExpressionKind::Identifier(id) => (
                expression.0,
                TypedExpressionKind::Identifier(TypedIdentifier {
                    id: id.0,
                    ty: self.get_or_existential(id.0),
                }),
            ),
            ExpressionKind::Unary(Unary {
                operation,
                expression: sub_expression,
            }) => (
                expression.0,
                TypedExpressionKind::Unary(TypedUnary {
                    operation: *operation,
                    expression: Box::new(self.annotate_expression(sub_expression)),
                    ty: self.next_existential(),
                }),
            ),
            ExpressionKind::Binary(Binary {
                operation,
                left,
                right,
            }) => (
                expression.0,
                TypedExpressionKind::Binary(TypedBinary {
                    operation: *operation,
                    left: Box::new(self.annotate_expression(left)),
                    right: Box::new(self.annotate_expression(right)),
                    ty: self.next_existential(),
                }),
            ),
            ExpressionKind::If(If {
                condition,
                then_branch,
                else_branch,
            }) => (
                expression.0,
                TypedExpressionKind::If(TypedIf {
                    condition: Box::new(self.annotate_expression(condition)),
                    then_branch: Box::new(self.annotate_expression(then_branch)),
                    else_branch: else_branch
                        .as_ref()
                        .map(|else_branch| Box::new(self.annotate_expression(else_branch))),
                    ty: self.next_existential(),
                }),
            ),
            ExpressionKind::Let(Let {
                name,
                given_type: _,
                expression: sub_expression,
            }) => {
                let ty = self.get_or_existential(name.1 .0);
                (
                    expression.0,
                    TypedExpressionKind::Let(TypedLet {
                        name: (
                            name.0,
                            TypedIdentifier {
                                id: name.1 .0,
                                ty: ty.clone(),
                            },
                        ),
                        given_type: None,
                        expression: Box::new(self.annotate_expression(sub_expression)),
                        ty,
                    }),
                )
            }
            ExpressionKind::Block(expressions) => (
                expression.0,
                TypedExpressionKind::Block(
                    expressions
                        .iter()
                        .map(|expression| self.annotate_expression(expression))
                        .collect(),
                ),
            ),
            ExpressionKind::Application(Application {
                function_name,
                parameters,
            }) => {
                let ty = self.get_or_existential(function_name.1 .0);
                (
                    expression.0,
                    TypedExpressionKind::Application(TypedApplication {
                        function_name: (
                            function_name.0,
                            TypedIdentifier {
                                id: function_name.1 .0,
                                ty: ty.clone(),
                            },
                        ),
                        parameters: parameters
                            .iter()
                            .map(|parameter| self.annotate_expression(parameter))
                            .collect(),
                        ty: self.next_existential(),
                    }),
                )
            }
            ExpressionKind::While(While {
                condition,
                expression: sub_expression,
            }) => (
                expression.0,
                TypedExpressionKind::While(TypedWhile {
                    condition: Box::new(self.annotate_expression(condition)),
                    expression: Box::new(self.annotate_expression(sub_expression)),
                }),
            ),
            ExpressionKind::Return(value) => (
                expression.0,
                TypedExpressionKind::Return(
                    value
                        .as_ref()
                        .map(|expression| Box::new(self.annotate_expression(expression))),
                ),
            ),
        }
    }

    fn get_or_existential(&mut self, id: usize) -> MonotypeRef {
        self.context
            .get(id)
            .unwrap_or_else(|| self.next_existential())
    }

    fn next_existential(&mut self) -> MonotypeRef {
        self.next_unknown += 1;
        Rc::new(Monotype::Existential(self.next_unknown - 1))
    }
}
