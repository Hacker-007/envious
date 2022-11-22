use envyc_source::snippet::Snippet;

pub type DiagnosticMessage = String;

#[derive(Debug)]
pub enum Level {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct Diagnostic {
    level: Level,
    diagnostic_id: usize,
    message: Vec<DiagnosticMessage>,
    snippet: Snippet,
    children: Vec<SubDiagnostic>,
}

#[derive(Debug)]
pub struct SubDiagnostic {
    level: Level,
    message: Vec<DiagnosticMessage>,
    snippet: Snippet,
}
