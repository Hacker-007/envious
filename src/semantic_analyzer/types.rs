//! The Types enum maintains most of the types withtin the Envious programming language.
//! These types will be used by the type checker to validate that the expressions created adhere to the types specified.

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Types {
    Int,
    Float,
    Boolean,
    String,
    
    Any,
    Void
}

impl Into<String> for Types {
    fn into(self) -> String {
        match self {
            Types::Int => "An Int",
            Types::Float => "A Float",
            Types::Boolean => "A Boolean",
            Types::String => "A String",
            Types::Any => "Any",
            Types::Void => "A Void",
        }.to_owned()
    }
}