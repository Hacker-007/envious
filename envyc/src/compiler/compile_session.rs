use crate::error::CompilerError;

pub(crate) struct CompileSession {
    has_errors: bool,
    errors: Vec<CompilerError>,
}

impl CompileSession {
    pub fn new() -> Self {
        Self {
            has_errors: false,
            errors: Vec::new(),
        }
    }

    pub fn emit_error(&mut self, error: CompilerError) {
        self.has_errors = true;
        self.errors.push(error);
    }
}
