#![allow(unused)]

use crate::{
    context::CompilationContext,
    lex::{token::buffer::TokenizedBuffer, Lexer},
    source::SourceId,
};

pub mod ast;
pub mod compiler;
pub mod diagnostics;
pub mod evaluator;
pub mod source;

pub(crate) mod context;
pub(crate) mod dense;
pub(crate) mod lex;
pub(crate) mod parser;
