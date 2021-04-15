use crate::{
    environment::Environment,
    error::{Error, Span},
    function_table::FunctionTable,
    parser::{
        ast::{Function, Parameter, Program},
        expression::{
            Application, Binary, BinaryOperation, Expression, ExpressionKind, Identifier, If, Let,
            Unary, UnaryOperation, While,
        },
        typed_ast::{TypedFunction, TypedParameter, TypedProgram, TypedPrototype},
        typed_expression::{
            TypedApplication, TypedBinary, TypedExpression, TypedExpressionKind, TypedIdentifier,
            TypedIf, TypedLet, TypedUnary, TypedWhile,
        },
    },
};

use super::types::Type;

pub trait TypeCheck<'a> {
    type Output;
    type Error;

    fn check(
        self,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait TypeCheckSpan<'a> {
    type Output;
    type Error;

    fn check_span(
        self,
        span: Span<'a>,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error>;
}

impl<'a, T: TypeCheck<'a>> TypeCheck<'a> for Vec<T> {
    type Output = Vec<T::Output>;
    type Error = Vec<T::Error>;

    fn check(
        self,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        let mut results = vec![];
        let mut errors = vec![];
        for value in self {
            match value.check(env, function_table) {
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

    fn check(
        self,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        for function in &self.functions {
            let function_name = function.prototype.name;
            let function_return_type = function.prototype.return_type.0;
            let parameter_types = function
                .prototype
                .parameters
                .iter()
                .map(|parameter| parameter.ty)
                .collect::<Vec<_>>();
            env.define(function_name, function_return_type);
            function_table.add_function_definition(function_name, parameter_types);
        }

        Ok(TypedProgram {
            functions: self.functions.check(env, function_table)?,
        })
    }
}

impl<'a> TypeCheck<'a> for Function<'a> {
    type Output = TypedFunction<'a>;
    type Error = Error<'a>;

    fn check(
        self,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        env.new_scope();
        let mut typed_params = vec![];
        for parameter in self.prototype.parameters {
            if parameter.ty == Type::Void {
                return Err(Error::IllegalType(parameter.span));
            } else {
                env.define(parameter.name, parameter.ty);
                typed_params.push(TypedParameter::new(
                    parameter.span,
                    parameter.ty,
                    parameter.name,
                ));
            }
        }

        let typed_body = self.body.check(env, function_table)?;
        let return_type = get_type(&typed_body.1);
        if self.prototype.return_type.0 != return_type {
            return Err(Error::TypeMismatch {
                span: typed_body.0,
                expected_type: self.prototype.return_type.0,
                actual_type: return_type,
            });
        }

        let typed_function = TypedFunction::new(
            TypedPrototype::new(
                self.prototype.span,
                self.prototype.name,
                typed_params,
                return_type,
            ),
            typed_body,
        );
        env.remove_top_scope();
        Ok(typed_function)
    }
}

impl<'a> TypeCheck<'a> for Parameter<'a> {
    type Output = TypedParameter<'a>;
    type Error = Error<'a>;

    fn check(
        self,
        _: &mut Environment<Type>,
        _: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        Ok(TypedParameter::new(self.span, self.ty, self.name))
    }
}

impl<'a> TypeCheck<'a> for Expression<'a> {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check(
        self,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        match self.1 {
            ExpressionKind::Int(value) => Ok((self.0, TypedExpressionKind::Int(value))),
            ExpressionKind::Float(value) => Ok((self.0, TypedExpressionKind::Float(value))),
            ExpressionKind::Boolean(value) => Ok((self.0, TypedExpressionKind::Boolean(value))),
            ExpressionKind::Char(value) => Ok((self.0, TypedExpressionKind::Char(value))),
            ExpressionKind::Identifier(inner) => inner.check_span(self.0, env, function_table),
            ExpressionKind::Unary(inner) => inner.check_span(self.0, env, function_table),
            ExpressionKind::Binary(inner) => inner.check_span(self.0, env, function_table),
            ExpressionKind::If(inner) => inner.check_span(self.0, env, function_table),
            ExpressionKind::Let(inner) => inner.check_span(self.0, env, function_table),
            ExpressionKind::Block(expressions) => {
                env.new_scope();
                match expressions.check(env, function_table) {
                    Ok(typed_expressions) => {
                        env.remove_top_scope();
                        Ok((self.0, TypedExpressionKind::Block(typed_expressions)))
                    }
                    Err(errors) => Err(errors.into_iter().next().unwrap()),
                }
            }
            ExpressionKind::Application(inner) => inner.check_span(self.0, env, function_table),
            ExpressionKind::While(inner) => inner.check_span(self.0, env, function_table),
        }
    }
}

impl<'a> TypeCheckSpan<'a> for Identifier {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check_span(
        self,
        span: Span<'a>,
        env: &mut Environment<Type>,
        _: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        match env.get(self.0) {
            Some(ty) => Ok((
                span,
                TypedExpressionKind::Identifier(TypedIdentifier { id: self.0, ty }),
            )),
            None => Err(Error::UndefinedVariable(span)),
        }
    }
}

impl<'a> TypeCheckSpan<'a> for Unary<'a> {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check_span(
        self,
        span: Span<'a>,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        let typed_expression = self.expression.check(env, function_table)?;
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
                }),
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

    fn check_span(
        self,
        span: Span<'a>,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        let typed_left = self.left.check(env, function_table)?;
        let typed_right = self.right.check(env, function_table)?;
        let left_type = get_type(&typed_left.1);
        let right_type = get_type(&typed_right.1);
        let result_type = match (self.operation, left_type, right_type) {
            (BinaryOperation::Plus, Type::Int, Type::Int) => Some(Type::Int),
            (BinaryOperation::Plus, Type::Float, Type::Float) => Some(Type::Float),
            (BinaryOperation::Plus, Type::Char, Type::Char) => Some(Type::Char),

            (BinaryOperation::Minus, Type::Int, Type::Int) => Some(Type::Int),
            (BinaryOperation::Minus, Type::Float, Type::Float) => Some(Type::Float),

            (BinaryOperation::Multiply, Type::Int, Type::Int) => Some(Type::Int),
            (BinaryOperation::Multiply, Type::Float, Type::Float) => Some(Type::Float),

            (BinaryOperation::Divide, Type::Int, Type::Int) => Some(Type::Int),
            (BinaryOperation::Divide, Type::Float, Type::Float) => Some(Type::Float),

            (BinaryOperation::Equals, Type::Int, Type::Int)
            | (BinaryOperation::Equals, Type::Float, Type::Float)
            | (BinaryOperation::Equals, Type::Char, Type::Char)
            | (BinaryOperation::Equals, Type::Boolean, Type::Boolean)
            | (BinaryOperation::LessThan, Type::Int, Type::Int)
            | (BinaryOperation::LessThan, Type::Float, Type::Float)
            | (BinaryOperation::LessThan, Type::Char, Type::Char)
            | (BinaryOperation::GreaterThan, Type::Int, Type::Int)
            | (BinaryOperation::GreaterThan, Type::Float, Type::Float)
            | (BinaryOperation::LessThanEquals, Type::Int, Type::Int)
            | (BinaryOperation::LessThanEquals, Type::Float, Type::Float)
            | (BinaryOperation::GreaterThanEquals, Type::Int, Type::Int)
            | (BinaryOperation::GreaterThanEquals, Type::Float, Type::Float)
            | (BinaryOperation::Or, Type::Boolean, Type::Boolean)
            | (BinaryOperation::And, Type::Boolean, Type::Boolean) => Some(Type::Boolean),
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
                }),
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

    fn check_span(
        self,
        span: Span<'a>,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        let typed_condition = self.condition.check(env, function_table)?;
        let condition_type = get_type(&typed_condition.1);
        if condition_type != Type::Boolean {
            return Err(Error::TypeMismatch {
                span: typed_condition.0,
                expected_type: Type::Boolean,
                actual_type: condition_type,
            });
        }

        let typed_then = self.then_branch.check(env, function_table)?;
        let then_type = get_type(&typed_then.1);
        if let Some(else_branch) = self.else_branch {
            let typed_else = else_branch.check(env, function_table)?;
            let else_type = get_type(&typed_else.1);

            if then_type == else_type {
                Ok((
                    span,
                    TypedExpressionKind::If(TypedIf {
                        condition: Box::new(typed_condition),
                        then_branch: Box::new(typed_then),
                        else_branch: Some(Box::new(typed_else)),
                        ty: then_type,
                    }),
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
                }),
            ))
        }
    }
}

impl<'a> TypeCheckSpan<'a> for Let<'a> {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check_span(
        self,
        span: Span<'a>,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        let typed_expression = self.expression.check(env, function_table)?;
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

        let (identifier_span, Identifier(identifier_id)) = self.name;
        let typed_name = (
            identifier_span,
            TypedIdentifier {
                id: identifier_id,
                ty: expression_type,
            },
        );

        env.define(identifier_id, expression_type);
        Ok((
            span,
            TypedExpressionKind::Let(TypedLet {
                name: typed_name,
                given_type: self.given_type,
                expression: Box::new(typed_expression),
                ty: expression_type,
            }),
        ))
    }
}

impl<'a> TypeCheckSpan<'a> for Application<'a> {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check_span(
        self,
        span: Span<'a>,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        let mut parameters = Vec::new();
        for parameter in self.parameters {
            let typed_value = parameter.check(env, function_table)?;
            parameters.push(typed_value);
        }

        let (function_span, Identifier(function_name)) = self.function_name;
        let defined_types = function_table.get_function_definition(function_name, function_span)?;
        if parameters.len() != defined_types.len() {
            return Err(Error::ParameterMismatch {
                span,
                expected_parameter_count: defined_types.len(),
                actual_parameter_count: parameters.len(),
            });
        }

        for (&defined_parameter_type, actual_parameter) in defined_types.iter().zip(&parameters) {
            let actual_parameter_type = get_type(&actual_parameter.1);
            if defined_parameter_type != actual_parameter_type {
                return Err(Error::TypeMismatch {
                    span: actual_parameter.0,
                    expected_type: defined_parameter_type,
                    actual_type: actual_parameter_type,
                });
            }
        }

        let return_type = env.get(function_name).unwrap();
        Ok((
            span,
            TypedExpressionKind::Application(TypedApplication {
                function_name: (function_span, function_name),
                parameters,
                ty: return_type,
            }),
        ))
    }
}

impl<'a> TypeCheckSpan<'a> for While<'a> {
    type Output = TypedExpression<'a>;
    type Error = Error<'a>;

    fn check_span(
        self,
        span: Span<'a>,
        env: &mut Environment<Type>,
        function_table: &mut FunctionTable,
    ) -> Result<Self::Output, Self::Error> {
        let typed_condition = self.condition.check(env, function_table)?;
        let condition_type = get_type(&typed_condition.1);
        if condition_type != Type::Boolean {
            return Err(Error::TypeMismatch {
                span: typed_condition.0,
                expected_type: Type::Boolean,
                actual_type: condition_type,
            });
        }

        let typed_expression = self.expression.check(env, function_table)?;
        Ok((
            span,
            TypedExpressionKind::While(TypedWhile {
                condition: Box::new(typed_condition),
                expression: Box::new(typed_expression),
            }),
        ))
    }
}

fn get_type(typed_expression_kind: &TypedExpressionKind) -> Type {
    match typed_expression_kind {
        TypedExpressionKind::Int(_) => Type::Int,
        TypedExpressionKind::Float(_) => Type::Float,
        TypedExpressionKind::Boolean(_) => Type::Boolean,
        TypedExpressionKind::Char(_) => Type::Char,
        TypedExpressionKind::Identifier(ref inner) => inner.ty,
        TypedExpressionKind::Unary(ref inner) => inner.ty,
        TypedExpressionKind::Binary(ref inner) => inner.ty,
        TypedExpressionKind::If(ref inner) => inner.ty,
        TypedExpressionKind::Let(ref inner) => inner.ty,
        TypedExpressionKind::Block(ref expressions) => expressions
            .iter()
            .last()
            .map(|expression| get_type(&expression.1))
            .unwrap_or(Type::Void),
        TypedExpressionKind::Application(ref inner) => inner.ty,
        TypedExpressionKind::While(_) => Type::Void,
    }
}
