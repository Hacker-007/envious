//! The Function struct represents a function, either builtin or user-created.
//! The Function struct contains the number of parameters, the types of the parameters, and the actual function.
//! The definition might change to have an optional function.

use super::standard_library::Return;
use std::ops::Range;

pub struct Function {
    // name: String,
    // number_of_args: Range<usize>,
    // types: Vec<Type>,
    function: fn(usize, &str, &[String]) -> Return,
}

impl Function {
    /// Constructs a new function.
    ///
    /// # Arguments
    /// `name` - The name of the function.
    /// `number_of_args` - The number of arguments.
    /// `function` - The actual fuction to call.
    pub fn new(
        _name: &str,
        _number_of_args: Range<usize>,
        function: fn(usize, &str, &[String]) -> Return,
    ) -> Function {
        Function {
            // name: name.to_owned(),
            // number_of_args,
            function,
        }
    }

    /// Returns the function associated with this struct.
    pub fn get_function(&self) -> fn(usize, &str, &[String]) -> Return {
        self.function
    }
}
