use crate::LendingIterator;

/// A lending iterator over windows.
///
/// This `struct` is created by the [`windows`] method on [`ToLendingIterator`]. See
/// its documentation for more.
///
/// [`ToLendingIterator`]: crate::ToLendingIterator
/// [`windows`]: crate::ToLendingIterator::windows
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Windows<I: Iterator> {
    iter: I,
    size: usize,
    buf: Vec<I::Item>,
}

impl<I: Iterator> Windows<I> {
    pub(crate) fn new(mut iter: I, size: usize) -> Self {
        let buf = iter.by_ref().take(size - 1).collect();
        Self { iter, size, buf }
    }
}

impl<I: Iterator> LendingIterator for Windows<I> {
    type Item<'a> = &'a [I::Item]
        where
            Self: 'a
    ;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|next| {
            if self.buf.len() == self.size * 2 - 1 {
                self.buf.drain(..self.size);
            }
            self.buf.push(next);
            &self.buf[self.buf.len() - self.size..]
        })
    }
}
