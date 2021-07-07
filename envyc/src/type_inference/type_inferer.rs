use std::{collections::HashSet, rc::Rc};

use crate::{
    parser::expression::{
        Application, Binary, BinaryOperation, Expression, ExpressionKind, If, Let, Unary,
        UnaryOperation, While,
    },
    type_inference::typed_expression::{
        TypedApplication, TypedBinary, TypedIdentifier, TypedIf, TypedLet, TypedUnary, TypedWhile,
    },
};

use super::{
    constraints::Constraint,
    context::Context,
    monotype::{Monotype, MonotypeRef},
    substitution::Substitution,
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
            &ExpressionKind::Int(value) => (
                expression.0,
                TypedExpressionKind::Int(self.next_existential(), value),
            ),
            &ExpressionKind::Float(value) => (
                expression.0,
                TypedExpressionKind::Float(self.next_existential(), value),
            ),
            &ExpressionKind::Boolean(value) => (
                expression.0,
                TypedExpressionKind::Boolean(self.next_existential(), value),
            ),
            &ExpressionKind::Char(value) => (
                expression.0,
                TypedExpressionKind::Char(self.next_existential(), value),
            ),
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
                        ty: Rc::new(Monotype::Void),
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
                                ty,
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

    pub fn get_constraints(&self, expression: &TypedExpression<'a>) -> HashSet<Constraint> {
        let mut set = HashSet::new();
        match &expression.1 {
            TypedExpressionKind::Int(ty, _) => {
                set.insert(Constraint::Equal(ty.clone(), Rc::new(Monotype::Int)));
            }
            TypedExpressionKind::Float(ty, _) => {
                set.insert(Constraint::Equal(ty.clone(), Rc::new(Monotype::Float)));
            }
            TypedExpressionKind::Boolean(ty, _) => {
                set.insert(Constraint::Equal(ty.clone(), Rc::new(Monotype::Boolean)));
            }
            TypedExpressionKind::Char(ty, _) => {
                set.insert(Constraint::Equal(ty.clone(), Rc::new(Monotype::Char)));
            }
            TypedExpressionKind::Identifier(_) => {}
            TypedExpressionKind::Unary(TypedUnary {
                operation,
                expression,
                ty,
            }) => {
                let expression_constraints = self.get_constraints(expression);
                match operation {
                    UnaryOperation::Plus | UnaryOperation::Minus => {
                        set.insert(Constraint::Equal(ty.clone(), Rc::new(Monotype::Int)));
                        set.insert(Constraint::Equal(
                            expression.1.get_type(),
                            Rc::new(Monotype::Int),
                        ));
                    }
                    UnaryOperation::Not => {
                        set.insert(Constraint::Equal(ty.clone(), Rc::new(Monotype::Boolean)));
                        set.insert(Constraint::Equal(
                            expression.1.get_type(),
                            Rc::new(Monotype::Boolean),
                        ));
                    }
                }

                set.extend(expression_constraints.into_iter());
            }
            TypedExpressionKind::Binary(TypedBinary {
                operation,
                left,
                right,
                ty,
            }) => {
                let left_constraints = self.get_constraints(left);
                let right_constraints = self.get_constraints(right);
                match operation {
                    BinaryOperation::Plus
                    | BinaryOperation::Minus
                    | BinaryOperation::Multiply
                    | BinaryOperation::Divide => {
                        set.insert(Constraint::Equal(ty.clone(), Rc::new(Monotype::Int)));
                        set.insert(Constraint::Equal(left.1.get_type(), Rc::new(Monotype::Int)));
                        set.insert(Constraint::Equal(
                            right.1.get_type(),
                            Rc::new(Monotype::Int),
                        ));
                    }
                    BinaryOperation::Equals
                    | BinaryOperation::LessThan
                    | BinaryOperation::GreaterThan
                    | BinaryOperation::LessThanEquals
                    | BinaryOperation::GreaterThanEquals => {
                        set.insert(Constraint::Equal(ty.clone(), Rc::new(Monotype::Boolean)));
                    }
                    BinaryOperation::Or | BinaryOperation::And => {
                        set.insert(Constraint::Equal(ty.clone(), Rc::new(Monotype::Boolean)));
                        set.insert(Constraint::Equal(
                            left.1.get_type(),
                            Rc::new(Monotype::Boolean),
                        ));
                        set.insert(Constraint::Equal(
                            right.1.get_type(),
                            Rc::new(Monotype::Boolean),
                        ));
                    }
                }
                set.extend(left_constraints);
                set.extend(right_constraints);
            }
            TypedExpressionKind::If(TypedIf {
                condition,
                then_branch,
                else_branch,
                ty,
            }) => {
                let condition_constraints = self.get_constraints(condition);
                let then_constraints = self.get_constraints(then_branch);
                let else_constraints = else_branch.as_ref().map_or_else(
                    || HashSet::new(),
                    |else_branch| self.get_constraints(else_branch),
                );
                set.insert(Constraint::Equal(
                    condition.1.get_type(),
                    Rc::new(Monotype::Boolean),
                ));
                if let Some(else_branch) = else_branch {
                    set.insert(Constraint::Equal(then_branch.1.get_type(), ty.clone()));
                    set.insert(Constraint::Equal(else_branch.1.get_type(), ty.clone()));
                } else {
                    set.insert(Constraint::Equal(
                        then_branch.1.get_type(),
                        Rc::new(Monotype::Void),
                    ));
                }

                set.extend(condition_constraints);
                set.extend(then_constraints);
                set.extend(else_constraints);
            }
            TypedExpressionKind::Let(TypedLet {
                name,
                given_type: _,
                expression,
                ty: _,
            }) => {
                let expression_constraints = self.get_constraints(expression);
                set.insert(Constraint::Equal(
                    name.1.ty.clone(),
                    expression.1.get_type(),
                ));
                set.extend(expression_constraints);
            }
            TypedExpressionKind::Block(expressions) => {
                for expression in expressions {
                    set.extend(self.get_constraints(expression));
                }
            }
            TypedExpressionKind::Application(TypedApplication {
                function_name,
                parameters,
                ty,
            }) => {
                let parameter_types = parameters
                    .iter()
                    .map(|parameter| parameter.1.get_type())
                    .collect::<Vec<_>>();
                for parameter in parameters {
                    let parameter_constraints = self.get_constraints(parameter);
                    set.extend(parameter_constraints);
                }

                set.insert(Constraint::Equal(
                    function_name.1.ty.clone(),
                    Rc::new(Monotype::Function {
                        parameters: parameter_types,
                        ret: ty.clone(),
                    }),
                ));
            }
            TypedExpressionKind::While(TypedWhile {
                condition,
                expression,
            }) => {
                let condition_constraints = self.get_constraints(condition);
                let expression_constraints = self.get_constraints(expression);
                set.insert(Constraint::Equal(
                    condition.1.get_type(),
                    Rc::new(Monotype::Boolean),
                ));
                set.extend(condition_constraints);
                set.extend(expression_constraints);
            }
            TypedExpressionKind::Return(_) => {}
        }

        set
    }

    pub fn unify(&self, constraints: &HashSet<Constraint>) -> Substitution {
        let mut iter = constraints.iter();
        match iter.next() {
            None => Substitution::new(),
            Some(constraint) => {
                let substitution_head = self.unify_one(constraint);
                let substituted_tail = substitution_head.apply_constraints(iter);
                let substitution_tail = self.unify(&substituted_tail);
                substitution_head.compose(substitution_tail)
            }
        }
    }

    fn unify_one(&self, constraint: &Constraint) -> Substitution {
        match constraint {
            Constraint::Equal(left, right) => match (&**left, &**right) {
                (Monotype::Int, Monotype::Int)
                | (Monotype::Float, Monotype::Float)
                | (Monotype::Boolean, Monotype::Boolean)
                | (Monotype::Char, Monotype::Char)
                | (Monotype::Void, Monotype::Void) => Substitution::new(),
                (
                    Monotype::Function {
                        parameters: first_parameters,
                        ret: first_ret,
                    },
                    Monotype::Function {
                        parameters: second_parameters,
                        ret: second_ret,
                    },
                ) if first_parameters.len() == second_parameters.len() => {
                    let mut set = HashSet::new();
                    first_parameters.iter().zip(second_parameters).for_each(
                        |(first_parameter, second_parameter)| {
                            set.insert(Constraint::Equal(
                                first_parameter.clone(),
                                second_parameter.clone(),
                            ));
                        },
                    );
                    set.insert(Constraint::Equal(first_ret.clone(), second_ret.clone()));
                    self.unify(&set)
                }
                (Monotype::Existential(id), _) => self.unify_var(*id, right.clone()),
                (_, Monotype::Existential(id)) => self.unify_var(*id, left.clone()),
                _ => todo!("Unification error occurred!"),
            },
        }
    }

    fn unify_var(&self, id: usize, ty: MonotypeRef) -> Substitution {
        match &*ty {
            &Monotype::Existential(ty_id) if id == ty_id => Substitution::new(),
            Monotype::Existential(_) => {
                let mut substitution = Substitution::new();
                substitution.insert(id, ty);
                substitution
            }
            _ if self.occurs(id, ty.clone()) => todo!("Occurs check failed!"),
            _ => {
                let mut substitution = Substitution::new();
                substitution.insert(id, ty);
                substitution
            }
        }
    }

    fn occurs(&self, id: usize, ty: MonotypeRef) -> bool {
        match &*ty {
            Monotype::Function { parameters, ret } => {
                parameters
                    .iter()
                    .any(|parameter| self.occurs(id, parameter.clone()))
                    || self.occurs(id, ret.clone())
            }
            &Monotype::Existential(ty_id) => id == ty_id,
            _ => false,
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
