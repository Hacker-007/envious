#![allow(unused)]

use crate::{
    context::CompilationContext,
    lex::{token::buffer::TokenizedBuffer, Lexer},
    source::SourceId,
};

pub mod ast;
pub mod compiler;
pub mod diagnostics;
pub mod source;

pub(crate) mod context;
pub(crate) mod dense;
pub(crate) mod lex;
pub(crate) mod parser;

#[cfg(test)]
mod test {
    use crate::{ast::printer::PrettyPrinter, compiler::Compiler};

    #[test]
    fn print_ok() {
        let mut compiler = Compiler::default();
        let id = compiler.lex("1+2");
        compiler.parse(&id);
        let printer = PrettyPrinter::new(
            compiler.source(&id),
            compiler.tokens(&id),
            compiler.ast(&id),
        );

        assert_eq!(printer.to_string(), "(1 + 2)");
    }
}
