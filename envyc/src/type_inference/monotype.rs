use std::rc::Rc;

use crate::error::Span;

#[derive(Debug)]
pub enum Monotype<'a> {
    Int,
    Float,
    Boolean,
    Char,
    Void,
    Never,
    Variable(usize),
    Function {
        parameters: Vec<MonotypeRef<'a>>,
        ret: MonotypeRef<'a>
    }
}

pub type MonotypeRef<'a> = (Span<'a>, Rc<Monotype<'a>>);
