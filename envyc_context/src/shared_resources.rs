use envyc_error::error_handler::ErrorHandler;

use crate::interner::Interner;

#[derive(Debug)]
pub struct SharedResources<E: ErrorHandler> {
    pub interner: Interner,
    pub error_handler: E,
}

impl<E: ErrorHandler> SharedResources<E> {
    pub fn new(error_handler: E) -> Self {
        Self {
            interner: Interner::default(),
            error_handler,
        }
    }
}