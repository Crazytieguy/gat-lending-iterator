use crate::{Windows, WindowsMut};

/// An extension trait for iterators that allows turning them into lending iterators (over windows of elements).
pub trait ToLendingIterator: Iterator {
    /// Turns this iterator into a lending iterator over windows of elements (&\[Item\]).
    ///
    /// `Windows` is backed by a buffer that grows to at most size * 2.
    /// This was chosen as a compromise between memory usage and time complexity:
    /// if the buffer was limited to size `size`, we would need to shift all the elements
    /// on every iteration.
    fn windows(self, size: usize) -> Windows<Self>
    where
        Self: Sized,
    {
        Windows::new(self, size)
    }

    /// Turns this iterator into a lending iterator over mutable windows of elements (&mut \[Item\]).
    ///
    /// `WindowsMut` is backed by a buffer that grows to at most size * 2.
    /// This was chosen as a compromise between memory usage and time complexity:
    /// if the buffer was limited to size `size`, we would need to shift all the elements
    /// on every iteration.
    fn windows_mut(self, size: usize) -> WindowsMut<Self>
    where
        Self: Sized,
    {
        WindowsMut::new(self, size)
    }
}

impl<I: Iterator> ToLendingIterator for I {}
