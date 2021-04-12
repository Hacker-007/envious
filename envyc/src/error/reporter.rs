use std::collections::HashMap;

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::termcolor::{ColorChoice, StandardStream},
};

use crate::{lexer::token::TokenKind, semantic_analyzer::types::Type};

use super::{Error, Span};

/// Struct that handles reporting the different errors that occur.
pub struct ErrorReporter<'a> {
    /// The files being reported.
    files: SimpleFiles<&'a str, &'a str>,
    /// The ids of the files compiled.
    /// This is used by the codespan_reporting crate.
    /// This map goes from the name of the file to its id.
    file_ids: HashMap<&'a str, usize>,
}

impl<'a> ErrorReporter<'a> {
    pub fn new(input_files: Vec<(&'a str, &'a str)>) -> Self {
        let mut files = SimpleFiles::new();
        let mut file_ids = HashMap::new();
        for (name, source) in input_files {
            let id = files.add(name, source);
            file_ids.insert(name, id);
        }

        Self { files, file_ids }
    }

    /// Adds a file to the input files.
    ///
    /// # Arguments
    /// * `file_name` - The name of the file.
    /// * `source` - The source of the input.
    pub fn add(&mut self, file_name: &'a str, source: &'a str) {
        let id = self.files.add(file_name, source);
        self.file_ids.insert(file_name, id);
    }

    /// Reports the error to the user. Note that this method does not consume the error.
    /// This allows errors to be reported in many different places.
    ///
    /// # Arguments
    /// * `error` - The error to report.
    pub fn report(&self, error: &Error) {
        let diagnostic = match error {
            Error::IntegerOverflow(span) => self.handle_integer_overflow(*span),
            Error::FloatOverflow(span) => self.handle_float_overflow(*span),
            Error::UnterminatedChar(span) => self.handle_unterminated_char(*span),
            Error::UnrecognizedCharacter(span) => self.handle_unrecognized_character(*span),
            Error::UnexpectedEndOfInput(span) => self.handle_end_of_input(*span),
            Error::ExpectedPrefixExpression {
                span,
                found_kind: kind,
            } => self.handle_expected_prefix_expression(*span, *kind),
            Error::ExpectedKind {
                span,
                expected_kinds,
                actual_kind,
            } => self.handle_expected_kind(*span, expected_kinds, *actual_kind),
            Error::UnsupportedOperation {
                operation_span,
                operands,
            } => self.handle_unsupported_operation(*operation_span, operands),
            Error::TypeMismatch {
                span,
                expected_type,
                actual_type,
            } => self.handle_type_mismatch(*span, *expected_type, *actual_type),
            Error::ConflictingType {
                first_span,
                first_type,
                second_span,
                second_type,
            } => self.handle_conflicting_type(*first_span, *first_type, *second_span, *second_type),
            Error::IllegalType(span) => self.handle_illegal_type(*span),
            Error::UndefinedVariable(span) => self.handle_undefined_variable(*span),
            Error::ParameterMismatch {
                span,
                expected_parameter_count,
                actual_parameter_count,
            } => self.handle_parameter_mismatch(
                *span,
                *expected_parameter_count,
                *actual_parameter_count,
            ),
            Error::UnknownFunction(span) => self.handle_unknown_function(*span),
            Error::ExpectedFunction => {
                println!("Expected a function to be selected when compiling to LLVM.");
                return;
            }
            Error::LLVMFunctionFailure => {
                println!("An unexpected error occurred when compiling a function to LLVM.");
                return;
            }
        };

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        codespan_reporting::term::emit(&mut writer.lock(), &config, &self.files, &diagnostic)
            .unwrap();
    }

    /// Handles an integer overflow error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    fn handle_integer_overflow(&self, span: Span) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("integer overflowed")
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )])
            .with_notes(vec![format!(
                "integers must be >= {} and <= {}",
                i64::MIN,
                i64::MAX
            )])
    }

    /// Handles a float overflow error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    fn handle_float_overflow(&self, span: Span) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("float overflow")
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )])
            .with_notes(vec![format!(
                "floats must be >= {} and <= {}",
                f64::MIN,
                f64::MAX
            )])
    }

    /// Handles an unterminated char error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    fn handle_unterminated_char(&self, span: Span) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("unterminated char")
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )])
            .with_notes(vec!["try ending the char with a \'".to_string()])
    }

    /// Handles an unrecognized character error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    fn handle_unrecognized_character(&self, span: Span) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("unrecognized character")
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )])
    }

    /// Handles an unexpected end of input error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    fn handle_end_of_input(&self, span: Span) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("expected an expression")
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )])
    }

    /// Handles an expected prefix expression error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    /// * `kind` - The `TokenKind` found.
    fn handle_expected_prefix_expression(&self, span: Span, kind: TokenKind) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("expected prefix expression")
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )
            .with_message(format!(
                "the token `{}` does not correspond to any prefix expression",
                kind
            ))])
    }

    /// Handles an expected kind error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    /// * `expected_kinds` - The `TokenKind`'s expected.
    /// * `actual_kind` - The `TokenKind` found.
    fn handle_expected_kind(
        &self,
        span: Span,
        expected_kinds: &[TokenKind],
        actual_kind: TokenKind,
    ) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        let expected_kinds = expected_kinds
            .iter()
            .map(|kind| format!("{}", kind))
            .collect::<Vec<_>>()
            .join(", or ");
        Diagnostic::error()
            .with_message(format!("expected {}", expected_kinds))
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )
            .with_message(format!("but found {}", actual_kind))])
    }

    /// Handles an unsupported operation error.
    ///
    /// # Arguments
    /// * `operation_span` - The `Span` of this error.
    /// * `operands` - The `Span` and the `Type` of the operands.
    fn handle_unsupported_operation(
        &self,
        operation_span: Span,
        operands: &[(Span, Type)],
    ) -> Diagnostic<usize> {
        let (operation_start, operation_end) = self.construct_source(operation_span);
        let mut labels = vec![Label::primary(
            self.get_file_id(&operation_span.file_name),
            operation_start..operation_end,
        )];
        for operand in operands {
            let (operand_start, operand_end) = self.construct_source(operand.0);
            labels.push(
                Label::secondary(
                    self.get_file_id(&operand.0.file_name),
                    operand_start..operand_end,
                )
                .with_message(format!("has a type of {}", operand.1)),
            )
        }

        Diagnostic::error()
            .with_message("unsupported operation")
            .with_labels(labels)
    }

    /// Handles a type mismatch error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    /// * `expected_type` - The `Type` expected.
    /// * `actual_type` - The `Type` found.
    fn handle_type_mismatch(
        &self,
        span: Span,
        expected_type: Type,
        actual_type: Type,
    ) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("type mismatch")
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )
            .with_message(format!(
                "expected `{}` but found `{}`",
                expected_type, actual_type
            ))])
    }

    /// Handles a conflicting type error.
    ///
    /// # Arguments
    /// * `first_span` - The `Span` of the first branch.
    /// * `first_type` - The `Type` of the first branch.
    /// * `second_span` - The `Span` of th second branch.
    /// * `second_type` - The `Type` of the second branch.
    fn handle_conflicting_type(
        &self,
        first_span: Span,
        first_type: Type,
        second_span: Span,
        second_type: Type,
    ) -> Diagnostic<usize> {
        let (first_start_column, first_end_column) = self.construct_source(first_span);
        let (second_start_column, second_end_column) = self.construct_source(second_span);
        Diagnostic::error()
            .with_message("type conflict occurred")
            .with_labels(vec![
                Label::primary(
                    self.get_file_id(&first_span.file_name),
                    first_start_column..first_end_column,
                )
                .with_message(format!("results in `{}`", first_type)),
                Label::primary(
                    self.get_file_id(&second_span.file_name),
                    second_start_column..second_end_column,
                )
                .with_message(format!("results in `{}`", second_type)),
            ])
    }

    /// Handles an illegal type error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    fn handle_illegal_type(&self, span: Span) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("placed a type where it was not allowed")
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )])
    }

    /// Handles an undefined variable error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    fn handle_undefined_variable(&self, span: Span) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("found undefined variable")
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )])
    }

    /// Handles a parameter mismatch error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    /// * `expected_parameter_count` - The number of parameters expected by the function.
    /// * `actual_parameter_count` - The number of parameters that were provided.
    fn handle_parameter_mismatch(
        &self,
        span: Span,
        expected_parameter_count: usize,
        actual_parameter_count: usize,
    ) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message(&format!(
                "this function expected {} parameters but received {} parameters",
                expected_parameter_count, actual_parameter_count
            ))
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )])
    }

    /// Handles an unknown function error.
    ///
    /// # Arguments
    /// * `span` - The `Span` of this error.
    fn handle_unknown_function(&self, span: Span) -> Diagnostic<usize> {
        let (start_column, end_column) = self.construct_source(span);
        Diagnostic::error()
            .with_message("found unknown function")
            .with_labels(vec![Label::primary(
                self.get_file_id(&span.file_name),
                start_column..end_column,
            )])
    }

    /// Takes the span of the error and
    /// calculates the beginning column and the ending column
    /// with respect to the entire file.
    ///
    /// # Arguments
    /// * `span` - The `Span` referenced by this error.
    fn construct_source(&self, span: Span) -> (usize, usize) {
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
                if current_line != span.line_start {
                    end_column += span.column_end;
                } else if span.line_start == span.line_end {
                    end_column += span.column_end - span.column_start;
                }

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
    /// * `file_name` - The name of the file.
    fn get_file_source(&self, file_name: &str) -> &str {
        let file_id = self.get_file_id(file_name);
        self.files.get(file_id).unwrap().source()
    }

    /// Gets the file id associated with the given file.
    /// This function unwraps the result, thus it expects
    /// that the file actually exists.
    ///
    /// # Arguments
    /// * `file_name` - The name of the file.
    #[inline]
    fn get_file_id(&self, file_name: &str) -> usize {
        *self.file_ids.get(file_name).unwrap()
    }
}

/// Trait to provide blanket implementations for containers of errors.
pub trait Reporter {
    /// The value returned by the reporter once used.
    type Output;

    /// Determines whether this container is in an error state.
    fn is_err(&self) -> bool;

    /// Reports errors using a reference to the error reporter.
    /// Return the output if there were no errors.
    ///
    /// # Arguments
    /// * `error_reporter` - The `ErrorReporter` reference to use to report errors.
    fn report(self, error_reporter: &ErrorReporter) -> Option<Self::Output>;
}

impl<'a> Reporter for Vec<Error<'a>> {
    type Output = ();

    fn is_err(&self) -> bool {
        !self.is_empty()
    }

    fn report(self, error_reporter: &ErrorReporter) -> Option<Self::Output> {
        for error in &self {
            error_reporter.report(error);
        }

        if !self.is_empty() {
            None
        } else {
            Some(())
        }
    }
}

impl<'a> Reporter for Option<Error<'a>> {
    type Output = ();

    fn is_err(&self) -> bool {
        self.is_some()
    }

    fn report(self, error_reporter: &ErrorReporter) -> Option<Self::Output> {
        if let Some(ref error) = self {
            error_reporter.report(error);
        }

        if self.is_some() {
            None
        } else {
            Some(())
        }
    }
}

impl<'a, T> Reporter for Result<T, Error<'a>> {
    type Output = T;

    fn is_err(&self) -> bool {
        self.is_err()
    }

    fn report(self, error_reporter: &ErrorReporter) -> Option<Self::Output> {
        match self {
            Ok(val) => Some(val),
            Err(error) => {
                error_reporter.report(&error);
                None
            }
        }
    }
}

impl<'a, T> Reporter for Result<T, Vec<Error<'a>>> {
    type Output = T;

    fn is_err(&self) -> bool {
        matches!(self, Err(errors) if !errors.is_empty())
    }

    fn report(self, error_reporter: &ErrorReporter) -> Option<Self::Output> {
        match self {
            Ok(val) => Some(val),
            Err(errors) => {
                errors.report(error_reporter)?;
                None
            }
        }
    }
}
