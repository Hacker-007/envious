//! The StandardLibrary struct is a wrapper over a HashMap and contains all of the different functions considered to be in the standard library.
//! Currently, there are no modules. This means that all of the defined functions will be under the same "module". This will change in the future.
//! The StandardLibrary will usually be invoked by the code generator.
//!
//! # Example
//! ```
//! let standard_library = StandardLibrary::new();
//! standard_library.compile_function(1, "print", "Hello, World!")
//! ```

use super::{function::Function, io};
use crate::errors::{error::Error, error_kind::ErrorKind};
use std::collections::HashMap;

pub type Return = Result<String, Error>;

pub struct StandardLibrary(HashMap<String, Function>);

impl StandardLibrary {
    /// Initializes the StandardLibrary with the different functions.
    pub fn new() -> StandardLibrary {
        let mut function_mapper = HashMap::new();
        StandardLibrary::initialize_io_module(&mut function_mapper);

        StandardLibrary(function_mapper)
    }

    fn initialize_io_module(function_mapper: &mut HashMap<String, Function>) {
        function_mapper.insert("print".to_owned(), Function::new("print", 1..2, io::print));
        function_mapper.insert(
            "println".to_owned(),
            Function::new("println", 1..2, io::println),
        );
    }

    /// Executes the function with the given name and returns the result.
    /// This function will return an error if the function does not exist or function fails.
    ///
    /// # Arguments
    /// `pos` - The position where the function was called.
    /// `indent` - The current level of indent.
    /// `name` - The name of the function called.
    /// `parameters` - The parameters to the function.
    pub fn compile_function(
        &self,
        pos: usize,
        indent: &str,
        name: &str,
        parameters: &[String],
    ) -> Return {
        if let Some(function) = self.0.get(name) {
            function.get_function()(pos, indent, parameters)
        } else {
            Err(Error::new(ErrorKind::UnknownFunction, pos))
        }
    }

    // /// Indents the code based on if the formatting feature was turned on and what the current indent size is.
    // ///
    // /// # Arguments
    // /// `current_indent` - The current level of indentation.
    // pub fn indent(format_code: bool, current_indent: &str) -> String {
    //     if format_code {
    //         format!("{}{}", current_indent, " ".repeat(4))
    //     } else {
    //         String::new()
    //     }
    // }
}
