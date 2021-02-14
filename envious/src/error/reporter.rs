use std::collections::HashMap;

use codespan_reporting::{diagnostic::{Diagnostic, Label}, files::SimpleFiles, term::termcolor::{ColorChoice, StandardStream}};

use super::{Error, Span};

/// Struct that handles reporting the different errors that occur.
pub struct ErrorReporter<'a> {
    /// The files being reported.
    files: SimpleFiles<&'a str, String>,
    /// The ids of the files compiled.
    /// This is used by the codespan_reporting crate.
    /// This map goes from the name of the file to its id.
    file_ids: HashMap<String, usize>,
}

impl<'a> ErrorReporter<'a> {
    pub fn new(input_files: Vec<(&'a str, String)>) -> Self {
        let mut files = SimpleFiles::new();
        let mut file_ids = HashMap::new();
        for (name, source) in input_files {
            let id = files.add(name, source);
            file_ids.insert(name.to_string(), id);
        }

        Self {
            files,
            file_ids,
        }
    }

    /// Reports the error to the user. Note that this method does not consume the error.
    /// This allows errors to be reported in many different places.
    ///
    /// # Arguments
    /// `error` - The error to report.
    pub fn report(&self, error: &Error) {
        let diagnostic = match error {
            Error::IntegerOverflow(span) => self.handle_integer_overflow(span),
            Error::FloatOverflow(span) => self.handle_float_overflow(span),
            Error::UnterminatedString(span) => {
                self.handle_unterminated_string(span)
            }
            Error::UnrecognizedCharacter(span) => {
                self.handle_unrecognized_character(span)
            }
            Error::UnexpectedEndOfInput(span) => self.handle_end_of_input(span),
            error => todo!("{:#?}", error),
        };

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        codespan_reporting::term::emit(&mut writer.lock(), &config, &self.files, &diagnostic).unwrap();
    }

    /// Handles an integer overflow error.
    ///
    /// # Arguments
    /// `span` - The span of this error.
    fn handle_integer_overflow(
        &self,
        span: &Span,
    ) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("integer overflowed")
            .with_labels(vec![Label::primary(self.get_file_id(&span.file_name), start_column..end_column)])
            .with_notes(vec![format!(
                "integers must be >= {} and <= {}",
                i64::MIN,
                i64::MAX
            )])
    }

    /// Handles a float overflow error.
    ///
    /// # Arguments
    /// `span` - The span of this error.
    fn handle_float_overflow(
        &self,
        span: &Span,
    ) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("float overflow")
            .with_labels(vec![Label::primary(self.get_file_id(&span.file_name), start_column..end_column)])
            .with_notes(vec![format!(
                "floats must be >= {} and <= {}",
                f64::MIN,
                f64::MAX
            )])
    }

    /// Handles an unterminated string error.
    ///
    /// # Arguments
    /// `span` - The span of this error.
    fn handle_unterminated_string(
        &self,
        span: &Span,
    ) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        let string_start = self.get_file_source(&span.file_name).chars().nth(start_column).unwrap();
        Diagnostic::error()
            .with_message("unterminated string")
            .with_labels(vec![Label::primary(self.get_file_id(&span.file_name), start_column..end_column)])
            .with_notes(vec![format!(
                "try ending the string with a {}",
                string_start
            )])
    }

    /// Handles an unrecognized character error.
    ///
    /// # Arguments
    /// `span` - The span of this error.
    fn handle_unrecognized_character(
        &self,
        span: &Span,
    ) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("unrecognized character")
            .with_labels(vec![Label::primary(self.get_file_id(&span.file_name), start_column..end_column)])
    }

    /// Handles an unexpected end of input error.
    ///
    /// # Arguments
    /// `span` - The span of this error.
    fn handle_end_of_input(
        &self,
        span: &Span,
    ) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("expected an expression")
            .with_labels(vec![Label::primary(self.get_file_id(&span.file_name), start_column..end_column)])
    }

    /// Takes the span of the error and
    /// calculates the beginning column and the ending column
    /// with respect to the entire file.
    ///
    /// # Arguments
    /// `span` - The `Span` referenced by this error.
    fn construct_source(&self, span: &Span) -> (usize, usize) {
        let mut start_column = 0;
        let mut end_column = 0;
        let mut current_line = 1;
        let mut found_start = false;
        let bytes = self.get_file_source(&span.file_name).as_bytes();
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'\n' {
                current_line += 1;
            } else if current_line == span.line_start && !found_start {
                start_column += span.column_start;
                end_column += span.column_start;
                index += span.column_start;
                found_start = true;
                continue;
            } else if current_line == span.line_end {
                end_column += span.column_end;
                break;
            }

            if !found_start {
                start_column += 1;
            }

            end_column += 1;
            index += 1;
        }

        (start_column - 1, end_column)
    }

    /// Gets the source of the file associated with the given file.
    /// This function unwraps the result, thus it expects
    /// that the file actually exists.
    ///
    /// # Arguments
    /// `file_name` - The name of the file.
    fn get_file_source(&self, file_name: &str) -> &str {
        let file_id = self.get_file_id(file_name);
        self.files.get(file_id).unwrap().source()
    }

    /// Gets the file id associated with the given file.
    /// This function unwraps the result, thus it expects
    /// that the file actually exists.
    ///
    /// # Arguments
    /// `file_name` - The name of the file.
    #[inline]
    fn get_file_id(&self, file_name: &str) -> usize {
        *self.file_ids.get(file_name).unwrap()
    }
}

/// Trait to provide blanket implementations for containers of errors.
pub trait Reporter {
    /// Reports errors using a reference to the error reporter.
    /// Return whether or not an error was reported.
    ///
    /// # Arguments
    /// `error_reporter` - The `ErrorReporter` reference to use to report errors.
    fn report(&self, error_reporter: &ErrorReporter) -> bool;
}

impl Reporter for Vec<Error> {
    fn report(&self, error_reporter: &ErrorReporter) -> bool {
        for error in self {
            error_reporter.report(error);
        }

        self.len() != 0
    }
}

impl Reporter for Option<Error> {
    fn report(&self, error_reporter: &ErrorReporter) -> bool {
        if let Some(error) = self {
            error_reporter.report(error);
        }

        self.is_some()
    }
}

impl<T> Reporter for Result<T, Error> {
    fn report(&self, error_reporter: &ErrorReporter) -> bool {
        if let Err(error) = self {
            error_reporter.report(error);
        }

        self.is_err()
    }
}