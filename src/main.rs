/// The Compiler module, which walks the AST generated and creates the various different parts of the .dark file.
pub mod compiler;

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

use crate::compiler::Compiler;
use crate::lexer::Lexer;
use cli::arguments::Arguments;
use parser::Parser;

fn main() {
    match Arguments::new() {
        Ok(args) => {
            if let Err(error) = args.run(run) {
                println!("{}", error);
            }
        }
        Err(error) => println!("{}", error.prettify("")),
    }
}

fn run(contents: &str, args: &Arguments) -> Result<(), String> {
    let tokens = Lexer::default()
        .lex(contents)
        .map_err(|error| error.prettify(contents))?;

    let ast = Parser::new(tokens)
        .parse()
        .map_err(|error| error.prettify(contents))?;

    Compiler::new()
        .compile(&args.get_path().ok_or("The REPL Is Not Yet Supported.".to_owned())?.replace(".envy", ".dark"), ast)
        .map_err(|error| error.prettify(contents))?;

    Ok(())
}
