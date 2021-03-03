use std::u64;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicType, BasicTypeEnum},
    values::{BasicValueEnum, FunctionValue, VectorValue},
};

use crate::{
    error::Error,
    interner::Interner,
    parser::{
        ast::{Function, Program},
        expression::{
            Binary, BinaryOperation, Expression, ExpressionKind, Identifier, If, Unary,
            UnaryOperation,
        },
    },
    semantic_analyzer::types::Type,
};

pub trait CodeGenerator<'ctx> {
    type Output;
    type Error;

    fn code_gen(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        interner: &mut Interner<String>,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait CodeGeneratorFunction<'ctx> {
    type Output;
    type Error;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: &FunctionValue<'ctx>,
        interner: &mut Interner<String>,
    ) -> Result<Self::Output, Self::Error>;
}

impl<'ctx> CodeGenerator<'ctx> for Program {
    type Output = ();
    type Error = Vec<Error>;

    fn code_gen(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        interner: &mut Interner<String>,
    ) -> Result<Self::Output, Self::Error> {
        let mut errors = vec![];
        for function in &self.functions {
            if let Err(error) = function.code_gen(context, module, builder, interner) {
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

impl<'ctx> CodeGenerator<'ctx> for Function {
    type Output = ();
    type Error = Error;

    fn code_gen(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        interner: &mut Interner<String>,
    ) -> Result<Self::Output, Self::Error> {
        let return_type = self.return_type.unwrap();
        let parameter_types = self
            .parameters
            .iter()
            .map(|parameter| parameter.ty)
            .map(|ty| convert_basic_type(ty, context))
            .collect::<Vec<_>>();

        let function_type = if let Type::Void = return_type {
            context.void_type().fn_type(&parameter_types, false)
        } else {
            convert_type(return_type, context).fn_type(&parameter_types, false)
        };

        let function = module.add_function(&interner.get(self.name), function_type, None);
        let entry = context.append_basic_block(function, "entry");
        builder.position_at_end(entry);

        let expression = self
            .body
            .code_gen_function(context, module, builder, &function, interner)?;
        if return_type == Type::Void {
            builder.build_return(None);
        } else {
            builder.build_return(Some(&expression));
        }

        function.verify(true);
        Ok(())
    }
}

impl<'ctx> CodeGeneratorFunction<'ctx> for Expression {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: &FunctionValue<'ctx>,
        interner: &mut Interner<String>,
    ) -> Result<Self::Output, Self::Error> {
        match self.1 {
            ExpressionKind::Int(value) => {
                let int = context.i64_type().const_int(value.abs() as u64, false);
                if value < 0 {
                    Ok(BasicValueEnum::IntValue(int.const_neg()))
                } else {
                    Ok(BasicValueEnum::IntValue(int))
                }
            }
            ExpressionKind::Float(value) => Ok(BasicValueEnum::FloatValue(
                context.f64_type().const_float(value),
            )),
            ExpressionKind::Boolean(value) => Ok(BasicValueEnum::IntValue(
                context.bool_type().const_int(value as u64, false),
            )),
            ExpressionKind::String(id) => Ok(BasicValueEnum::VectorValue(
                context.const_string(interner.get(id).as_bytes(), true),
            )),
            ExpressionKind::Identifier(ref inner) => {
                inner.code_gen_function(context, module, builder, current_function, interner)
            }
            ExpressionKind::Unary(ref inner) => {
                inner.code_gen_function(context, module, builder, current_function, interner)
            }
            ExpressionKind::Binary(ref inner) => {
                inner.code_gen_function(context, module, builder, current_function, interner)
            }
            ExpressionKind::If(ref inner) => {
                inner.code_gen_function(context, module, builder, current_function, interner)
            }
            _ => todo!(),
        }
    }
}

impl<'ctx> CodeGeneratorFunction<'ctx> for Identifier {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: &FunctionValue<'ctx>,
        interner: &mut Interner<String>,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

impl<'ctx> CodeGeneratorFunction<'ctx> for Unary {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: &FunctionValue<'ctx>,
        interner: &mut Interner<String>,
    ) -> Result<Self::Output, Self::Error> {
        let expression = self.expression.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
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

impl<'ctx> CodeGeneratorFunction<'ctx> for Binary {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: &FunctionValue<'ctx>,
        interner: &mut Interner<String>,
    ) -> Result<Self::Output, Self::Error> {
        let left =
            self.left
                .code_gen_function(context, module, builder, current_function, interner)?;
        let right =
            self.right
                .code_gen_function(context, module, builder, current_function, interner)?;
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
                BinaryOperation::Plus,
                BasicValueEnum::VectorValue(left),
                BasicValueEnum::VectorValue(right),
            ) if left.is_const_string() && right.is_const_string() => {
                let concated = format!("{}{}", convert_to_str(left), convert_to_str(right));
                BasicValueEnum::VectorValue(context.const_string(concated.as_bytes(), true))
            }
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
            _ => unreachable!(),
        };

        Ok(value)
    }
}

impl<'ctx> CodeGeneratorFunction<'ctx> for If {
    type Output = BasicValueEnum<'ctx>;
    type Error = Error;

    fn code_gen_function(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        current_function: &FunctionValue<'ctx>,
        interner: &mut Interner<String>,
    ) -> Result<Self::Output, Self::Error> {
        let then_block = context.append_basic_block(*current_function, "ifthen");
        let else_block = context.append_basic_block(*current_function, "ifelse");
        let end_block = context.append_basic_block(*current_function, "ifend");

        let condition = self.condition.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
        )?;
        builder.build_conditional_branch(condition.into_int_value(), then_block, else_block);

        builder.position_at_end(then_block);
        let then_branch = self.then_branch.code_gen_function(
            context,
            module,
            builder,
            current_function,
            interner,
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

fn convert_type(ty: Type, context: &Context) -> Box<dyn BasicType + '_> {
    match ty {
        Type::Int => Box::new(context.i64_type()),
        Type::Float => Box::new(context.f64_type()),
        Type::Boolean => Box::new(context.bool_type()),
        Type::String => Box::new(context.i8_type()),
        _ => unreachable!(),
    }
}

fn convert_basic_type(ty: Type, context: &Context) -> BasicTypeEnum {
    match ty {
        Type::Int => BasicTypeEnum::IntType(context.i64_type()),
        Type::Float => BasicTypeEnum::FloatType(context.f64_type()),
        Type::Boolean => BasicTypeEnum::IntType(context.bool_type()),
        Type::String => BasicTypeEnum::VectorType(context.i8_type().vec_type(0)),
        _ => unreachable!(),
    }
}

/// Converts a `VectorValue` to a &str. This function expects that
/// the check has already been performed on this value.
///
/// # Arguments
/// * `value` - The `VectorValue` to convert.
fn convert_to_str(value: VectorValue) -> String {
    value.get_string_constant().to_str().unwrap().to_string()
}
