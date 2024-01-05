use crate::LendingIterator;
use alloc::vec::Vec;

/// A lending iterator over mutable windows.
///
/// This `struct` is created by the [`windows_mut`] method on [`ToLendingIterator`]. See
/// its documentation for more.
///
/// [`ToLendingIterator`]: crate::ToLendingIterator
/// [`windows_mut`]: crate::ToLendingIterator::windows_mut
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct WindowsMut<I: Iterator> {
    iter: I,
    size: usize,
    buf: Vec<I::Item>,
}

impl<I: Iterator> WindowsMut<I> {
    pub(crate) fn new(mut iter: I, size: usize) -> Self {
        let buf = iter.by_ref().take(size - 1).collect();
        Self { iter, size, buf }
    }
}

impl<I: Iterator> LendingIterator for WindowsMut<I> {
    type Item<'a> = &'a mut [I::Item]
        where
            Self: 'a
    ;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|next| {
            if self.buf.len() == self.size * 2 - 1 {
                self.buf.drain(..self.size);
            }
            self.buf.push(next);
            let range = self.buf.len() - self.size..;
            &mut self.buf[range]
        })
    }
}
