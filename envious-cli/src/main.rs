use std::{error::Error, fs, time::Instant};

use envious::{
    codegen::Runner,
    error::reporter::{ErrorReporter, Reporter},
    interner::Interner,
    lexer::{token::TokenKind, Lexer},
    parser::Parser,
    semantic_analyzer::type_check::TypeCheck,
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
        let file_name = file.file_name().unwrap().to_str().unwrap();
        error_reporter.add(file_name, source);
        let bytes = source.as_bytes();
        let compilation_start = Instant::now();
        compile_code(&error_reporter, &mut interner, file_name.to_string(), bytes);
        println!(
            "Finished full compilation process after {} seconds",
            compilation_start.elapsed().as_secs_f64()
        );
    }

    Ok(())
}

fn compile_code(
    error_reporter: &ErrorReporter,
    interner: &mut Interner<String>,
    file_name: String,
    bytes: &[u8],
) -> Option<()> {
    let tokens = time("lexical analysis", &error_reporter, || {
        Lexer::new(&file_name, bytes).get_tokens(interner)
    })?;

    let filtered_tokens = tokens
        .into_iter()
        .filter(|token| !matches!(token.1, TokenKind::Whitespace(_)))
        .peekable();

    let program = time("syntactic analysis", &error_reporter, || {
        Parser::new(filtered_tokens).parse()
    })?;

    let typed_program = time("semantic analysis", &error_reporter, || program.check())?;

    time("compilation", &error_reporter, || {
        Runner::new(typed_program).run(&file_name, interner)
    })?;

    Some(())
}

fn time<O: Reporter>(
    name: &str,
    error_reporter: &ErrorReporter,
    function: impl FnOnce() -> O,
) -> Option<O::Output> {
    let start_time = Instant::now();
    println!("Running process `{}`", name);
    let value = function();
    println!(
        "Finished process `{}` after {} seconds",
        name,
        start_time.elapsed().as_secs_f64()
    );
    value.report(error_reporter)
}
