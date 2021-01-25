use std::fs;

use lexical_analysis::Lexer;

mod error;
mod lexical_analysis;
mod span;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("src/test.envy")?;
    for token in Lexer::new(input.as_bytes()) {
        match token {
            Ok(token) => println!("{:#?}", token),
            Err(error) => {
                println!("{:#?}", error);
            }
        }
    }

    Ok(())
}
