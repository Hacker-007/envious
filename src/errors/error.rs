//! The Error struct maintains the errors that occur during execution.

use super::error_kind::ErrorKind;

pub struct Error {
    kind: ErrorKind,
    position: Option<usize>,
}

impl Error {
    /// Constructs a new error with the error kind and the position.
    ///
    /// # Arguments
    /// `kind` - The value of the error. Maintaining the value allows for the messages to be controlled across execution.
    /// `position` - The position where the error occurred.
    pub fn new(kind: ErrorKind, position: usize) -> Error {
        Error {
            kind,
            position: Some(position),
        }
    }

    /// Constructs a new error with the error kind and no position.
    ///
    /// # Arguments
    /// `kind` - The value of the error. Maintaining the value allows for the messages to be controlled across execution.
    pub fn message_only(kind: ErrorKind) -> Error {
        Error {
            kind,
            position: None,
        }
    }

    /// This function generates a pretty version of the error, with arrows pointing to the exact location of the error.
    /// This function also consumes the error, therefore, it should be the last thing called.
    ///
    /// # Arguments
    /// `input` - The input for the program. This is not maintained with every error because the input might be different.
    pub fn prettify(self, input: &str) -> String {
        if self.position.is_some() {
            // Get the line and column number of where the error occurred.
            let (line_number, column_number) = self.get_line_column_info(input);

            // Check if a line is present. If not, the error is printed without the arrows.
            // This should usually produce a line, but it may not.
            let option_line = input.split_terminator('\n').nth(line_number - 1);

            // Convert the kind into an error message.
            let error_message: String = self.kind.into();
            if let Some(line) = option_line {
                let len = line_number.to_string().len();
                format!(
                    "{} |\n{} | {}\n{} | {}^-- {}\n",
                    " ".repeat(len),
                    line_number,
                    line,
                    " ".repeat(len),
                    " ".repeat(column_number - 1),
                    error_message,
                )
            } else {
                format!(
                    "An Error Occurred On Line {} And Column {}.\n{}",
                    line_number, column_number, error_message,
                )
            }
        } else {
            // Convert the kind into an error message.
            let error_message: String = self.kind.into();
            format!("An Error Occurred.\n{}", error_message)
        }
    }

    /// This function gets the line and column number of where the error occurred with respect to the input.
    fn get_line_column_info(&self, input: &str) -> (usize, usize) {
        let (mut line_number, mut column_number) = (1, 0);

        // Go through the characters and find the index that matches the position given in the error struct.
        input.chars().enumerate().find(|(idx, ch)| {
            if ch == &'\n' {
                line_number += 1;
                column_number = 0;
            } else {
                column_number += 1;
            }

            idx == &(self.position.unwrap() - 1)
        });

        (line_number, column_number)
    }
}
