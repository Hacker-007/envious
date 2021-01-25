use crate::span::Span;

#[derive(Debug)]
pub enum Error {
    IntegerOverflow(Span),
    FloatOverflow(Span),
    UnterminatedString(Span),
    UnrecognizedCharacter(Span),
}
