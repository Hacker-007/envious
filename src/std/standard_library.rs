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
use crate::{semantic_analyzer::types::Types, errors::{error::Error, error_kind::ErrorKind}};
use std::collections::HashMap;

macro_rules! initialize_functions {
    ($function_mapper: ident, $(($name: expr, $num_params: expr, $parameter_types: expr, $return_types: expr, $function: path)),*) => {
        $($function_mapper.insert($name.to_owned(), Function::new($name, $num_params, $parameter_types, $return_types, Some($function)));)*
    };
}

pub type Return = Result<(String, usize), Error>;

pub struct StandardLibrary(HashMap<String, Function>);

impl StandardLibrary {
    /// Initializes the StandardLibrary with the different functions.
    pub fn new() -> StandardLibrary {
        let mut function_mapper = HashMap::new();
        StandardLibrary::initialize_io_module(&mut function_mapper);
        StandardLibrary(function_mapper)
    }

    fn initialize_io_module(function_mapper: &mut HashMap<String, Function>) {
        initialize_functions!(
            function_mapper,
            (
                "print",
                1,
                vec![Types::Any],
                Types::Void,
                io::print
            ),
            (
                "println",
                1,
                vec![Types::Any],
                Types::Void,
                io::println
            )
            // ,
            // (
            //     "read_file",
            //     1..2,
            //     vec![Types::String],
            //     Types::String,
            //     io::read_file
            // )
        );
    }

    /// Gets the function with the given name. An error is returned if the function does not exist.
    ///
    /// # Arguments
    /// `pos` - The position where the function was called.
    /// `name` - The name of the function.
    pub fn get_function(&self, pos: usize, name: &str) -> Result<&Function, Error> {
        self.0
            .get(name)
            .ok_or_else(|| Error::new(ErrorKind::UnknownFunction, pos))
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
            (function.function.unwrap())(pos, indent, parameters)
        } else {
            Err(Error::new(ErrorKind::UnknownFunction, pos))
        }
    }
}
