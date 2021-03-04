use inkwell::context::Context;

use crate::{error::Error, interner::Interner, parser::ast::Program};

use super::code_generator::CodeGenerator;

/// A struct that handles running the final stage of the compiler.
/// This ensures that other packages that handles maintaining either
/// the CLI or the REPL do not have to import the LLVM library.
pub struct Runner<'a> {
    program: Program<'a>,
}

impl<'a> Runner<'a> {
    pub fn new(program: Program<'a>) -> Self {
        Self { program }
    }

    /// Generates the code for the given ast and provides a result.
    /// This function will most likely to change in the future to
    /// handle multiple threads running or to handle compiling many files that
    /// may have references to each other.
    ///
    /// # Arguments
    /// `module_name` - The name of the module to use.
    /// `interner` - The `Interner` used to store all string literals.
    pub fn run(
        &mut self,
        module_name: &str,
        interner: &mut Interner<String>,
    ) -> Result<(), Vec<Error<'a>>> {
        let context = Context::create();
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        // let return_type = context.i64_type();
        // let main_function_type = return_type.fn_type(&[], false);
        // let main_function = Some(module.add_function("main", main_function_type, None));
        self.program
            .code_gen(&context, &module, &builder, interner)?;
        module.print_to_stderr();
        Ok(())
        // CodeGenerator::new(&context, &module, &builder, &main_function).compile(interner, &self.ast)
    }
}
