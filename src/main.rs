pub mod context;
pub mod source;
pub mod error;

pub mod lexical_analysis;

use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use context::{CompilationContext, DiagnosticStats};
use lexical_analysis::token_stream::TokenStream;

use crate::lexical_analysis::parser::Parser;

fn main() {
    let mut ctx = CompilationContext::new(Box::new(StandardStream::stderr(ColorChoice::Auto)));
    let source_id = ctx.add_source("test.envy", "hello not + - 13");
    let token_stream = TokenStream::new(&ctx, ctx.get_source(source_id).unwrap());
    let mut parser = Parser::from_stream(token_stream);
    let program = parser.parse();
    println!("{:#?}", program);
    
    let DiagnosticStats {
        error_count,
        warning_count,
    } = ctx.get_diagnostic_stats();
    println!(
        "\nfinished with {} {} and {} {}",
        error_count,
        pluralize("error", "errors", error_count),
        warning_count,
        pluralize("warning", "warnings", warning_count)
    );
}

fn pluralize<'a>(base_form: &'a str, plural_form: &'a str, count: usize) -> &'a str {
    if count == 1 {
        base_form
    } else {
        plural_form
    }
}
