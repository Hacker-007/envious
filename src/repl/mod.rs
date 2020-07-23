//! The REPL struct maintains all of the code for REPL.
//! For now, the features are very limited and will expand in the future.
//! Some features would be:
//!     - syntax highlighting
//!     - autocompletion
//!     - support for multliple lines
//!     - ability to edit already written code.

use crate::{
    cli::arguments::Arguments, code_generation::CodeGenerator, lexer::Lexer, parser::Parser,
};
use console::{Key, Term};
use std::io;

pub struct Repl;

impl Repl {
    pub fn new() -> Repl {
        Repl
    }

    pub fn start_loop(&self, args: &Arguments) -> io::Result<()> {
        let term = Term::stdout();
        let mut input = String::new();
        let mut current = String::new();
        term.write_str("envious -> ")?;
        loop {
            let key = term.read_key()?;
            match key {
                Key::Char(';') => {
                    term.write_str(";\n\n")?;
                    input.push('\n');
                    match self.evaluate(&input, args) {
                        Ok(code) => term.write_str(&code)?,
                        Err(error) => {
                            input = input[0..(input.len() - current.len() - 1)].to_owned();
                            term.write_str(&error)?
                        }
                    }

                    current.clear();
                    term.write_str("\n\nenvious -> ")?;
                }
                Key::Char(ch) => {
                    term.write_str(&ch.to_string())?;
                    input.push(ch);
                    current.push(ch);
                }
                Key::Enter => {
                    term.write_str("\n         | ")?;
                    input.push('\n');
                    current.push('\n');
                }
                Key::Backspace => {
                    term.clear_chars(1)?;
                    if !input.is_empty() {
                        input.remove(input.len() - 1);
                    }

                    if !current.is_empty() {
                        current.remove(current.len() - 1);
                    }
                }
                Key::Unknown => {}
                Key::Escape => {
                    term.write_line("\nGoodbye.")?;
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn evaluate(&self, input: &str, args: &Arguments) -> Result<String, String> {
        let tokens = Lexer::default()
            .lex(input)
            .map_err(|error| error.prettify(input))?;

        let ast = Parser::new(tokens)
            .parse()
            .map_err(|error| error.prettify(input))?;

        CodeGenerator::new(args.format_output())
            .generate_code(None, ast)
            .map_err(|error| error.prettify(input))
    }
}
