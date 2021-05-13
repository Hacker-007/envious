use std::{iter::Peekable, path::Path};

use codegen::code_generator::CodeGenerator;
use environment::Environment;
use error::Error;
use function_table::FunctionTable;
use inkwell::{
    context::Context,
    passes::{PassManager, PassManagerBuilder},
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};
use interner::Interner;
use lexer::{token::Token, Lexer};
use parser::{ast::Program, typed_ast::TypedProgram, Parser};
use semantic_analyzer::{type_check::TypeCheck, types::Type};

use crate::lexer::token::TokenKind;

pub mod codegen;
pub mod environment;
pub mod error;
pub mod function_table;
pub mod interner;
pub mod lexer;
pub mod parser;
pub mod semantic_analyzer;

pub fn lex<'a>(
    file_path: &'a str,
    bytes: &'a [u8],
    interner: &mut Interner<String>,
) -> Result<Vec<Token<'a>>, Vec<Error<'a>>> {
    Lexer::new(file_path, bytes).get_tokens(interner)
}

pub fn filter_tokens(tokens: Vec<Token>) -> Peekable<impl Iterator<Item = Token>> {
    tokens
        .into_iter()
        .filter(|token| !matches!(token.1, TokenKind::Whitespace(_)))
        .peekable()
}

pub fn parse<'a>(
    filtered_tokens: Peekable<impl Iterator<Item = Token<'a>>>,
) -> Result<Program<'a>, Vec<Error<'a>>> {
    Parser::new(filtered_tokens).parse()
}

pub fn type_check<'a>(
    program: Program<'a>,
    env: &mut Environment<Type>,
    function_table: &mut FunctionTable,
) -> Result<TypedProgram<'a>, Vec<Error<'a>>> {
    program.check(env, function_table)
}

pub struct Config<'a> {
    pub writing_to_file: bool,
    pub output_file_path: &'a str,
}

pub fn compile<'a>(
    program: &TypedProgram<'a>,
    module_name: &str,
    interner: &mut Interner<String>,
    config: Option<Config<'a>>,
) -> Result<String, Vec<Error<'a>>> {
    let context = Context::create();
    let module = context.create_module(module_name);
    let builder = context.create_builder();

    let mut value_env = Environment::default();
    CodeGenerator::new(&context, &module, &builder, interner, &mut value_env)
        .generate_program(program)?;

    let pass_manager_builder = PassManagerBuilder::create();
    pass_manager_builder.set_optimization_level(OptimizationLevel::Default);
    let pass_manager = PassManager::create(());
    pass_manager_builder.populate_module_pass_manager(&pass_manager);

    pass_manager.add_promote_memory_to_register_pass();
    pass_manager.add_instruction_combining_pass();
    pass_manager.add_reassociate_pass();
    pass_manager.add_gvn_pass();
    pass_manager.add_cfg_simplification_pass();
    pass_manager.run_on(&module);

    if let Some(config) = config {
        let target_triple = TargetMachine::get_default_triple();
        let init_config = InitializationConfig {
            asm_parser: true,
            asm_printer: true,
            base: true,
            disassembler: true,
            info: true,
            machine_code: true,
        };

        Target::initialize_all(&init_config);
        let target = Target::from_triple(&target_triple).unwrap();
        module.set_triple(&target_triple);
        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        module.set_data_layout(&target_machine.get_target_data().get_data_layout());
        target_machine.add_analysis_passes(&pass_manager);

        if config.writing_to_file {
            target_machine
                .write_to_file(
                    &module,
                    FileType::Object,
                    Path::new(config.output_file_path),
                )
                .unwrap();
        }
    }

    Ok(module.print_to_string().to_string())
}
