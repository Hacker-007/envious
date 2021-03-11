use std::path::Path;

use codegen::code_generator::CodeGenerator;
use environment::Environment;
use error::Error;
use inkwell::{OptimizationLevel, context::Context, passes::{PassManager, PassManagerBuilder}, targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine}};
use interner::Interner;
use parser::typed_ast::TypedProgram;

pub mod codegen;
pub mod environment;
pub mod error;
pub mod interner;
pub mod lexer;
pub mod parser;
pub mod semantic_analyzer;

pub fn run<'a>(
    program: &TypedProgram<'a>,
    module_name: &str,
    interner: &mut Interner<String>,
) -> Result<(), Vec<Error<'a>>> {
    let context = Context::create();
    let module = context.create_module(module_name);
    let builder = context.create_builder();

    let mut value_env = Environment::default();
    program.code_gen(&context, &module, &builder, interner, &mut value_env)?;
    
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
    let target_machine = target.create_target_machine(
        &target_triple,
        "generic",
        "",
        OptimizationLevel::Default,
        RelocMode::Default,
        CodeModel::Default
    ).unwrap();
    
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());
    target_machine.add_analysis_passes(&pass_manager);
    target_machine.write_to_file(&module, FileType::Object, Path::new("./test.o")).unwrap();
    Ok(())
}
