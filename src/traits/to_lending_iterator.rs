use crate::{IntoLending, Windows, WindowsMut};

/// An extension trait for iterators that allows turning them into lending iterators (over windows of elements).
pub trait ToLendingIterator: IntoIterator {
    /// Turns this iterator into a lending iterator over windows of elements (&\[Item\]).
    ///
    /// `Windows` is backed by a buffer that grows to at most size * 2.
    /// This was chosen as a compromise between memory usage and time complexity:
    /// if the buffer was limited to size `size`, we would need to shift all the elements
    /// on every iteration.
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
}

impl<I: IntoIterator> ToLendingIterator for I {}
