use crate::error::Diagnostic;

pub trait ErrorHandler {
    fn handle_diagnostic(&mut self, diagnostic: Diagnostic);
}

pub struct InMemoryErrorHandler {
    diagnostics: Vec<Diagnostic>,
}

impl ErrorHandler for InMemoryErrorHandler {
    fn handle_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}
