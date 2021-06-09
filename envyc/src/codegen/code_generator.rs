use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicType, BasicTypeEnum},
    values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
    FloatPredicate, IntPredicate,
};

use crate::{
    environment::Environment,
    error::Error,
    interner::Interner,
    parser::{
        expression::{BinaryOperation, UnaryOperation},
        typed_ast::{TypedExternDeclaration, TypedFunction, TypedProgram, TypedPrototype},
        typed_expression::{
            TypedApplication, TypedBinary, TypedExpression, TypedExpressionKind, TypedIdentifier,
            TypedIf, TypedLet, TypedUnary, TypedWhile,
        },
    },
    semantic_analyzer::types::Type,
};

use super::context::FunctionContext;

pub struct CodeGenerator<'a, 'b, 'ctx> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    builder: &'a Builder<'ctx>,
    interner: &'b mut Interner<String>,
    env: &'a mut Environment<PointerValue<'ctx>>,
}

impl<'a, 'b, 'c, 'ctx> CodeGenerator<'a, 'b, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        builder: &'a Builder<'ctx>,
        interner: &'b mut Interner<String>,
        env: &'a mut Environment<PointerValue<'ctx>>,
    ) -> Self {
        Self {
            context,
            module,
            builder,
            interner,
            env,
        }
    }

    pub fn generate_program(&mut self, program: &TypedProgram<'c>) -> Result<(), Vec<Error<'c>>> {
        let mut errors = vec![];
        for extern_declaration in &program.extern_declarations {
            if let Err(error) = self.generate_extern(extern_declaration) {
                errors.push(error);
            }
        }

        for function in &program.functions {
            if let Err(error) = self.generate_prototype(&function.prototype) {
                errors.push(error);
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            for function in &program.functions {
                if let Err(error) = self.generate_function(function) {
                    errors.push(error);
                }
            }

            if !errors.is_empty() {
                Err(errors)
            } else {
                Ok(())
            }
        }
    }

    fn generate_extern(
        &mut self,
        extern_declaration: &TypedExternDeclaration<'c>,
    ) -> Result<(), Error<'c>> {
        let parameter_types = extern_declaration
            .parameters
            .iter()
            .map(|parameter| parameter.0)
            .map(|ty| convert_basic_type(ty, self.context))
            .collect::<Vec<_>>();

        let function_type = if let Type::Void = extern_declaration.return_type.0 {
            self.context.void_type().fn_type(&parameter_types, false)
        } else {
            convert_type(extern_declaration.return_type.0, self.context)
                .fn_type(&parameter_types, false)
        };

        self.module.add_function(
            &self.interner.get(extern_declaration.name),
            function_type,
            None,
        );
        Ok(())
    }

    fn generate_prototype(&mut self, prototype: &TypedPrototype<'c>) -> Result<(), Error<'c>> {
        let parameter_types = prototype
            .parameters
            .iter()
            .map(|parameter| parameter.ty)
            .map(|ty| convert_basic_type(ty, self.context))
            .collect::<Vec<_>>();

        let function_type = if let Type::Void = prototype.return_type {
            self.context.void_type().fn_type(&parameter_types, false)
        } else {
            convert_type(prototype.return_type, self.context).fn_type(&parameter_types, false)
        };

        self.module
            .add_function(&self.interner.get(prototype.name), function_type, None);
        Ok(())
    }

    fn generate_function(&mut self, defined_function: &TypedFunction<'c>) -> Result<(), Error<'c>> {
        let function = self
            .module
            .get_function(self.interner.get(defined_function.prototype.name))
            .ok_or(Error::UnknownFunction(defined_function.prototype.span))?;
        let entry_block = self.context.append_basic_block(function, "entry");
        let return_block = self.context.append_basic_block(function, "return");
        self.builder.position_at_end(entry_block);

        self.env.new_scope();
        function
            .get_param_iter()
            .zip(defined_function.prototype.parameters.iter())
            .map(|(llvm_param, param)| (llvm_param, param.name))
            .for_each(|(llvm_param, param_name)| {
                let name = self.interner.get(param_name);
                llvm_param.set_name(name);
                let pointer = self.builder.build_alloca(llvm_param.get_type(), name);
                self.builder.build_store(pointer, llvm_param);
                self.env.define(param_name, pointer);
            });

        let mut function_context =
            FunctionContext::new(defined_function.prototype.name, return_block);
        let expression =
            self.compile_expression(&defined_function.body, function, &mut function_context)?;

        if defined_function.body.1.get_runtime_type() != Type::Never {
            if defined_function.body.1.get_runtime_type() != Type::Void {
                function_context
                    .add_return_block(self.builder.get_insert_block().unwrap(), Some(expression));
            } else {
                function_context.add_return_block(self.builder.get_insert_block().unwrap(), None);
            }

            self.builder.build_unconditional_branch(return_block);
        }

        self.builder.position_at_end(return_block);
        if defined_function.prototype.return_type != Type::Void {
            let return_value = self.builder.build_phi(
                convert_basic_type(defined_function.prototype.return_type, self.context),
                "return_value",
            );

            let phi_nodes = function_context
                .return_blocks
                .iter()
                .map(|(block, value)| (value.as_ref().unwrap() as &dyn BasicValue<'ctx>, *block))
                .collect::<Vec<_>>();
            return_value.add_incoming(phi_nodes.as_slice());
            self.builder
                .build_return(Some(&return_value.as_basic_value()));
        } else {
            self.builder.build_return(None);
        }

        self.env.remove_top_scope();

        if function.verify(true) {
            Ok(())
        } else {
            unsafe {
                function.delete();
            }

            Err(Error::LLVMFunctionFailure)
        }
    }

    fn compile_expression(
        &mut self,
        expression: &TypedExpression<'c>,
        current_function: FunctionValue<'ctx>,
        function_context: &mut FunctionContext<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error<'c>> {
        match expression.1 {
            TypedExpressionKind::Int(value) => {
                let int = self.context.i64_type().const_int(value.abs() as u64, false);
                if value < 0 {
                    Ok(BasicValueEnum::IntValue(int.const_neg()))
                } else {
                    Ok(BasicValueEnum::IntValue(int))
                }
            }
            TypedExpressionKind::Float(value) => Ok(BasicValueEnum::FloatValue(
                self.context.f64_type().const_float(value),
            )),
            TypedExpressionKind::Boolean(value) => Ok(BasicValueEnum::IntValue(
                self.context.bool_type().const_int(value as u64, false),
            )),
            TypedExpressionKind::Char(value) => Ok(BasicValueEnum::IntValue(
                self.context.i8_type().const_int(value as u64, false),
            )),
            TypedExpressionKind::Identifier(ref inner) => self.compile_identifier(inner),
            TypedExpressionKind::Unary(ref inner) => {
                self.compile_unary(inner, current_function, function_context)
            }
            TypedExpressionKind::Binary(ref inner) => {
                self.compile_binary(inner, current_function, function_context)
            }
            TypedExpressionKind::If(ref inner) => {
                self.compile_if(inner, current_function, function_context)
            }
            TypedExpressionKind::Let(ref inner) => {
                self.compile_let(inner, current_function, function_context)?;
                Ok(BasicValueEnum::IntValue(
                    self.context.i64_type().const_zero(),
                ))
            }
            TypedExpressionKind::Block(ref expressions) => expressions.iter().fold(
                Ok(BasicValueEnum::IntValue(
                    self.context.i64_type().const_zero(),
                )),
                |_, expression| {
                    self.compile_expression(expression, current_function, function_context)
                },
            ),
            TypedExpressionKind::Application(ref inner) => {
                self.compile_application(inner, current_function, function_context)
            }
            TypedExpressionKind::While(ref inner) => {
                self.compile_while(inner, current_function, function_context)?;
                Ok(BasicValueEnum::IntValue(
                    self.context.i64_type().const_zero(),
                ))
            }
            TypedExpressionKind::Return(ref value) => {
                let return_value = value
                    .as_ref()
                    .map(|expression| {
                        self.compile_expression(expression, current_function, function_context)
                            .ok()
                    })
                    .flatten();
                function_context
                    .add_return_block(self.builder.get_insert_block().unwrap(), return_value);
                self.builder
                    .build_unconditional_branch(function_context.return_block);
                if let Some(value) = return_value {
                    Ok(value)
                } else {
                    Ok(BasicValueEnum::IntValue(
                        self.context.i64_type().const_zero(),
                    ))
                }
            }
        }
    }

    fn compile_identifier(
        &mut self,
        identifier: &TypedIdentifier,
    ) -> Result<BasicValueEnum<'ctx>, Error<'c>> {
        let value = self.env.get(identifier.id).unwrap();
        Ok(self
            .builder
            .build_load(value, self.interner.get(identifier.id)))
    }

    fn compile_unary(
        &mut self,
        unary: &TypedUnary<'c>,
        current_function: FunctionValue<'ctx>,
        function_context: &mut FunctionContext<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error<'c>> {
        let expression =
            self.compile_expression(&unary.expression, current_function, function_context)?;
        let value = match (unary.operation, expression) {
            (UnaryOperation::Plus, value) => value,
            (UnaryOperation::Minus, BasicValueEnum::IntValue(value)) => {
                BasicValueEnum::IntValue(self.builder.build_int_neg(value, "intneg"))
            }
            (UnaryOperation::Minus, BasicValueEnum::FloatValue(value)) => {
                BasicValueEnum::FloatValue(self.builder.build_float_neg(value, "floatneg"))
            }
            (UnaryOperation::Not, BasicValueEnum::IntValue(value)) => {
                BasicValueEnum::IntValue(self.builder.build_not(value, "boolnot"))
            }
            _ => unreachable!(),
        };

        Ok(value)
    }

    fn compile_binary(
        &mut self,
        binary: &TypedBinary<'c>,
        current_function: FunctionValue<'ctx>,
        function_context: &mut FunctionContext<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error<'c>> {
        let left = self.compile_expression(&binary.left, current_function, function_context)?;
        let right = self.compile_expression(&binary.right, current_function, function_context)?;
        let value = match (binary.operation, left, right) {
            (
                BinaryOperation::Plus,
                BasicValueEnum::IntValue(left),
                BasicValueEnum::IntValue(right),
            ) => BasicValueEnum::IntValue(self.builder.build_int_add(left, right, "intadd")),
            (
                BinaryOperation::Plus,
                BasicValueEnum::FloatValue(left),
                BasicValueEnum::FloatValue(right),
            ) => BasicValueEnum::FloatValue(self.builder.build_float_add(left, right, "floatadd")),
            (
                BinaryOperation::Minus,
                BasicValueEnum::IntValue(left),
                BasicValueEnum::IntValue(right),
            ) => BasicValueEnum::IntValue(self.builder.build_int_sub(left, right, "intsub")),
            (
                BinaryOperation::Minus,
                BasicValueEnum::FloatValue(left),
                BasicValueEnum::FloatValue(right),
            ) => BasicValueEnum::FloatValue(self.builder.build_float_sub(left, right, "floatsub")),
            (
                BinaryOperation::Multiply,
                BasicValueEnum::IntValue(left),
                BasicValueEnum::IntValue(right),
            ) => BasicValueEnum::IntValue(self.builder.build_int_mul(left, right, "intmul")),
            (
                BinaryOperation::Multiply,
                BasicValueEnum::FloatValue(left),
                BasicValueEnum::FloatValue(right),
            ) => BasicValueEnum::FloatValue(self.builder.build_float_mul(left, right, "floatmul")),
            (
                BinaryOperation::Divide,
                BasicValueEnum::IntValue(left),
                BasicValueEnum::IntValue(right),
            ) => BasicValueEnum::IntValue(self.builder.build_int_signed_div(left, right, "intdiv")),
            (
                BinaryOperation::Divide,
                BasicValueEnum::FloatValue(left),
                BasicValueEnum::FloatValue(right),
            ) => BasicValueEnum::FloatValue(self.builder.build_float_div(left, right, "floatdiv")),
            (operation, BasicValueEnum::IntValue(left), BasicValueEnum::IntValue(right)) => {
                let op = match operation {
                    BinaryOperation::Equals => IntPredicate::EQ,
                    BinaryOperation::LessThan => IntPredicate::SLT,
                    BinaryOperation::GreaterThan => IntPredicate::SGT,
                    BinaryOperation::LessThanEquals => IntPredicate::SLE,
                    BinaryOperation::GreaterThanEquals => IntPredicate::SGE,
                    BinaryOperation::Or => {
                        return Ok(BasicValueEnum::IntValue(
                            self.builder.build_or(left, right, "boolor"),
                        ));
                    }
                    BinaryOperation::And => {
                        return Ok(BasicValueEnum::IntValue(
                            self.builder.build_and(left, right, "booland"),
                        ));
                    }
                    _ => unreachable!(),
                };

                BasicValueEnum::IntValue(self.builder.build_int_compare(op, left, right, "intcmp"))
            }
            (operation, BasicValueEnum::FloatValue(left), BasicValueEnum::FloatValue(right)) => {
                let op = match operation {
                    BinaryOperation::Equals => FloatPredicate::OEQ,
                    BinaryOperation::LessThan => FloatPredicate::OLT,
                    BinaryOperation::GreaterThan => FloatPredicate::OGT,
                    BinaryOperation::LessThanEquals => FloatPredicate::OLE,
                    BinaryOperation::GreaterThanEquals => FloatPredicate::OGE,
                    _ => unreachable!(),
                };

                BasicValueEnum::IntValue(
                    self.builder
                        .build_float_compare(op, left, right, "floatcmp"),
                )
            }
            _ => unreachable!(),
        };

        Ok(value)
    }

    fn compile_if(
        &mut self,
        typed_if: &TypedIf<'c>,
        current_function: FunctionValue<'ctx>,
        function_context: &mut FunctionContext<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error<'c>> {
        let then_block = self.context.append_basic_block(current_function, "ifthen");
        let else_block = self.context.append_basic_block(current_function, "ifelse");
        let end_block = self.context.append_basic_block(current_function, "ifend");

        let condition =
            self.compile_expression(&typed_if.condition, current_function, function_context)?;
        self.builder
            .build_conditional_branch(condition.into_int_value(), then_block, else_block);

        self.builder.position_at_end(then_block);
        let then_branch =
            self.compile_expression(&typed_if.then_branch, current_function, function_context)?;

        if typed_if.then_branch.1.get_runtime_type() != Type::Never {
            self.builder.build_unconditional_branch(end_block);
        }

        if let Some(ref else_branch) = typed_if.else_branch {
            self.builder.position_at_end(else_block);
            let else_branch_gen =
                self.compile_expression(else_branch, current_function, function_context)?;

            if else_branch.1.get_runtime_type() != Type::Never {
                self.builder.build_unconditional_branch(end_block);
            }

            self.builder.position_at_end(end_block);
            if typed_if.then_branch.1.get_runtime_type() == Type::Never {
                Ok(else_branch_gen)
            } else if else_branch.1.get_runtime_type() == Type::Never {
                Ok(then_branch)
            } else {
                let phi = self.builder.build_phi(then_branch.get_type(), "ifphi");
                phi.add_incoming(&[(&then_branch, then_block), (&else_branch_gen, else_block)]);
                Ok(phi.as_basic_value())
            }
        } else {
            self.builder.position_at_end(else_block);
            self.builder.build_unconditional_branch(end_block);
            self.builder.position_at_end(end_block);
            Ok(BasicValueEnum::IntValue(
                self.context.i64_type().const_zero(),
            ))
        }
    }

    fn compile_let(
        &mut self,
        typed_let: &TypedLet<'c>,
        current_function: FunctionValue<'ctx>,
        function_context: &mut FunctionContext<'ctx>,
    ) -> Result<(), Error<'c>> {
        let value =
            self.compile_expression(&typed_let.expression, current_function, function_context)?;
        let id = typed_let.name.1.id;
        if self.env.get(id).is_none() {
            let pointer = self
                .builder
                .build_alloca(value.get_type(), self.interner.get(id));
            self.env.define(id, pointer);
        }

        let pointer = self.env.get(id).unwrap();
        self.builder.build_store(pointer, value);

        Ok(())
    }

    fn compile_application(
        &mut self,
        application: &TypedApplication<'c>,
        current_function: FunctionValue<'ctx>,
        function_context: &mut FunctionContext<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error<'c>> {
        let function_name = self.interner.get(application.function_name.1);
        let function_call = format!("call_{}", function_name);
        let function = self.module.get_function(&function_name).unwrap();
        let mut arguments = Vec::new();
        for parameter in &application.parameters {
            arguments.push(self.compile_expression(
                parameter,
                current_function,
                function_context,
            )?);
        }

        Ok(self
            .builder
            .build_call(function, &arguments, &function_call)
            .try_as_basic_value()
            .left()
            .unwrap_or_else(|| BasicValueEnum::IntValue(self.context.i64_type().const_zero())))
    }

    fn compile_while(
        &mut self,
        typed_while: &TypedWhile<'c>,
        current_function: FunctionValue<'ctx>,
        function_context: &mut FunctionContext<'ctx>,
    ) -> Result<(), Error<'c>> {
        let condition_check_block = self
            .context
            .append_basic_block(current_function, "condition_check");
        let loop_block = self.context.append_basic_block(current_function, "loop");
        let after_loop_block = self
            .context
            .append_basic_block(current_function, "after_loop");
        self.builder
            .build_unconditional_branch(condition_check_block);

        self.builder.position_at_end(condition_check_block);
        let condition =
            self.compile_expression(&typed_while.condition, current_function, function_context)?;

        self.builder.build_conditional_branch(
            condition.into_int_value(),
            loop_block,
            after_loop_block,
        );
        self.builder.position_at_end(loop_block);
        self.compile_expression(&typed_while.expression, current_function, function_context)?;

        if typed_while.expression.1.get_runtime_type() != Type::Never {
            self.builder
                .build_unconditional_branch(condition_check_block);
        }

        self.builder.position_at_end(after_loop_block);
        Ok(())
    }
}

fn convert_type(ty: Type, context: &Context) -> Box<dyn BasicType + '_> {
    match ty {
        Type::Int => Box::new(context.i64_type()),
        Type::Float => Box::new(context.f64_type()),
        Type::Boolean => Box::new(context.bool_type()),
        Type::Char => Box::new(context.i8_type()),
        _ => unreachable!(),
    }
}

fn convert_basic_type(ty: Type, context: &Context) -> BasicTypeEnum {
    match ty {
        Type::Int => BasicTypeEnum::IntType(context.i64_type()),
        Type::Float => BasicTypeEnum::FloatType(context.f64_type()),
        Type::Boolean => BasicTypeEnum::IntType(context.bool_type()),
        Type::Char => BasicTypeEnum::IntType(context.i8_type()),
        _ => unreachable!(),
    }
}
