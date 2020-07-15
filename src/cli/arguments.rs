//! The Arguments struct deals with the arguments that were passed in to the program.
//! For example, the option of showing the time of execution is an argument.
//! These arguments are analyzed and collected into this struct.

use crate::errors::{error::Error, error_kind::ErrorKind};
use std::env;

pub struct Arguments {
    path: Option<String>,
    show_time: bool,
}

impl Arguments {
    /// Creates a new Arguments struct that contains all of the arguments passed in to the program.
    /// An error is reported if an unrecognized argument is passed in.
    pub fn new() -> Result<Arguments, Error> {
        let args = env::args().skip(1);
        let mut arguments = Arguments {
            path: None,
            show_time: false,
        };

        for arg in args {
            match arg.as_str() {
                "-t" | "--show-time" => arguments.show_time = true,
                _ if arguments.path.is_none() => arguments.path = Some(arg),
                _ => return Err(Error::message_only(ErrorKind::UnrecognizedArgument(arg))),
            }
        }

        Ok(arguments)
    }

    /// This function gets the path provided.
    /// This function returns an option because a REPL could be instantiated if no path was defined.
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// This function retuns if the 'show_time' parameter was passed in.
    pub fn show_time(&self) -> bool {
        self.show_time
    }
}
