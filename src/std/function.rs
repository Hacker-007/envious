//! The Function struct represents a function, either builtin or user-created.
//! The Function struct contains the number of parameters, the types of the parameters, and the actual function.
//! The definition might change to have an optional function.

use super::standard_library::Return;
use crate::semantic_analyzer::types::Types;

pub struct Function {
    pub name: String,
    pub number_of_args: usize,
    pub parameter_types: Vec<Types>,
    pub return_type: Types,
    pub function: Option<fn(usize, &str, &[String]) -> Return>,
}

impl Function {
    /// Constructs a new function.
    ///
    /// # Arguments
    /// `name` - The name of the function.
    /// `number_of_args` - The number of arguments.
    /// `parameter_types` - The types of the parameters.
    /// `return_type` - The return type of the function.
    /// `function` - The actual fuction to call.
    pub fn new(
        name: &str,
        number_of_args: usize,
        parameter_types: Vec<Types>,
        return_type: Types,
        function: Option<fn(usize, &str, &[String]) -> Return>,
    ) -> Function {
        Function {
            name: name.to_owned(),
            number_of_args,
            parameter_types,
            return_type,
            function,
        }
    }
}
