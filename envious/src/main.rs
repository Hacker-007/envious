use std::fs;

use interner::Interner;
use lexer::{token::TokenKind, Lexer};
use parser::Parser;
use semantic_analyzer::type_checker::TypeChecker;

mod error;
mod interner;
mod lexer;
mod parser;
mod semantic_analyzer;
mod span;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input =
        fs::read_to_string("/home/revanthp/projects/envious-upgraded/envious/src/test.envy")?;
    let mut interner = Interner::default();
    let (tokens, errors) = Lexer::new(
        "/home/revanthp/projects/envious-upgraded/envious/src/test.envy".to_string(),
        input.as_bytes(),
    )
    .get_tokens(&mut interner);
    errors.iter().for_each(|error| error.report_error());
    let filtered_tokens = tokens
        .into_iter()
        .filter(|token| token.1 != TokenKind::Whitespace)
        .collect::<Vec<_>>();
    let (expressions, errors) = Parser::new(filtered_tokens).parse_program();
    errors.iter().for_each(|error| error.report_error());
    for mut expression in expressions {
        if let Err(error) = TypeChecker::analyze(&mut interner, &mut expression) {
            error.report_error();
        }

        println!("{:#?}", expression);
    }

    Ok(())
}
