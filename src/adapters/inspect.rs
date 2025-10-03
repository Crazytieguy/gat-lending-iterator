use crate::LendingIterator;
use core::fmt;

/// A lending iterator that calls a function with a reference to each element
/// before yielding it.
///
/// This `struct` is created by the [`inspect`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`inspect`]: crate::LendingIterator::inspect
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Inspect<I, F> {
    iter: I,
    f: F,
}

impl<I, F> Inspect<I, F> {
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I: fmt::Debug, F> fmt::Debug for Inspect<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inspect")
            .field("iter", &self.iter)
            .finish_non_exhaustive()
    }
}

impl<I, F> LendingIterator for Inspect<I, F>
where
    I: LendingIterator,
    F: for<'a> FnMut(&I::Item<'a>),
{
    type Item<'a>
        = I::Item<'a>
    where
        Self: 'a;

    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        let item = self.iter.next()?;
        (self.f)(&item);
        Some(item)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use crate::{LendingIterator, ToLendingIterator};

    fn identity(x: i32) -> i32 {
        x
    }

    #[test]
    fn inspect_basic() {
        let result: Vec<_> = (0..5)
            .into_lending()
            .inspect(|_| {})
            .map(identity)
            .into_iter()
            .collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn inspect_with_filter() {
        let result: Vec<_> = (0..10)
            .into_lending()
            .inspect(|_| {})
            .filter(|&x| x % 2 == 0)
            .map(identity)
            .into_iter()
            .collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn inspect_with_windows() {
        let result: Vec<_> = (0..5)
            .windows(2)
            .inspect(|_| {})
            .map(|w: &[i32]| w[0] + w[1])
            .into_iter()
            .collect();
        assert_eq!(result, vec![1, 3, 5, 7]);
    }
}
