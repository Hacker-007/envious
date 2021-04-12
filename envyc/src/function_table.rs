use std::collections::HashMap;

use crate::{
    error::{Error, Span},
    semantic_analyzer::types::Type,
};

pub struct FunctionTable {
    function_parameter_types: HashMap<usize, Vec<Type>>,
}

impl FunctionTable {
    pub fn add_function_definition(
        &mut self,
        function_name: usize,
        function_parameter_types: Vec<Type>,
    ) {
        self.function_parameter_types
            .insert(function_name, function_parameter_types);
    }

    pub fn get_function_definition<'a>(
        &self,
        function_name: usize,
        function_span: Span<'a>,
    ) -> Result<&Vec<Type>, Error<'a>> {
        if let Some(function_parameter_types) = self.function_parameter_types.get(&function_name) {
            Ok(function_parameter_types)
        } else {
            Err(Error::UnknownFunction(function_span))
        }
    }
}

impl Default for FunctionTable {
    fn default() -> Self {
        Self {
            function_parameter_types: HashMap::new(),
        }
    }
}
