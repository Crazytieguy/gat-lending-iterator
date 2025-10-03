use crate::LendingIterator;
use core::fmt;

/// A lending iterator that repeats endlessly.
///
/// This `struct` is created by the [`cycle`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`cycle`]: crate::LendingIterator::cycle
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Cycle<I> {
    orig: I,
    iter: I,
    first_iteration: bool,
}

impl<I: Clone> Cycle<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            orig: iter.clone(),
            iter,
            first_iteration: true,
        }
    }
}

impl<I: fmt::Debug> fmt::Debug for Cycle<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cycle")
            .field("iter", &self.iter)
            .finish_non_exhaustive()
    }
}

impl<I> LendingIterator for Cycle<I>
where
    I: Clone + LendingIterator,
{
    type Item<'a>
        = I::Item<'a>
    where
        Self: 'a;

    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        loop {
            // SAFETY: The polonius pattern is needed because self.iter.next() returns
            // an item that borrows from self, which would prevent us from modifying
            // self.iter later. By using unsafe, we tell the compiler that we know
            // the returned item doesn't overlap with the modification to self.iter.
            // This is sound because we only reset self.iter when it returns None.
            let self_ = unsafe { &mut *(self as *mut Self) };
            match self_.iter.next() {
                None => {
                    if self.first_iteration {
                        return None;
                    }
                    self.iter = self.orig.clone();
                }
                Some(item) => {
                    self.first_iteration = false;
                    return Some(item);
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.orig.size_hint() {
            (0, Some(0)) => (0, Some(0)),
            (0, _) => (0, None),
            _ => (usize::MAX, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LendingIterator, ToLendingIterator};

    fn identity(x: i32) -> i32 {
        x
    }

    #[test]
    fn cycle_basic() {
        let result: Vec<_> = (0..3)
            .into_lending()
            .cycle()
            .take(10)
            .map(identity)
            .into_iter()
            .collect();
        assert_eq!(result, vec![0, 1, 2, 0, 1, 2, 0, 1, 2, 0]);
    }

    #[test]
    fn cycle_empty() {
        let result: Vec<_> = core::iter::empty::<i32>()
            .into_lending()
            .cycle()
            .take(5)
            .map(identity)
            .into_iter()
            .collect();
        assert_eq!(result, Vec::<i32>::new());
    }
}
