use envyc_source::snippet::Snippet;

pub type DiagnosticMessage = String;

#[derive(Debug)]
pub enum Level {
    Error,
    Warning,
    Hint,
}

#[derive(Debug)]
pub struct Diagnostic {
    level: Level,
    messages: Vec<DiagnosticMessage>,
    snippet: Snippet,
    children: Vec<SubDiagnostic>,
}

impl Diagnostic {
    pub fn new<M: Into<DiagnosticMessage>>(
        level: Level,
        messages: Vec<M>,
        snippet: Snippet,
    ) -> Self {
        Self {
            level,
            messages: messages
                .into_iter()
                .map(|diagnostic_message| diagnostic_message.into())
                .collect(),
            snippet,
            children: vec![],
        }
    }
}

#[derive(Debug)]
pub struct SubDiagnostic {
    level: Level,
    message: Vec<DiagnosticMessage>,
    snippet: Snippet,
}
