use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};

/// Enum used by compiler to construct the various errors.
/// Every error needs to keep a track of the span of the error
/// to provide a better representation when reported to the user,
/// unless the error stems from the LLVM compiler, which is not
/// derived from the user's code.
#[derive(Debug)]
pub enum Error {
    // Occurs when an integer that exceeeds the maximum possible value of an integer.
    IntegerOverflow(Span),
    // Occurs when a float that exceeeds the maximum possible value of a float.
    FloatOverflow(Span),
    // Occurs when a string that has been started but not closed.
    UnterminatedString(Span),
    // Occurs when a character that is not recognized by the `Lexer`.
    UnrecognizedCharacter(Span),

    // Occurs when an expression was expected by the `Parser` but
    // there were no more tokens to inspect.
    UnexpectedEndOfInput(Span),
    // Occurs when a token does not have a corresponding expression.
    ExpectedPrefixExpression {
        span: Span,
        found_kind: TokenKind,
    },
    // Occurs when a certain token was expected by an expression but
    // a different token was found.
    ExpectedKind {
        span: Span,
        expected_kinds: Vec<TokenKind>,
        actual_kind: TokenKind,
    },

    // Occurs when the specified operation could not be applied to operands.
    UnsupportedOperation {
        operation_span: Span,
        operands: Vec<(Span, Type)>,
    },
    // Occurs when the type of an expression does not match the expected type.
    TypeMismatch {
        span: Span,
        expected_type: Type,
        actual_type: Type,
    },
    // Occurs when the type of two branches do not match. For example, if the type
    // of the then branch and the type of the else branch do not match, this error
    // is returned.
    ConflictingType {
        first_span: Span,
        first_type: Type,
        second_span: Span,
        second_type: Type,
    },
    // Occurs when a cast could not be performed between the original type and the new
    // type.
    IllegalCast {
        span: Span,
        from_type: Type,
        to_type: Type,
    },

    /// Occurs when a function was expected during the LLVM compilation.
    ExpectedFunction,
}

impl Error {
    /// Reports the error to the user. Note that this method does not consume the error.
    /// This allows errors to be reported in many different places.
    ///
    /// # Arguments
    /// `input` - The source code or the input given to the compiler.
    pub fn report_error(&self, input: &[u8]) {
        match self {
            Error::IntegerOverflow(span) => self.handle_integer_overflow(input, span),
            Error::FloatOverflow(span) => self.handle_float_overflow(input, span),
            Error::UnterminatedString(span) => self.handle_unterminated_string(input, span),
            Error::UnrecognizedCharacter(span) => self.handle_unrecognized_character(input, span),
            Error::UnexpectedEndOfInput(span) => self.handle_end_of_input(input, span),
            error => todo!("{:#?}", error),
        }
    }

    /// Handles an integer overflow error.
    ///
    /// # Arguments
    /// `input` - The source code or the input given to the compiler.
    /// `span` - The span of this error.
    fn handle_integer_overflow(&self, input: &[u8], span: &Span) {
        let mut snippet = self.generate_empty_snippet();
        snippet.title = Some(Annotation {
            id: None,
            label: Some("integer overflowed"),
            annotation_type: AnnotationType::Error,
        });

        let (source, start_column, end_column) = Error::construct_source(input, &span);
        snippet.slices.push(Slice {
            source: &source,
            line_start: span.line_start,
            origin: Some(&span.file_name),
            annotations: vec![SourceAnnotation {
                label: "",
                annotation_type: AnnotationType::Error,
                range: (start_column, end_column),
            }],
            fold: true,
        });

        let hint = format!("integers must be >= {} and <= {}", i64::MIN, i64::MAX);
        snippet.footer.push(Annotation {
            id: None,
            label: Some(hint.as_str()),
            annotation_type: AnnotationType::Note,
        });

        let display_list = DisplayList::from(snippet);
        println!("{}", display_list);
    }

    /// Handles a float overflow error.
    ///
    /// # Arguments
    /// `input` - The source code or the input given to the compiler.
    /// `span` - The span of this error.
    fn handle_float_overflow(&self, input: &[u8], span: &Span) {
        let mut snippet = self.generate_empty_snippet();
        snippet.title = Some(Annotation {
            id: None,
            label: Some("float overflowed"),
            annotation_type: AnnotationType::Error,
        });

        let (source, start_column, end_column) = Error::construct_source(input, span);
        snippet.slices.push(Slice {
            source: &source,
            line_start: span.line_start,
            origin: Some(&span.file_name),
            annotations: vec![SourceAnnotation {
                label: "",
                annotation_type: AnnotationType::Error,
                range: (start_column, end_column),
            }],
            fold: true,
        });

        let hint = format!("floats must be >= {} and <= {}", f64::MIN, f64::MAX);
        snippet.footer.push(Annotation {
            id: None,
            label: Some(hint.as_str()),
            annotation_type: AnnotationType::Note,
        });

        let display_list = DisplayList::from(snippet);
        println!("{}", display_list);
    }

    /// Handles an unterminated string error.
    ///
    /// # Arguments
    /// `input` - The source code or the input given to the compiler.
    /// `span` - The span of this error.
    fn handle_unterminated_string(&self, input: &[u8], span: &Span) {
        let mut snippet = self.generate_empty_snippet();
        snippet.title = Some(Annotation {
            id: None,
            label: Some("unterminated string"),
            annotation_type: AnnotationType::Error,
        });

        let (source, start_column, end_column) = Error::construct_source(input, span);
        snippet.slices.push(Slice {
            source: &source,
            line_start: span.line_start,
            origin: Some(&span.file_name),
            annotations: vec![SourceAnnotation {
                label: "",
                annotation_type: AnnotationType::Error,
                range: (start_column, end_column),
            }],
            fold: true,
        });

        let help = format!(
            "try ending the string with a {}",
            source.chars().next().unwrap()
        );
        snippet.footer.push(Annotation {
            id: None,
            label: Some(&help),
            annotation_type: AnnotationType::Help,
        });

        let display_list = DisplayList::from(snippet);
        println!("{}", display_list);
    }

    /// Handles an unrecognized character error.
    ///
    /// # Arguments
    /// `input` - The source code or the input given to the compiler.
    /// `span` - The span of this error.
    fn handle_unrecognized_character(&self, input: &[u8], span: &Span) {
        let mut snippet = self.generate_empty_snippet();
        snippet.title = Some(Annotation {
            id: None,
            label: Some("unrecognized character"),
            annotation_type: AnnotationType::Error,
        });

        let (source, start_column, end_column) = Error::construct_source(input, span);
        snippet.slices.push(Slice {
            source: &source,
            line_start: span.line_start,
            origin: Some(&span.file_name),
            annotations: vec![SourceAnnotation {
                label: "",
                annotation_type: AnnotationType::Error,
                range: (start_column, end_column),
            }],
            fold: true,
        });

        let display_list = DisplayList::from(snippet);
        println!("{}", display_list);
    }

    /// Handles an unexpected end of input error.
    ///
    /// # Arguments
    /// `input` - The source code or the input given to the compiler.
    fn handle_end_of_input(&self, input: &[u8], span: &Span) {
        let mut snippet = self.generate_empty_snippet();
        snippet.title = Some(Annotation {
            id: None,
            label: Some("expected an expression"),
            annotation_type: AnnotationType::Error,
        });

        let (source, start_column, end_column) = Error::construct_source(input, span);
        snippet.slices.push(Slice {
            source: &source,
            line_start: span.line_start,
            origin: Some(&span.file_name),
            annotations: vec![SourceAnnotation {
                label: "",
                annotation_type: AnnotationType::Error,
                range: (start_column, end_column),
            }],
            fold: true,
        });

        let display_list = DisplayList::from(snippet);
        println!("{}", display_list);
    }

    /// Generates a blank snippet to manipulate from other methods.
    fn generate_empty_snippet(&self) -> Snippet {
        Snippet {
            title: None,
            footer: vec![],
            slices: vec![],
            opt: FormatOptions {
                color: true,
                ..Default::default()
            },
        }
    }

    /// Takes the input and the span of the error and
    /// constructs a string representing the section of code
    /// referenced by the error and the beginning column and the ending column.
    ///
    /// # Arguments
    /// `input` - The source code or the input given to the compiler.
    /// `span` - The `Span` referenced by this error from this input.
    fn construct_source(input: &[u8], span: &Span) -> (String, usize, usize) {
        let span_bytes = input
            .split(|byte| *byte == b'\n')
            .skip(span.line_start - 1)
            .take(span.line_end - span.line_start + 1)
            .collect::<Vec<_>>();

        let mut constructed_string = String::new();
        if span_bytes.len() == 1 {
            span_bytes[0]
                .iter()
                .take(span.column_end)
                .map(|byte| *byte as char)
                .for_each(|char| {
                    constructed_string.push(char);
                });

            let end_column = constructed_string.len();
            return (constructed_string, span.column_start - 1, end_column);
        }

        let mut offset = 0;
        for line in span_bytes.iter().take(span.line_end) {
            line.iter().map(|byte| *byte as char).for_each(|char| {
                constructed_string.push(char);
                offset += 1;
            });

            constructed_string.push('\n');
            offset += 1;
        }

        let mut end_column = offset;
        let line = span_bytes[span.line_end - 1];
        if !line.is_empty() {
            line[0..(span.column_end - 1)]
                .iter()
                .map(|byte| *byte as char)
                .for_each(|char| constructed_string.push(char));
            end_column += span.column_end - 1;
        } else {
            end_column -= 1;
        }

        (constructed_string, span.column_start - 1, end_column)
    }
}

pub mod span;
pub use span::Span;

use crate::{lexer::token::TokenKind, semantic_analyzer::types::Type};
