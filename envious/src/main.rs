//! The compiler for the Envious programming language.

use std::{fs, path::PathBuf};

use codegen::CodeGenerator;
use error::reporter::{ErrorReporter, Reporter};
use inkwell::context::Context;
use interner::Interner;
use lexer::{token::TokenKind, Lexer};
use parser::Parser;
use semantic_analyzer::type_checker::TypeChecker;

mod codegen;
mod error;
mod interner;
mod lexer;
mod parser;
mod semantic_analyzer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: PathBuf = "envious/src/test.envy".into();
    let input = fs::read_to_string(&path)?;
    let file_name = path.file_name().unwrap().to_str().unwrap();
    run_envy(file_name, input);
    Ok(())
}

fn run_envy(file_name: &str, input: String) {
    let error_reporter = ErrorReporter::new(vec![(file_name, input.clone())]);
    let mut interner = Interner::default();
    let bytes = input.as_bytes();
    let (tokens, errors) = Lexer::new(file_name.to_string(), bytes).get_tokens(&mut interner);
    if errors.report(&error_reporter) {
        return;
    }

    let filtered_tokens = tokens
        .into_iter()
        .filter(|token| !matches!(token.1, TokenKind::Whitespace(_)))
        .peekable();
    let (mut expressions, errors) = Parser::new(filtered_tokens).parse_program();
    if errors.report(&error_reporter) {
        return;
    }

    let errors = TypeChecker::analyze_program(&mut interner, &mut expressions);
    if errors.report(&error_reporter) {
        return;
    }

    let context = Context::create();
    let module = context.create_module("envious");
    let builder = context.create_builder();

    let return_type = context.i64_type();
    let main_function_type = return_type.fn_type(&[], false);
    let main_function = Some(module.add_function("main", main_function_type, None));
    let mut code_generator = CodeGenerator::new(&context, &module, &builder, &main_function);
    code_generator.compile(&mut interner, &expressions).report(&error_reporter);
}
