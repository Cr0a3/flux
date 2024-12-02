use std::ops::{Deref, DerefMut, Range};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span<T: Debug + Clone + PartialEq + Eq + ?Sized> {
    pub line: Range<u64>,
    pub coloumn: Range<u64>,
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