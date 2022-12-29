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
    footer: Vec<FooterMessage>,
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
            footer: vec![],
        }
    }

    pub fn add_footer<M: Into<DiagnosticMessage>>(mut self, level: Level, message: M) -> Self {
        self.footer.push(FooterMessage::new(level, message.into()));
        self
    }
}

#[derive(Debug)]
pub struct SubDiagnostic {
    level: Level,
    message: Vec<DiagnosticMessage>,
    snippet: Snippet,
}

#[derive(Debug)]
pub struct FooterMessage {
    level: Level,
    message: DiagnosticMessage,
}

impl FooterMessage {
    pub fn new(level: Level, message: DiagnosticMessage) -> Self {
        Self { level, message }
    }
}
