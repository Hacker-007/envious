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
    UnsupportedOperation(String, Vec<String>),
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
            ErrorKind::UnsupportedOperation(operation, arguments) => {
                let mut concated_args = String::new();
                for arg in 0..arguments.len() - 2 {
                    concated_args.push_str(arguments.get(arg).unwrap());
                    concated_args.push_str(", ");
                }

                concated_args.push_str(arguments.get(arguments.len() - 2).unwrap());
                concated_args.push_str(" And ");
                concated_args.push_str(arguments.last().unwrap());
                
                return format!("The Operation '{}' Can Not Be Applied To {}.", operation, concated_args)
            }
        }
        .to_owned()
    }
}
