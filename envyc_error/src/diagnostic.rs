use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet as AnnotateSnippet, SourceAnnotation},
};

use envyc_context::{context::CompilationContext, diagnostic_handler::DiagnosticHandler};
use envyc_source::snippet::Snippet;

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Error,
    Warning,
    Help,
}

impl Into<AnnotationType> for Level {
    fn into(self) -> AnnotationType {
        match self {
            Level::Error => AnnotationType::Error,
            Level::Warning => AnnotationType::Warning,
            Level::Help => AnnotationType::Help,
        }
    }
}

#[derive(Debug)]
pub struct Message<'a> {
    snippet: Option<Snippet>,
    level: Level,
    message: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(level: Level, message: &'a str, snippet: Option<Snippet>) -> Self {
        Self {
            level,
            message,
            snippet,
        }
    }
}

#[derive(Debug)]
pub struct SubDiagnostic<'a> {
    messages: Vec<Message<'a>>,
    snippet: Snippet,
}

impl<'a> SubDiagnostic<'a> {
    pub fn new(messages: Vec<Message<'a>>, snippet: Snippet) -> Self {
        Self { messages, snippet }
    }
}

#[derive(Debug)]
pub struct Diagnostic<'a> {
    title: Message<'a>,
    children: Vec<SubDiagnostic<'a>>,
    footer: Vec<Message<'a>>,
}

impl<'a> Diagnostic<'a> {
    pub fn new(level: Level, title: &'a str) -> Self {
        Self {
            title: Message::new(level, title, None),
            children: vec![],
            footer: vec![],
        }
    }

    pub fn add_child(&mut self, child: SubDiagnostic<'a>) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn add_footer(&mut self, footer: Message<'a>) -> &mut Self {
        self.footer.push(footer);
        self
    }

    pub fn emit<'ctx, D: DiagnosticHandler>(&self, compilation_ctx: &'ctx CompilationContext<D>) {
        let title = Some(Annotation {
            id: None,
            label: Some(self.title.message),
            annotation_type: self.title.level.into(),
        });

        let footer = self
            .footer
            .iter()
            .map(|footer_message| Annotation {
                id: None,
                label: Some(footer_message.message),
                annotation_type: footer_message.level.into(),
            })
            .collect();

        let slices = self
            .children
            .iter()
            .map(|sub_diagnostic| {
                let main_snippet = sub_diagnostic.snippet;
                let source = compilation_ctx.get_source(main_snippet.source_id);
                let extended_start = source.extend_back(main_snippet.start);
                let extended_end = source.extend(main_snippet.end);
                let source_text = source.get_text(extended_start, extended_end);
                let annotations = sub_diagnostic
                    .messages
                    .iter()
                    .filter(|message| message.snippet.is_some())
                    .map(|message| {
                        let message_snippet = message.snippet.unwrap();
                        SourceAnnotation {
                            range: (
                                (message_snippet.start - extended_start).0,
                                (message_snippet.end - extended_start).0,
                            ),
                            label: message.message,
                            annotation_type: message.level.into(),
                        }
                    })
                    .collect::<Vec<_>>();

                Slice {
                    source: source_text,
                    line_start: source.get_line_information(extended_start).line_number,
                    origin: Some(&source.name),
                    annotations,
                    fold: true,
                }
            })
            .collect();

        let options = FormatOptions {
            color: true,
            ..Default::default()
        };

        let snippet = AnnotateSnippet {
            title,
            footer,
            slices,
            opt: options,
        };

        let display_list = DisplayList::from(snippet);
        compilation_ctx
            .diagnostic_handler
            .handle(format!("{}", display_list));
    }
}
