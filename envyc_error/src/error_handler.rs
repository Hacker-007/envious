use crate::error::Diagnostic;

pub trait ErrorHandler {
    fn handle_diagnostic(&mut self, diagnostic: Diagnostic);
}

#[derive(Debug, Default)]
pub struct StdoutErrorHandler {}

impl ErrorHandler for StdoutErrorHandler {
    fn handle_diagnostic(&mut self, diagnostic: Diagnostic) {
        println!("{:#?}", diagnostic);
    }
}
