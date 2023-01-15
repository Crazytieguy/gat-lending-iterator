use crate::{Windows, WindowsMut};

pub trait ToLendingIterator: Iterator {
    fn windows(self, size: usize) -> Windows<Self>
    where
        Self: Sized,
    {
        Windows::new(self, size)
    }

    fn windows_mut(self, size: usize) -> WindowsMut<Self>
    where
        Self: Sized,
    {
        WindowsMut::new(self, size)
    }
}

impl<I: Iterator> ToLendingIterator for I {}
