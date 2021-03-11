use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment<T> {
    scopes: Vec<Scope<T>>,
}

impl<T: Copy> Environment<T> {
    pub fn new_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    pub fn remove_top_scope(&mut self) -> Option<Scope<T>> {
        self.scopes.pop()
    }

    pub fn define(&mut self, id: usize, value: T) {
        if self.scopes.is_empty() {
            self.new_scope();
        }

        self.scopes.last_mut().unwrap().insert(id, value);
    }

    pub fn get(&self, id: usize) -> Option<T> {
        self.scopes.iter().rev().find_map(|scope| scope.get(id))
    }
}

impl<T> Default for Environment<T> {
    fn default() -> Self {
        Self { scopes: Vec::new() }
    }
}

#[derive(Debug)]
pub struct Scope<T> {
    inner: HashMap<usize, T>,
}

impl<T: Copy> Scope<T> {
    pub fn insert(&mut self, id: usize, value: T) {
        self.inner.insert(id, value);
    }

    pub fn get(&self, id: usize) -> Option<T> {
        self.inner.get(&id).copied()
    }
}

impl<T> Default for Scope<T> {
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}
