//! The Classification enum contains the various classifications for the tokens.
//! It is used for syntax highlighting in the REPL. In the future, it can be used
//! for syntax highlighting in text editors.

pub enum Classification {
    Whitespace(String),
    Type(String),
    Keyword(String),
    Values(String),
    Punctuation(String),
    Identifier(String),
}