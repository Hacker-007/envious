use super::repl_trait::Repl;
use std::io::Write;
use crate::{parser::Parser, lexer::Lexer, code_generation::CodeGenerator};
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
        match self.lexer.lex(text) {
            Err(error) => {
                write!(stdout, "{}", error.prettify(text))?;
                Ok(())
            }
            Ok(tokens) => {
                if self.parser.is_none() {
                    self.parser = Some(Parser::new(tokens));
                } else {
                    self.parser.as_mut().unwrap().with_tokens(tokens);
                }

                match self.parser.as_mut().unwrap().parse() {
                    Err(error) => {
                        write!(stdout, "{}", error.prettify(text))?;
                        Ok(())
                    }
                    Ok(syntax_tree) => {
                        match self.code_gen.generate_code(None, syntax_tree) {
                            Err(error) => {
                                write!(stdout, "{}", error.prettify(text))?;
                                Ok(())
                            }
                            Ok(generated_code) => {
                                disable_raw_mode()?;
                                match dark_vm::lexer::Lexer::default().lex(&generated_code) {
                                    Ok(tokens) => {
                                        if self.vm.is_none() {
                                            match VM::new(tokens) {
                                                Ok(vm) => self.vm = Some(vm),
                                                Err(error) => {
                                                    write!(stdout, "{}", error.prettify(&generated_code))?;
                                                }
                                            }
                                        } else {
                                            let vm = self.vm.as_mut().unwrap();
                                            if let Err(error) = vm.with_tokens(tokens) {
                                                write!(stdout, "{}", error.prettify(&generated_code))?;
                                            }
                                        }

                                        match self.vm.as_mut().unwrap().run() {
                                            Ok(res) => {
                                                if let Some(res) = res {
                                                    write!(stdout, "{:#?}", res)?;
                                                }
                                            }
                                            Err(error) => {
                                                write!(stdout, "{}", error.prettify(&generated_code))?;
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        write!(stdout, "{}", error.prettify(&generated_code))?;
                                    }
                                }

                                enable_raw_mode()
                            }
                        }
                    }
                }
            }
        }
    }

    fn render_line(&mut self, stdout: &mut std::io::Stdout, lines: &[String], line_index: usize) -> crossterm::Result<()> {
        write!(stdout, "{}", lines.get(line_index).unwrap())?;
        Ok(())
    }
}