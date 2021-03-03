// use std::{collections::HashMap, u64};

// use inkwell::{
//     builder::Builder,
//     context::Context,
//     module::Module,
//     values::{BasicValue, BasicValueEnum, FunctionValue, VectorValue},
//     OptimizationLevel,
// };

// use crate::{
//     error::Error,
//     interner::Interner,
//     parser::expression::{BinaryOperation, Expression, ExpressionKind, UnaryOperation},
// };

// /// Struct that transforms the IR into LLVM code.
// /// The `CodeGenerator` expects that the code has been
// /// analyzed properly and does not check for any
// /// extraneous possibilities during code generation
// /// to reduce the time taken.
// pub struct CodeGenerator<'a, 'ctx> {
//     context: &'ctx Context,
//     module: &'a Module<'ctx>,
//     builder: &'a Builder<'ctx>,
//     current_function: &'a Option<FunctionValue<'ctx>>,
//     vars: HashMap<usize, BasicValueEnum<'ctx>>,
// }

// impl<'a, 'ctx> CodeGenerator<'a, 'ctx> {
//     pub fn new(
//         context: &'ctx Context,
//         module: &'a Module<'ctx>,
//         builder: &'a Builder<'ctx>,
//         current_function: &'a Option<FunctionValue<'ctx>>,
//     ) -> Self {
//         Self {
//             context,
//             module,
//             builder,
//             current_function,
//             vars: HashMap::new(),
//         }
//     }

//     /// Compiles the IR or slice of `Expression` into LLVM code.
//     /// This function will only result in an error if there is a problem with
//     /// the LLVM backend.
//     ///
//     /// # Arguments
//     /// * `interner` - The `Interner` that stores all of the different string literals.
//     /// * `expressions` - The different `Expression`'s that have been validated and are ready to be compiled.
//     pub fn compile(
//         &mut self,
//         interner: &mut Interner<String>,
//         expressions: &[Expression],
//     ) -> Result<(), Error> {
//         let main_function = self.get_function()?;
//         let entry = self.context.append_basic_block(main_function, "entry");
//         self.builder.position_at_end(entry);
//         let last = expressions.iter().fold(
//             Ok(BasicValueEnum::IntValue(
//                 self.context.i64_type().const_int(0, false),
//             )),
//             |_, expression| self.compile_expression(interner, expression),
//         );
//         let return_value: &dyn BasicValue<'ctx> = &last?;
//         self.builder.build_return(Some(return_value));
//         main_function.verify(true);
//         unsafe {
//             let execution_engine = self
//                 .module
//                 .create_jit_execution_engine(OptimizationLevel::None)
//                 .unwrap();
//             println!(
//                 "{:#?}",
//                 execution_engine
//                     .get_function::<unsafe extern "C" fn() -> i64>("main")
//                     .unwrap()
//                     .call()
//             );
//         }

//         main_function.print_to_stderr();
//         Ok(())
//     }

//     /// Compiles a single `Expression` and returns the `BasicValueEnum` that is
//     /// generated as a result of compiling the `Expression`.
//     ///
//     /// # Arguments
//     /// * `interner` - The `Interner` that stores all of the different string literals.
//     /// * `expression` - The `Expression` to compile.
//     fn compile_expression(
//         &mut self,
//         interner: &mut Interner<String>,
//         expression: &Expression,
//     ) -> Result<BasicValueEnum<'ctx>, Error> {
//         let value = match expression.1 {
//             ExpressionKind::Int(value) => {
//                 let llvm_int = self.context.i64_type().const_int(value.abs() as u64, false);
//                 if value < 0 {
//                     BasicValueEnum::IntValue(llvm_int.const_neg())
//                 } else {
//                     BasicValueEnum::IntValue(llvm_int)
//                 }
//             }
//             ExpressionKind::Float(value) => {
//                 BasicValueEnum::FloatValue(self.context.f64_type().const_float(value))
//             }
//             ExpressionKind::Boolean(value) => {
//                 BasicValueEnum::IntValue(self.context.bool_type().const_int(value as u64, false))
//             }
//             ExpressionKind::String(id) => BasicValueEnum::VectorValue(
//                 self.context.const_string(interner.get(id).as_bytes(), true),
//             ),
//             ExpressionKind::Identifier(id) => *self.vars.get(&id).unwrap(),
//             ExpressionKind::Unary {
//                 ref operation,
//                 ref expression,
//             } => self.compile_unary_expression(interner, operation, expression)?,
//             ExpressionKind::Binary {
//                 ref operation,
//                 ref left,
//                 ref right,
//             } => self.compile_binary_expression(interner, operation, left, right)?,
//             ExpressionKind::If {
//                 ref condition,
//                 ref then_branch,
//                 ref else_branch,
//             } => self.compile_if_expression(
//                 interner,
//                 condition,
//                 then_branch,
//                 else_branch.as_deref(),
//             )?,
//             ref expression => todo!("{:#?}", expression),
//         };

//         Ok(value)
//     }

//     /// Compiles a unary expression and returns the `BasicValueEnum` that is
//     /// generated as a result of compiling it.
//     ///
//     /// # Arguments
//     /// * `interner` - The `Interner` that stores all of the different string literals.
//     /// * `operation` - The `UnaryOperation` applied.
//     /// * `expression` - The `Expression` to compile.
//     fn compile_unary_expression(
//         &mut self,
//         interner: &mut Interner<String>,
//         operation: &UnaryOperation,
//         expression: &Expression,
//     ) -> Result<BasicValueEnum<'ctx>, Error> {
//         let value = match (operation, self.compile_expression(interner, expression)?) {
//             (UnaryOperation::Minus, BasicValueEnum::IntValue(value)) => {
//                 BasicValueEnum::IntValue(self.builder.build_int_neg(value, "intneg"))
//             }
//             (UnaryOperation::Minus, BasicValueEnum::FloatValue(value)) => {
//                 BasicValueEnum::FloatValue(self.builder.build_float_neg(value, "floatneg"))
//             }
//             (UnaryOperation::Not, BasicValueEnum::IntValue(value)) => {
//                 BasicValueEnum::IntValue(self.builder.build_not(value, "boolnot"))
//             }
//             _ => unreachable!(),
//         };

//         Ok(value)
//     }

//     /// Compiles a binary expression and returns the `BasicValueEnum` that is
//     /// generated as a result of compiling it.
//     ///
//     /// # Arguments
//     /// * `interner` - The `Interner` that stores all of the different string literals.
//     /// * `operation` - The `BinaryOperation` applied.
//     /// * `left` - The left `Expression` to compile.
//     /// * `right` - The right `Expression` to compile.
//     fn compile_binary_expression(
//         &mut self,
//         interner: &mut Interner<String>,
//         operation: &BinaryOperation,
//         left: &Expression,
//         right: &Expression,
//     ) -> Result<BasicValueEnum<'ctx>, Error> {
//         let (left, right) = (
//             self.compile_expression(interner, left)?,
//             self.compile_expression(interner, right)?,
//         );

//         let value = match (operation, left, right) {
//             (
//                 BinaryOperation::Plus,
//                 BasicValueEnum::IntValue(left),
//                 BasicValueEnum::IntValue(right),
//             ) => BasicValueEnum::IntValue(self.builder.build_int_add(left, right, "intadd")),
//             (
//                 BinaryOperation::Plus,
//                 BasicValueEnum::FloatValue(left),
//                 BasicValueEnum::FloatValue(right),
//             ) => BasicValueEnum::FloatValue(self.builder.build_float_add(left, right, "floatadd")),
//             (
//                 BinaryOperation::Plus,
//                 BasicValueEnum::VectorValue(left),
//                 BasicValueEnum::VectorValue(right),
//             ) if left.is_const_string() && right.is_const_string() => {
//                 let concated = format!("{}{}", convert_to_str(left), convert_to_str(right));
//                 BasicValueEnum::VectorValue(self.context.const_string(concated.as_bytes(), true))
//             }
//             (
//                 BinaryOperation::Minus,
//                 BasicValueEnum::IntValue(left),
//                 BasicValueEnum::IntValue(right),
//             ) => BasicValueEnum::IntValue(self.builder.build_int_sub(left, right, "intsub")),
//             (
//                 BinaryOperation::Minus,
//                 BasicValueEnum::FloatValue(left),
//                 BasicValueEnum::FloatValue(right),
//             ) => BasicValueEnum::FloatValue(self.builder.build_float_sub(left, right, "floatsub")),
//             (
//                 BinaryOperation::Multiply,
//                 BasicValueEnum::IntValue(left),
//                 BasicValueEnum::IntValue(right),
//             ) => BasicValueEnum::IntValue(self.builder.build_int_mul(left, right, "intmul")),
//             (
//                 BinaryOperation::Multiply,
//                 BasicValueEnum::FloatValue(left),
//                 BasicValueEnum::FloatValue(right),
//             ) => BasicValueEnum::FloatValue(self.builder.build_float_mul(left, right, "floatmul")),
//             (
//                 BinaryOperation::Divide,
//                 BasicValueEnum::IntValue(left),
//                 BasicValueEnum::IntValue(right),
//             ) => BasicValueEnum::IntValue(self.builder.build_int_signed_div(left, right, "intdiv")),
//             (
//                 BinaryOperation::Divide,
//                 BasicValueEnum::FloatValue(left),
//                 BasicValueEnum::FloatValue(right),
//             ) => BasicValueEnum::FloatValue(self.builder.build_float_div(left, right, "floatdiv")),
//             _ => unreachable!(),
//         };

//         Ok(value)
//     }

//     /// Compiles an if expression and returns the `BasicValueEnum` that is
//     /// generated as a result of compiling it.
//     ///
//     /// # Arguments
//     /// * `interner` - The `Interner` that stores all of the different string literals.
//     /// * `condition` - The `Expression` that represents represents the condition.
//     /// * `then_branch` - The `Expression` to compile if the condition is true.
//     /// * `else_branch` - The `Expression` to compile if the condition is false.
//     fn compile_if_expression(
//         &mut self,
//         interner: &mut Interner<String>,
//         condition: &Expression,
//         then_branch: &Expression,
//         else_branch: Option<&Expression>,
//     ) -> Result<BasicValueEnum<'ctx>, Error> {
//         let function = self.get_function()?;
//         if let Some(else_branch) = else_branch {
//             let then_block = self.context.append_basic_block(function, "if_true");
//             let else_block = self.context.append_basic_block(function, "if_false");
//             let end_block = self.context.append_basic_block(function, "if_end");

//             let condition = self.compile_expression(interner, condition)?;
//             self.builder.build_conditional_branch(
//                 condition.into_int_value(),
//                 then_block,
//                 else_block,
//             );

//             self.builder.position_at_end(then_block);
//             let then_branch = self.compile_expression(interner, then_branch)?;
//             self.builder.build_unconditional_branch(end_block);

//             self.builder.position_at_end(else_block);
//             let else_branch = self.compile_expression(interner, else_branch)?;
//             self.builder.build_unconditional_branch(end_block);

//             self.builder.position_at_end(end_block);
//             let phi = self.builder.build_phi(then_branch.get_type(), "phi");
//             phi.add_incoming(&[(&then_branch, then_block), (&else_branch, else_block)]);
//             Ok(phi.as_basic_value())
//         } else {
//             let then_block = self.context.append_basic_block(function, "if_true");
//             let end_block = self.context.append_basic_block(function, "if_end");

//             let condition = self.compile_expression(interner, condition)?;
//             self.builder.build_conditional_branch(
//                 condition.into_int_value(),
//                 then_block,
//                 end_block,
//             );

//             self.builder.position_at_end(then_block);
//             self.compile_expression(interner, then_branch)?;
//             self.builder.build_unconditional_branch(end_block);

//             self.builder.position_at_end(end_block);
//             Ok(BasicValueEnum::IntValue(
//                 self.context.i64_type().const_zero(),
//             ))
//         }
//     }

//     /// Unwraps the current function and returns the `FunctionValue`
//     /// that refers to the current function being compiled.
//     fn get_function(&self) -> Result<FunctionValue<'ctx>, Error> {
//         self.current_function.ok_or_else(|| Error::ExpectedFunction)
//     }
// }

// /// Converts a `VectorValue` to a &str. This function expects that
// /// the check has already been performed on this value.
// ///
// /// # Arguments
// /// * `value` - The `VectorValue` to convert.
// fn convert_to_str(value: VectorValue) -> String {
//     value.get_string_constant().to_str().unwrap().to_string()
// }

pub mod code_generator;
pub mod runner;
pub use runner::Runner;
