use crate::LendingIterator;
use core::fmt;

/// A lending iterator that maintains internal state and maps elements using that state.
///
/// This `struct` is created by the [`scan`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`scan`]: crate::LendingIterator::scan
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Scan<I, St, F> {
    iter: I,
    f: F,
    state: St,
}

impl<I, St, F> Scan<I, St, F> {
    pub(crate) fn new(iter: I, state: St, f: F) -> Self {
        Self { iter, f, state }
    }
}

impl<I: fmt::Debug, St: fmt::Debug, F> fmt::Debug for Scan<I, St, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scan")
            .field("iter", &self.iter)
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

impl<I, St, F, B> LendingIterator for Scan<I, St, F>
where
    I: LendingIterator,
    F: for<'a> FnMut(&'a mut St, I::Item<'a>) -> Option<B>,
{
    type Item<'a>
        = B
    where
        Self: 'a;

    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        let item = self.iter.next()?;
        (self.f)(&mut self.state, item)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }
}

#[cfg(test)]
mod tests {
    use crate::{LendingIterator, ToLendingIterator};

    #[test]
    fn scan_basic() {
        let mut result = Vec::new();
        (1..=5)
            .into_lending()
            .scan(0, |state: &mut i32, x| {
                *state += x;
                Some(*state)
            })
            .for_each(|x| result.push(x));
        assert_eq!(result, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn scan_early_termination() {
        let mut result = Vec::new();
        (1..=10)
            .into_lending()
            .scan(0, |state: &mut i32, x| {
                *state += x;
                if *state > 10 {
                    None
                } else {
                    Some(*state)
                }
            })
            .for_each(|x| result.push(x));
        assert_eq!(result, vec![1, 3, 6, 10]);
    }
}
