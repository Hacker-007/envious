use crate::location::Location;

#[derive(Debug, Clone, Copy)]
pub(crate) enum CompilerError {
	IntegerOverflow(Location),
	FloatOverflow(Location),
	UnterminatedString(Location),
	UnrecognizedCharacter(Location),
}