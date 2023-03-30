use lasso::{Rodeo, Spur};

pub type SymbolId = Spur;

#[derive(Debug, Default)]
pub struct SymbolInterner {
    interner: Rodeo,
}

impl SymbolInterner {
    /// Returns the symbol id of the given value. If the
    /// value is not in the `SymbolInterner`, then the
    /// value is interened in the structure and its
    /// associated id is returned.
    ///
    /// # Examples
    /// ```
    /// use envyc::context::symbol_interner::SymbolInterner;
    ///
    /// let mut symbol_interner = SymbolInterner::default();
    /// let int_symbol = symbol_interner.get_symbol("Int");
    /// ```
    pub fn get_symbol(&mut self, key: &str) -> SymbolId {
        self.interner.get_or_intern(key)
    }

    /// Returns the key associated with the `id`. Note that
    /// this method assumes that the `id` was previously
    /// produced by the interner.
    /// 
    /// # Examples
    /// ```
    /// use envyc::context::symbol_interner::SymbolInterner;
    ///
    /// let mut symbol_interner = SymbolInterner::default();
    /// let int_symbol = symbol_interner.get_symbol("Int");
    /// assert_eq!(symbol_interner.get_key(&int_symbol), "Int");
    /// ```
    pub fn get_key(&self, id: &SymbolId) -> &str {
        self.interner.resolve(id)
    }
}
