use envyc_unit::span::Span;

pub type DiagnosticMessage = String;

pub enum Level {
    Error,
    Warning,
}

pub struct Diagnostic {
    level: Level,
    diagnostic_id: usize,
    message: Vec<DiagnosticMessage>,
    span: Span,
    children: Vec<SubDiagnostic>,
}

pub struct SubDiagnostic {
    level: Level,
    message: Vec<DiagnosticMessage>,
    span: Span,
}
