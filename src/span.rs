//! A span is value with a position 

use std::ops::{Deref, DerefMut, Range};
use std::fmt::{Debug, Display};


/// A span is value with a position 
#[derive(Clone, PartialEq, Eq)]
pub struct Span<T: Debug + Clone + PartialEq + Eq + ?Sized> {
    /// The line
    pub line: Range<u64>,
    /// The coloumn
    pub coloumn: Range<u64>,
    /// The inner value
    pub inner: T,
}

impl<T: Debug + Clone + PartialEq + Eq + ?Sized> Deref for Span<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Debug + Clone + PartialEq + Eq + ?Sized> DerefMut for Span<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Debug + Display + Clone + PartialEq + Eq + ?Sized> Display for Span<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
impl<T: Debug + Clone + PartialEq + Eq + ?Sized> Debug for Span<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}