use super::repl_trait::Repl;
use std::io::Write;
use crate::{parser::Parser, lexer::Lexer, code_generation::CodeGenerator, errors::error::Error, tokens::classification::Classification};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::style::Colorize;
use dark_vm::vm::VM;
use crate::{semantic_analyzer::type_checker::TypeChecker, std::standard_library::StandardLibrary};

pub struct EnvyRepl {
    lexer: Lexer,
    parser: Option<Parser>,
    code_gen: CodeGenerator,
    standard_library: StandardLibrary,
    vm: Option<VM>,
}

impl EnvyRepl {
    pub fn new() -> EnvyRepl {
        EnvyRepl {
            lexer: Lexer::default(),
            parser: None,
            code_gen: CodeGenerator::new(false, vec![]),
            standard_library: StandardLibrary::new(),
            vm: None,
        }
    }
}

impl Repl for EnvyRepl {
    fn evaluate_submission(&mut self, stdout: &mut std::io::Stdout, text: &String) -> crossterm::Result<()> {
        self.lexer.reset();
        let tokens = self.lexer
            .lex(text)
            .map_err(|error| prettify(stdout, error, text))?;
        
        println!("{:?}", &tokens);
        
        let parser = if let Some(ref mut p) = self.parser {
            p.with_tokens(tokens);
            p
        } else {
            self.parser = Some(Parser::new(tokens));
            self.parser.as_mut().unwrap()
        };

        let mut type_checker = TypeChecker::new();
        let ast = parser
            .parse(&self.standard_library, &mut type_checker)
            .map_err(|error| prettify(stdout, error, text))?;
        
        type_checker.perform_type_checking(&ast, &self.standard_library).map_err(|error| prettify(stdout, error, text))?;

        self.code_gen.user_defined_functions = type_checker.user_defined_functions.keys().cloned().collect::<Vec<_>>();
        let mut generated_code = String::new();
        for expression in ast.iter() {
            generated_code = format!("{}{}\n", generated_code, self.code_gen.compile_expression(expression, &self.standard_library, "").map_err(|error| prettify(stdout, error, text))?);
        }
        
        let tokens = dark_vm::lexer::Lexer::default()
            .lex(&generated_code)
            .map_err(|error| prettify_vm(stdout, error, text))?;

        let vm = if let Some(ref mut vm) = self.vm {
            vm.load_tokens(tokens).map_err(|error| prettify_vm(stdout, error, text))?;
            vm
        } else {
            let mut vm = VM::repl().map_err(|error| prettify_vm(stdout, error, text))?;
            vm.load_tokens(tokens).map_err(|error| prettify_vm(stdout, error, text))?;
            self.vm = Some(vm);
            self.vm.as_mut().unwrap()
        };

        disable_raw_mode()?;
        if let Some(result) = vm.run().map_err(|error| prettify_vm(stdout, error, text))? {
            write!(stdout, "{:#?}", result)?;
        }

        enable_raw_mode()
    }

    fn render_line(&mut self, stdout: &mut std::io::Stdout, lines: &[String], line_index: usize) -> crossterm::Result<()> {
        let text = lines.get(line_index).unwrap();
        let tokens = self
            .lexer
            .lex(&text)
            .ok();

        if let Some(tokens) = tokens {
            for token in tokens {
                match token.kind.get_classification() {
                    Classification::Whitespace(text) => {
                        write!(stdout, "{}", text)?;
                    }
                    Classification::Type(text) => {
                        write!(stdout, "{}", text.yellow())?;
                    }
                    Classification::Keyword(text) => {
                        write!(stdout, "{}", text.blue())?;
                    }
                    Classification::Values(text) => {
                        write!(stdout, "{}", text.cyan())?;
                    }
                    Classification::Punctuation(text) => {
                        write!(stdout, "{}", text.dark_grey())?;
                    }
                    Classification::Identifier(text) => {
                        write!(stdout, "{}", text.dark_yellow())?;
                    }
                }
            }
        } else {
            write!(stdout, "{}", lines.get(line_index).unwrap())?;
        }
        
        Ok(())
    }
}

fn prettify(stdout: &mut std::io::Stdout, error: Error, text: &str) -> std::io::Error {
    if let Err(error) = write!(stdout, "{}", error.prettify(text)) {
        error
    } else {
        std::io::Error::new(std::io::ErrorKind::Other, "Whoops!\nAn Unidentifiable Error Occurred.")
    }
}

fn prettify_vm(stdout: &mut std::io::Stdout, error: dark_vm::errors::error::Error, text: &str) -> std::io::Error {
    if let Err(error) = write!(stdout, "{}", error.prettify(text)) {
        error
    } else {
        std::io::Error::new(std::io::ErrorKind::Other, "Whoops!\nSome Error Occurred And Could Not Be Represented.")
    }
}