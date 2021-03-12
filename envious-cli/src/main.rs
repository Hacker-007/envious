use std::{error::Error, fs, thread, time::{Duration, Instant}};

use envyc::{environment::Environment, error::reporter::{ErrorReporter, Reporter}, interner::Interner, lexer::{token::TokenKind, Lexer}, parser::Parser, run, semantic_analyzer::type_check::TypeCheck};
use options::Options;

mod options;

use progress_bar::{color::{Color, Style}, progress_bar::ProgressBar};
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
        let file_stem = file.file_stem().unwrap().to_str().unwrap();
        error_reporter.add(file_name, source);
        let bytes = source.as_bytes();
        let compilation_start = Instant::now();
        compile_code(&error_reporter, &mut interner, file_name, file_stem, bytes);
        println!(
            "Finished full compilation process after {} seconds",
            compilation_start.elapsed().as_millis() as f32 / 1000.0 - 2.0
        ); 
    }

    Ok(())
}

fn compile_code(
    error_reporter: &ErrorReporter,
    interner: &mut Interner<String>,
    file_name: &str,
    file_stem: &str,
    bytes: &[u8],
) -> Option<()> {
    let mut progress_bar = ProgressBar::new(4);
    let tokens = time(&mut progress_bar, "Lexing", &file_name, &error_reporter, || {
        Lexer::new(file_name, bytes).get_tokens(interner)
    })?;

    let filtered_tokens = tokens
        .into_iter()
        .filter(|token| !matches!(token.1, TokenKind::Whitespace(_)))
        .peekable();

    let program = time(&mut progress_bar, "Parsing", &file_name, &error_reporter, || {
        Parser::new(filtered_tokens).parse()
    })?;

    let mut type_env = Environment::default();
    let typed_program = time(&mut progress_bar, "Checking", file_name, &error_reporter, || {
        program.check(&mut type_env)
    })?;

    time(&mut progress_bar, "Compiling", &file_name, &error_reporter, || {
        run(&typed_program, &file_name, file_stem, interner)
    })?;

    Some(())
}

fn time<O: Reporter>(
    progress_bar: &mut ProgressBar,
    name: &str,
    value: &str,
    error_reporter: &ErrorReporter,
    function: impl FnOnce() -> O,
) -> Option<O::Output> {
    thread::sleep(Duration::from_millis(500));
    progress_bar.print_info(name, value, Color::Green, Style::Bold);
    let value = function();
    if value.is_err() {
        progress_bar.finalize();
    } else {
        progress_bar.inc();
    }
    
    value.report(error_reporter)
}
