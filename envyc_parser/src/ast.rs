use envyc_context::symbol::Symbol;
use envyc_source::snippet::Snippet;

#[derive(Debug)]
pub struct SnippetedSymbol {
    snippet: Snippet,
    symbol: Symbol,
}

impl SnippetedSymbol {
    pub fn new(snippet: Snippet, symbol: Symbol) -> Self {
        Self { snippet, symbol }
    }
}

#[derive(Debug)]
pub struct Program {
    members: Vec<Member>,
}

impl Program {
    pub fn new(members: Vec<Member>) -> Self {
        Self { members }
    }
}

#[derive(Debug)]
pub enum Member {
    GlobalStatement,
    Function(Function),
}

#[derive(Debug)]
pub struct Function {
    define_keyword: Snippet,
    identifier: SnippetedSymbol,
    left_parenthesis: Snippet,
    parameters: Vec<Parameter>,
    right_parenthesis: Snippet,
    equal_sign: Snippet,
}

impl Function {
    pub fn new(
        define_keyword: Snippet,
        identifier: SnippetedSymbol,
        left_parenthesis: Snippet,
        parameters: Vec<Parameter>,
        right_parenthesis: Snippet,
        equal_sign: Snippet,
    ) -> Self {
        Self {
            define_keyword,
            identifier,
            left_parenthesis,
            parameters,
            right_parenthesis,
            equal_sign,
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    name: SnippetedSymbol,
}

impl Parameter {
    pub fn new(name: SnippetedSymbol) -> Self {
        Self {
            name,
        }
    }
}