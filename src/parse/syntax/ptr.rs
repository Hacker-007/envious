//! Note that the following pointer implementation comes entirely from the
//! Rust compiler team. The link to the code is https://github.com/rust-lang/rust/blob/master/compiler/rustc_ast/src/ptr.rs.
//! 
//! The main benefit of using this pointer abstraction is the flexibility
//! in internal implemenation that is provided. Furthermore, the Rust
//! compiler team has added numerous utility methods that make this
//! implementation quite ergonomic to work with.
//! 
//! Initially, the implementation used `Box`, but after discovering
//! `P` and that the implementations can be easily switched over,
//! it was an easy decision to switch over.

use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

pub struct P<T: ?Sized> {
    ptr: Box<T>,
}

#[allow(non_snake_case)]
pub fn P<T: 'static>(value: T) -> P<T> {
    P {
        ptr: Box::new(value),
    }
}

impl<T: 'static> P<T> {
    pub fn and_then<U, F>(self, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        f(*self.ptr)
    }

    pub fn into_inner(self) -> T {
        *self.ptr
    }

    pub fn map<F>(mut self, f: F) -> P<T>
    where
        F: FnOnce(T) -> T,
    {
        let x = f(*self.ptr);
        *self.ptr = x;
        self
    }

    pub fn filter_map<F>(mut self, f: F) -> Option<P<T>>
    where
        F: FnOnce(T) -> Option<T>,
    {
        *self.ptr = f(*self.ptr)?;
        Some(self)
    }
}

impl<T: ?Sized> Deref for P<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T: ?Sized> DerefMut for P<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}

impl<T: 'static + Clone> Clone for P<T> {
    fn clone(&self) -> Self {
        P((**self).clone())
    }
}

impl<T: ?Sized + Debug> Debug for P<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.ptr, f)
    }
}

impl<T: Display> Display for P<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl<T> fmt::Pointer for P<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}
