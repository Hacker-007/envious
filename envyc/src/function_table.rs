use std::collections::HashMap;

use crate::{error::{Error, Span}, parser::ast::Prototype};

pub struct FunctionTable<'a> {
    function_definitions: HashMap<usize, &'a Prototype<'a>>,
}

impl<'a> FunctionTable<'a> {
    pub fn add_function_definition(&mut self, function_name: usize, function_prototype: &'a Prototype<'a>) {
        self.function_definitions.insert(function_name, function_prototype);
    }

    pub fn get_function_definition(&self, function_name: usize, function_span: Span<'a>) -> Result<&'a Prototype<'a>, Error> {
        if let Some(function_definition) = self.function_definitions.get(&function_name) {
            Ok(*function_definition)
        } else {
            Err(Error::UnknownFunction(function_span))
        }
    }
}

impl<'a> Default for FunctionTable<'a> {
    fn default() -> Self {
        Self {
            function_definitions: HashMap::new(),
        }
    }
}