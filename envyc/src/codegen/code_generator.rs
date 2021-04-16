use std::u64;

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

pub trait CodeGenerator<'a, 'ctx> {
    type Output;
    type Error;

    fn code_gen(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait CodeGeneratorFunction<'a, 'ctx> {
    type Output;
    type Error;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: FunctionValue<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error>;
}

impl<'a, 'ctx> CodeGenerator<'a, 'ctx> for TypedProgram<'a> {
    type Output = ();
    type Error = Vec<Error<'a>>;

    fn code_gen(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let mut errors = vec![];
        for extern_declaration in &self.extern_declarations {
            if let Err(error) = extern_declaration.code_gen(context, module, builder, interner, env)
            {
                errors.push(error);
            }
        }

        for function in &self.functions {
            if let Err(error) = function
                .prototype
                .code_gen(context, module, builder, interner, env)
            {
                errors.push(error);
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            for function in &self.functions {
                if let Err(error) = function.code_gen(context, module, builder, interner, env) {
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
}

impl<'a, 'ctx> CodeGenerator<'a, 'ctx> for TypedPrototype<'a> {
    type Output = ();
    type Error = Error<'a>;

    fn code_gen(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        _: &Builder<'ctx>,
        interner: &mut Interner<String>,
        _: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let parameter_types = self
            .parameters
            .iter()
            .map(|parameter| parameter.ty)
            .map(|ty| convert_basic_type(ty, context))
            .collect::<Vec<_>>();

        let function_type = if let Type::Void = self.return_type {
            context.void_type().fn_type(&parameter_types, false)
        } else {
            convert_type(self.return_type, context).fn_type(&parameter_types, false)
        };

        module.add_function(&interner.get(self.name), function_type, None);
        Ok(())
    }
}

impl<'a, 'ctx> CodeGenerator<'a, 'ctx> for TypedExternDeclaration<'a> {
    type Output = ();
    type Error = Error<'a>;

    fn code_gen(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        _: &Builder<'ctx>,
        interner: &mut Interner<String>,
        _: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let parameter_types = self
            .parameters
            .iter()
            .map(|parameter| parameter.0)
            .map(|ty| convert_basic_type(ty, context))
            .collect::<Vec<_>>();

        let function_type = if let Type::Void = self.return_type.0 {
            context.void_type().fn_type(&parameter_types, false)
        } else {
            convert_type(self.return_type.0, context).fn_type(&parameter_types, false)
        };

        module.add_function(&interner.get(self.name), function_type, None);
        Ok(())
    }
}

impl<'a, 'ctx> CodeGenerator<'a, 'ctx> for TypedFunction<'a> {
    type Output = ();
    type Error = Error<'a>;

    fn code_gen(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let function = module
            .get_function(interner.get(self.prototype.name))
            .ok_or(Error::UnknownFunction(self.prototype.span))?;
        let entry = context.append_basic_block(function, "entry");
        builder.position_at_end(entry);

        env.new_scope();
        function
            .get_param_iter()
            .zip(self.prototype.parameters.iter())
            .map(|(llvm_param, param)| (llvm_param, param.name))
            .for_each(|(llvm_param, param_name)| {
                let name = interner.get(param_name);
                llvm_param.set_name(name);
                let pointer = builder.build_alloca(llvm_param.get_type(), name);
                builder.build_store(pointer, llvm_param);
                env.define(param_name, pointer);
            });

        let expression = self
            .body
            .code_gen_function(context, module, builder, function, interner, env)?;
        if self.prototype.return_type == Type::Void {
            builder.build_return(None);
        } else {
            builder.build_return(Some(&expression));
        }

        env.remove_top_scope();
        if function.verify(true) {
            Ok(())
        } else {
            unsafe {
                function.delete();
            }

            Err(Error::LLVMFunctionFailure)
        }
    }
}

impl<'a, 'ctx> CodeGeneratorFunction<'a, 'ctx> for TypedExpression<'a> {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error<'a>;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: FunctionValue<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        match self.1 {
            TypedExpressionKind::Int(value) => {
                let int = context.i64_type().const_int(value.abs() as u64, false);
                if value < 0 {
                    Ok(BasicValueEnum::IntValue(int.const_neg()))
                } else {
                    Ok(BasicValueEnum::IntValue(int))
                }
            }
            TypedExpressionKind::Float(value) => Ok(BasicValueEnum::FloatValue(
                context.f64_type().const_float(value),
            )),
            TypedExpressionKind::Boolean(value) => Ok(BasicValueEnum::IntValue(
                context.bool_type().const_int(value as u64, false),
            )),
            TypedExpressionKind::Char(value) => Ok(BasicValueEnum::IntValue(
                context.i8_type().const_int(value as u64, false),
            )),
            TypedExpressionKind::Identifier(ref inner) => {
                inner.code_gen_function(context, module, builder, current_function, interner, env)
            }
            TypedExpressionKind::Unary(ref inner) => {
                inner.code_gen_function(context, module, builder, current_function, interner, env)
            }
            TypedExpressionKind::Binary(ref inner) => {
                inner.code_gen_function(context, module, builder, current_function, interner, env)
            }
            TypedExpressionKind::If(ref inner) => {
                inner.code_gen_function(context, module, builder, current_function, interner, env)
            }
            TypedExpressionKind::Let(ref inner) => {
                inner.code_gen_function(
                    context,
                    module,
                    builder,
                    current_function,
                    interner,
                    env,
                )?;
                Ok(BasicValueEnum::IntValue(context.i64_type().const_zero()))
            }
            TypedExpressionKind::Block(ref expressions) => expressions.iter().fold(
                Ok(BasicValueEnum::IntValue(context.i64_type().const_zero())),
                |_, expression| {
                    expression.code_gen_function(
                        context,
                        module,
                        builder,
                        current_function,
                        interner,
                        env,
                    )
                },
            ),
            TypedExpressionKind::Application(ref inner) => {
                inner.code_gen_function(context, module, builder, current_function, interner, env)
            }
            TypedExpressionKind::While(ref inner) => {
                inner.code_gen_function(
                    context,
                    module,
                    builder,
                    current_function,
                    interner,
                    env,
                )?;
                Ok(BasicValueEnum::IntValue(context.i64_type().const_zero()))
            }
        }
    }
}

impl<'a, 'ctx> CodeGeneratorFunction<'a, 'ctx> for TypedIdentifier {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error<'a>;

    fn code_gen_function(
        &self,
        _: &'ctx Context,
        _: &Module<'ctx>,
        builder: &Builder<'ctx>,
        _: FunctionValue<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let value = env.get(self.id).unwrap();
        Ok(builder.build_load(value, interner.get(self.id)))
    }
}

impl<'a, 'ctx> CodeGeneratorFunction<'a, 'ctx> for TypedUnary<'a> {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error<'a>;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: FunctionValue<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let expression = self.expression.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
            env,
        )?;
        let value = match (self.operation, expression) {
            (UnaryOperation::Plus, value) => value,
            (UnaryOperation::Minus, BasicValueEnum::IntValue(value)) => {
                BasicValueEnum::IntValue(builder.build_int_neg(value, "intneg"))
            }
            (UnaryOperation::Minus, BasicValueEnum::FloatValue(value)) => {
                BasicValueEnum::FloatValue(builder.build_float_neg(value, "floatneg"))
            }
            (UnaryOperation::Not, BasicValueEnum::IntValue(value)) => {
                BasicValueEnum::IntValue(builder.build_not(value, "boolnot"))
            }
            _ => unreachable!(),
        };

        Ok(value)
    }
}

impl<'a, 'ctx> CodeGeneratorFunction<'a, 'ctx> for TypedBinary<'a> {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error<'a>;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: FunctionValue<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let left = self.left.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
            env,
        )?;
        let right = self.right.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
            env,
        )?;
        let value = match (self.operation, left, right) {
            (
                BinaryOperation::Plus,
                BasicValueEnum::IntValue(left),
                BasicValueEnum::IntValue(right),
            ) => BasicValueEnum::IntValue(builder.build_int_add(left, right, "intadd")),
            (
                BinaryOperation::Plus,
                BasicValueEnum::FloatValue(left),
                BasicValueEnum::FloatValue(right),
            ) => BasicValueEnum::FloatValue(builder.build_float_add(left, right, "floatadd")),
            (
                BinaryOperation::Minus,
                BasicValueEnum::IntValue(left),
                BasicValueEnum::IntValue(right),
            ) => BasicValueEnum::IntValue(builder.build_int_sub(left, right, "intsub")),
            (
                BinaryOperation::Minus,
                BasicValueEnum::FloatValue(left),
                BasicValueEnum::FloatValue(right),
            ) => BasicValueEnum::FloatValue(builder.build_float_sub(left, right, "floatsub")),
            (
                BinaryOperation::Multiply,
                BasicValueEnum::IntValue(left),
                BasicValueEnum::IntValue(right),
            ) => BasicValueEnum::IntValue(builder.build_int_mul(left, right, "intmul")),
            (
                BinaryOperation::Multiply,
                BasicValueEnum::FloatValue(left),
                BasicValueEnum::FloatValue(right),
            ) => BasicValueEnum::FloatValue(builder.build_float_mul(left, right, "floatmul")),
            (
                BinaryOperation::Divide,
                BasicValueEnum::IntValue(left),
                BasicValueEnum::IntValue(right),
            ) => BasicValueEnum::IntValue(builder.build_int_signed_div(left, right, "intdiv")),
            (
                BinaryOperation::Divide,
                BasicValueEnum::FloatValue(left),
                BasicValueEnum::FloatValue(right),
            ) => BasicValueEnum::FloatValue(builder.build_float_div(left, right, "floatdiv")),
            (operation, BasicValueEnum::IntValue(left), BasicValueEnum::IntValue(right)) => {
                let op = match operation {
                    BinaryOperation::Equals => IntPredicate::EQ,
                    BinaryOperation::LessThan => IntPredicate::SLT,
                    BinaryOperation::GreaterThan => IntPredicate::SGT,
                    BinaryOperation::LessThanEquals => IntPredicate::SLE,
                    BinaryOperation::GreaterThanEquals => IntPredicate::SGE,
                    BinaryOperation::Or => {
                        return Ok(BasicValueEnum::IntValue(
                            builder.build_or(left, right, "boolor"),
                        ));
                    }
                    BinaryOperation::And => {
                        return Ok(BasicValueEnum::IntValue(
                            builder.build_and(left, right, "booland"),
                        ));
                    }
                    _ => unreachable!(),
                };

                BasicValueEnum::IntValue(builder.build_int_compare(op, left, right, "intcmp"))
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

                BasicValueEnum::IntValue(builder.build_float_compare(op, left, right, "floatcmp"))
            }
            _ => unreachable!(),
        };

        Ok(value)
    }
}

impl<'a, 'ctx> CodeGeneratorFunction<'a, 'ctx> for TypedIf<'a> {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error<'a>;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: FunctionValue<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let then_block = context.append_basic_block(current_function, "ifthen");
        let else_block = context.append_basic_block(current_function, "ifelse");
        let end_block = context.append_basic_block(current_function, "ifend");

        let condition = self.condition.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
            env,
        )?;
        builder.build_conditional_branch(condition.into_int_value(), then_block, else_block);

        builder.position_at_end(then_block);
        let then_branch = self.then_branch.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
            env,
        )?;
        builder.build_unconditional_branch(end_block);

        if let Some(ref else_branch) = self.else_branch {
            builder.position_at_end(else_block);
            let else_branch = else_branch.code_gen_function(
                context,
                module,
                builder,
                current_function,
                interner,
                env,
            )?;
            builder.build_unconditional_branch(end_block);
            builder.position_at_end(end_block);
            let phi = builder.build_phi(then_branch.get_type(), "ifphi");
            phi.add_incoming(&[(&then_branch, then_block), (&else_branch, else_block)]);
            Ok(phi.as_basic_value())
        } else {
            builder.position_at_end(else_block);
            builder.build_unconditional_branch(end_block);
            builder.position_at_end(end_block);
            Ok(BasicValueEnum::IntValue(context.i64_type().const_zero()))
        }
    }
}

impl<'a, 'ctx> CodeGeneratorFunction<'a, 'ctx> for TypedLet<'a> {
    type Output = ();
    type Error = Error<'a>;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: FunctionValue<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let value = self.expression.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
            env,
        )?;
        let id = self.name.1.id;
        if env.get(id).is_none() {
            let pointer = builder.build_alloca(value.get_type(), interner.get(id));
            env.define(id, pointer);
        }

        let pointer = env.get(id).unwrap();
        builder.build_store(pointer, value);

        Ok(())
    }
}

impl<'a, 'ctx> CodeGeneratorFunction<'a, 'ctx> for TypedApplication<'a> {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error<'a>;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: FunctionValue<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let function_name = interner.get(self.function_name.1);
        let function_call = format!("call_{}", function_name);
        let function = module.get_function(&function_name).unwrap();
        let mut arguments = Vec::new();
        for parameter in &self.parameters {
            arguments.push(parameter.code_gen_function(
                context,
                module,
                builder,
                current_function,
                interner,
                env,
            )?);
        }

        Ok(builder
            .build_call(function, &arguments, &function_call)
            .try_as_basic_value()
            .left()
            .unwrap_or_else(|| BasicValueEnum::IntValue(context.i64_type().const_zero())))
    }
}

impl<'a, 'ctx> CodeGeneratorFunction<'a, 'ctx> for TypedWhile<'a> {
    type Output = ();
    type Error = Error<'a>;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: FunctionValue<'ctx>,
        interner: &mut Interner<String>,
        env: &mut Environment<PointerValue<'ctx>>,
    ) -> Result<Self::Output, Self::Error> {
        let condition_check_block = context.append_basic_block(current_function, "condition_check");
        let loop_block = context.append_basic_block(current_function, "loop");
        let after_loop_block = context.append_basic_block(current_function, "after_loop");
        builder.build_unconditional_branch(condition_check_block);

        builder.position_at_end(condition_check_block);
        let condition = self.condition.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
            env,
        )?;

        builder.build_conditional_branch(condition.into_int_value(), loop_block, after_loop_block);
        builder.position_at_end(loop_block);
        self.expression.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
            env,
        )?;

        builder.build_unconditional_branch(condition_check_block);
        builder.position_at_end(after_loop_block);
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
