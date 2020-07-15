/// The CodeGenerator module, which walks the AST generated and creates the various different parts of the .dark file.
pub mod code_generation;

/// The Parser module, which creates a AST that can be walked using the Visitor patter. The parser parses all of the tokens from the lexer.
pub mod parser;

/// The AST module, which contains the Expression struct and the ExpressionKind enum. These describe the various expressions that can occur in the program.
pub mod ast;

/// The Lexer module, which creates a vector of all of the tokens in the input. This input may come from either a file or a REPL.
pub mod lexer;

/// The Tokens module, which contains the Token struct and the TokenKind enum. These describe the various tokens that can be recognized.
pub mod tokens;

/// The Errors module, which contains the Error struct and the ErrorKind enum. These describe the various errors that could occur during the program execution.
mod errors;

/// The CLI module, which contains all of the arguments that were passed in to the program.
mod cli;

use crate::code_generation::CodeGenerator;
use crate::lexer::Lexer;
use cli::{arguments::Arguments, runner};
use parser::Parser;

fn main() {
    if let Err(error) = runner(run) {
        println!("{}", error)
    }
}

/// This runs the lexer, the parser, and the code generator on the contents passed in.
/// An error is reported if any part of the process returns an error.
fn run(contents: &str, path: &str, args: &Arguments) -> Result<(), String> {
    let tokens = Lexer::default()
        .lex(contents)
        .map_err(|error| error.prettify(contents))?;

    let ast = Parser::new(tokens)
        .parse()
        .map_err(|error| error.prettify(contents))?;

    CodeGenerator::new(args.format_output())
        .generate_code(&path.replace(".envy", ".dark"), ast)
        .map_err(|error| error.prettify(contents))?;

    Ok(())
}
