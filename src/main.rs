extern crate crossterm;

/// The code_generation module, which walks the AST generated and creates the various different parts of the .dark file.
pub mod code_generation;

/// The semantic_analyzer module, which performs semantic analysis on the code. This includes type checking and proper code.
pub mod semantic_analyzer;

/// The parser module, which creates a AST that can be walked using the Visitor patter. The parser parses all of the tokens from the lexer.
pub mod parser;

/// The ast module, which contains the Expression struct and the ExpressionKind enum. These describe the various expressions that can occur in the program.
pub mod ast;

/// The lexer module, which creates a vector of all of the tokens in the input. This input may come from either a file or a REPL.
pub mod lexer;

/// The std module, which contains all of the standard library functions. There is still no module system, so all functions get grouped together.
/// However, this will change.
pub mod std;

/// The tokens module, which contains the Token struct and the TokenKind enum. These describe the various tokens that can be recognized.
pub mod tokens;

/// The errors module, which contains the Error struct and the ErrorKind enum. These describe the various errors that could occur during the program execution.
mod errors;

/// The cli module, which contains all of the arguments that were passed in to the program.
mod cli;

/// The repl_helpers module, which contains the REPL for the Envious programming language.
mod repl_helpers;

use crate::code_generation::CodeGenerator;
use crate::lexer::Lexer;
use cli::{arguments::Arguments, runner};
use parser::Parser;
use semantic_analyzer::type_checker::TypeChecker;
use crate::std::standard_library::StandardLibrary;

fn main() {
    if let Err(error) = runner(run) {
        println!("{}", error)
    }
}

/// This runs the lexer, the parser, and the code generator on the contents passed in.
/// An error is reported if any part of the process returns an error.
fn run(contents: &str, path: &str, args: &Arguments) -> Result<String, String> {
    let tokens = Lexer::default()
        .lex(contents)
        .map_err(|error| error.prettify(contents))?;
    
    let standard_library = StandardLibrary::new();
    let mut type_checker = TypeChecker::new();
    let ast = Parser::new(tokens)
        .parse(&standard_library, &mut type_checker)
        .map_err(|error| error.prettify(contents))?;

    
    type_checker
        .perform_type_checking(&ast, &standard_library)
        .map_err(|error| error.prettify(contents))?;

    CodeGenerator::new(args.format_output(), type_checker.user_defined_functions.keys().cloned().collect())
        .generate_code(Some(&path.replace(".envy", ".dark")), ast, &standard_library)
        .map_err(|error| error.prettify(contents))
}
