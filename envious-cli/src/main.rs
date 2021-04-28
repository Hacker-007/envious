use std::{error::Error, fs, time::Instant};

use envyc::{
    compile,
    environment::Environment,
    error::reporter::{ErrorReporter, Reporter},
    filter_tokens,
    function_table::FunctionTable,
    interner::Interner,
    lex, parse, type_check,
};
use options::Options;

mod options;
use structopt::StructOpt;

pub fn main() -> Result<(), Box<dyn Error>> {
    let options: Options = Options::from_args();
    if options.files.is_empty() {
        println!("An input file must be provided.");
        return Ok(());
    }

    let mut error_reporter = ErrorReporter::new(vec![]);
    let mut interner = Interner::default();
    let mut sources = vec![];
    for file in &options.files {
        if !file.exists() {
            println!(
                "{} could not found or access was denied.",
                file.to_str().unwrap()
            );
            return Ok(());
        }

        let source = fs::read_to_string(&file)?;
        sources.push(source);
    }

    for (file, source) in options.files.iter().zip(sources.iter()) {
        let module_name = file.file_stem().unwrap().to_str().unwrap();
        let file_path = file.as_os_str().to_str().unwrap();
        error_reporter.add(&file_path, source);
        let output_path = file.parent().unwrap().join(format!("{}.o", module_name));
        let output_file_path = output_path.as_os_str().to_str().unwrap();
        let bytes = source.as_bytes();
        let compilation_start = Instant::now();
        compile_code(
            &error_reporter,
            &mut interner,
            &module_name,
            &file_path,
            output_file_path,
            bytes,
        );
        println!(
            "Finished full compilation process for file `{}` after {} seconds.",
            file_path,
            compilation_start.elapsed().as_secs_f64()
        );
    }

    Ok(())
}

fn compile_code(
    error_reporter: &ErrorReporter,
    interner: &mut Interner<String>,
    module_name: &str,
    file_path: &str,
    output_file_path: &str,
    bytes: &[u8],
) -> Option<()> {
    let tokens = time("Lexing", &error_reporter, || {
        lex(file_path, bytes, interner)
    })?;

    let filtered_tokens = filter_tokens(tokens);
    let program = time("Parsing", &error_reporter, || parse(filtered_tokens))?;

    let mut type_env = Environment::default();
    let mut function_table = FunctionTable::default();
    let typed_program = time("Checking", &error_reporter, || {
        type_check(program, &mut type_env, &mut function_table)
    })?;

    time("Compiling", &error_reporter, || {
        compile(&typed_program, module_name, output_file_path, interner)
    })?;

    Some(())
}

fn time<O: Reporter>(
    name: &str,
    error_reporter: &ErrorReporter,
    function: impl FnOnce() -> O,
) -> Option<O::Output> {
    let start = Instant::now();
    let value = function();
    println!(
        "Process `{}` took {} seconds.",
        name,
        start.elapsed().as_secs_f64()
    );
    value.report(error_reporter)
}
