use crate::LendingIterator;

/// A lending iterator that iterates over an iterator.
#[derive(Clone)]
pub struct IntoLending<I: Iterator> {
    iter: I,
}

impl<I: Iterator> IntoLending<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I: Iterator> LendingIterator for IntoLending<I> {
    type Item<'a> = I::Item where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
