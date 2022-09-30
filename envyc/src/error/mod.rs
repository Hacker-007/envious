use crate::location::Location;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct CompilerError {
    pub(crate) kind: CompilerErrorKind,
    pub(crate) location: Location,
}

impl CompilerError {
    pub fn new(kind: CompilerErrorKind, location: Location) -> Self {
        Self { kind, location }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum CompilerErrorKind {
    UnrecognizedCharacter,
    IntegerOverflow,
}
