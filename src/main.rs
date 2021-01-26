use std::fs;

use lexer::{Lexer, token::TokenKind};
use parser::Parser;

mod error;
mod span;
mod lexer;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("src/test.envy")?;
    match Lexer::new(input.as_bytes()).get_tokens() {
        Ok(tokens) => {
            let filtered_tokens = tokens
                .into_iter()
                .filter(|token| token.1 != TokenKind::Whitespace)
                .collect::<Vec<_>>();
            match Parser::new(filtered_tokens).parse_program() {
                Ok(ast) => {
                    for expression in ast {
                        println!("{:#?}", expression);
                    }
                },
                Err(errors) => errors.iter().for_each(|error| error.report_error()),
            }              
        },
        Err(errors) => errors.iter().for_each(|error| error.report_error()),
    }

    Ok(())
}
