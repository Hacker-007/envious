use super::repl_trait::Repl;
use std::io::Write;
use crate::{parser::Parser, lexer::Lexer, code_generation::CodeGenerator, errors::error::Error};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use dark_vm::vm::VM;

pub struct EnvyRepl {
    lexer: Lexer,
    parser: Option<Parser>,
    code_gen: CodeGenerator,
    vm: Option<VM>,
}

impl EnvyRepl {
    pub fn new() -> EnvyRepl {
        EnvyRepl {
            lexer: Lexer::default(),
            parser: None,
            code_gen: CodeGenerator::new(false),
            vm: None,
        }
    }
}

impl Repl for EnvyRepl {
    fn evaluate_submission(&mut self, stdout: &mut std::io::Stdout, text: &String) -> crossterm::Result<()> {
        let tokens = self.lexer
            .lex(text)
            .map_err(|error| prettify(stdout, error, text))?;
        
        let parser = if let Some(ref mut p) = self.parser {
            p.with_tokens(tokens);
            p
        } else {
            self.parser = Some(Parser::new(tokens));
            self.parser.as_mut().unwrap()
        };

        let ast = parser
            .parse()
            .map_err(|error| prettify(stdout, error, text))?;
        
        let mut generated_code = String::new();
        for expression in ast.iter() {
            generated_code = format!("{}{}\n", generated_code, self.code_gen.compile_expression(expression, "").map_err(|error| prettify(stdout, error, text))?);
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
        write!(stdout, "{}", lines.get(line_index).unwrap())?;
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