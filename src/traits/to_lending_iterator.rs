use crate::{IntoLending, LendRefs, LendRefsMut};
#[cfg(feature = "alloc")]
use crate::{Windows, WindowsMut};
/// An extension trait for iterators that allows turning them into lending iterators (over windows of elements).
pub trait ToLendingIterator: IntoIterator {
    /// Turns this iterator into a lending iterator over windows of elements (&\[Item\]).
    ///
    /// `Windows` is backed by a buffer that grows to at most size * 2.
    /// This was chosen as a compromise between memory usage and time complexity:
    /// if the buffer was limited to size `size`, we would need to shift all the elements
    /// on every iteration.
    #[cfg(feature = "alloc")]
    fn windows(self, size: usize) -> Windows<Self::IntoIter>
    where
        Self: Sized,
    {
        Windows::new(self.into_iter(), size)
    }

    /// Turns this iterator into a lending iterator over mutable windows of elements (&mut \[Item\]).
    ///
    /// `WindowsMut` is backed by a buffer that grows to at most size * 2.
    /// This was chosen as a compromise between memory usage and time complexity:
    /// if the buffer was limited to size `size`, we would need to shift all the elements
    /// on every iteration.
    #[cfg(feature = "alloc")]
    fn windows_mut(self, size: usize) -> WindowsMut<Self::IntoIter>
    where
        Self: Sized,
    {
        WindowsMut::new(self.into_iter(), size)
    }

    /// Turns this iterator into a lending iterator trivially.
    fn into_lending(self) -> IntoLending<Self::IntoIter>
    where
        Self: Sized,
    {
        IntoLending::new(self.into_iter())
    }

    /// Turns this iterator into a lending iterator that lends references
    /// to the iterator's items.
    fn lend_refs(self) -> LendRefs<Self::IntoIter>
    where
        Self: Sized,
    {
        LendRefs::new(self.into_iter())
    }

    /// Turns this iterator into a lending iterator that lends mutable references
    /// to the iterator's items.
    fn lend_refs_mut(self) -> LendRefsMut<Self::IntoIter>
    where
        Self: Sized,
    {
        LendRefsMut::new(self.into_iter())
    }
}

impl<I: IntoIterator> ToLendingIterator for I {}
