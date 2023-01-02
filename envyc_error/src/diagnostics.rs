use envyc_source::snippet::Snippet;

pub type DiagnosticMessage = String;

#[derive(Debug)]
pub enum Level {
    Error,
    Warning,
    Hint,
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

#[derive(Debug)]
pub struct Diagnostic {
    level: Level,
    title: DiagnosticMessage,
    snippet: Snippet,
    children: Vec<SubDiagnostic>,
    footer: Vec<FooterMessage>,
}

impl Diagnostic {
    pub fn new<M: Into<DiagnosticMessage>>(level: Level, title: M, snippet: Snippet) -> Self {
        Self {
            level,
            title: title.into(),
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

impl Into<annotate_snippets::snippet::Snippet<'static>> for Diagnostic {

    fn into(self) -> annotate_snippets::snippet::Snippet<'static> {
        todo!()
    }
}