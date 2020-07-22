//! The ErrorKind enum maintains the different errors that can occur during the execution of the program.
//! This allows for uniformity across the various errors because the error messages are the same.
//! This also increases readibility within the code, because the ErrorKind's are more descriptive.

pub enum ErrorKind {
    SystemError(String),
    UnrecognizedArgument(String),

    UnknownCharacter,
    InvalidNumberFormat,
    UnterminatedString,

    TypeMismatch(String, String),
    Expected(String),
    UnknownFunction,
    WrongNumberOfParameters,
}

/// Converts the ErrorKind into a String.
/// This is used in the prettify method to produce the error messages needed.
impl Into<String> for ErrorKind {
    fn into(self) -> String {
        match self {
            ErrorKind::SystemError(error) => return error,
            ErrorKind::UnrecognizedArgument(arg) => {
                return format!("The Argument '{}' Is Not A Valid Argument.", arg)
            }

            ErrorKind::UnknownCharacter => "Unknown Character Found Here.",
            ErrorKind::InvalidNumberFormat => "Invalid Number Format.",
            ErrorKind::UnterminatedString => "Expected The End Of This String.",

            ErrorKind::TypeMismatch(expected, actual) => {
                return format!("Expected {}, But Found {}.", expected, actual)
            }
            ErrorKind::Expected(expected) => return format!("Expected {} After Here.", expected),
            ErrorKind::UnknownFunction => "Unknown Function Called Here.",
            ErrorKind::WrongNumberOfParameters => {
                "The Wrong Number Of Parameters Were Supplied To This Function."
            }
        }
        .to_owned()
    }
}
