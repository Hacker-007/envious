use std::{fs, path::PathBuf};

use interner::Interner;
use lexer::{token::TokenKind, Lexer};
use parser::Parser;
use semantic_analyzer::type_checker::TypeChecker;

mod error;
mod interner;
mod lexer;
mod parser;
mod semantic_analyzer;
macro_rules! handle_errors {
    ($errors: ident) => {
        $errors.iter().for_each(|error| error.report_error());
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: PathBuf = "envious/src/test.envy".into();
    let input = fs::read_to_string(&path)?;
    let file_name = path.file_name().unwrap().to_str().unwrap();
    run_envy(file_name, input.as_bytes());
    Ok(())
}

fn run_envy(file_name: &str, bytes: &[u8]) {
    let mut interner = Interner::default();
    let (tokens, errors) = Lexer::new(file_name.to_string(), bytes).get_tokens(&mut interner);
    handle_errors!(errors);

    let filtered_tokens = tokens
        .into_iter()
        .filter(|token| token.1 != TokenKind::Whitespace)
        .collect::<Vec<_>>();
    let (mut expressions, errors) = Parser::new(filtered_tokens).parse_program();
    handle_errors!(errors);

    for mut expression in expressions.iter_mut() {
        if let Err(error) = TypeChecker::analyze(&mut interner, &mut expression) {
            error.report_error();
        } else {
            println!("{:#?}", expression);
        }
    }
}
