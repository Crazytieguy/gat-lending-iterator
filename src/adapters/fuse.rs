use crate::LendingIterator;
use core::fmt;

/// A lending iterator that yields `None` forever after the underlying iterator
/// yields `None` once.
///
/// This `struct` is created by the [`fuse`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`fuse`]: crate::LendingIterator::fuse
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Fuse<I> {
    iter: Option<I>,
}

impl<I> Fuse<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter: Some(iter) }
    }
}

impl<I: fmt::Debug> fmt::Debug for Fuse<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fuse")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<I> LendingIterator for Fuse<I>
where
    I: LendingIterator,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        // SAFETY: see https://docs.rs/polonius-the-crab/0.3.1/polonius_the_crab/#the-arcanemagic
        let self_ = unsafe { &mut *(self as *mut Self) };
        let iter = self_.iter.as_mut()?;
        if let Some(item) = iter.next() {
            Some(item)
        } else {
            self.iter = None;
            None
        }
    }

    #[inline]
    #[allow(clippy::redundant_closure_for_method_calls)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter
            .as_ref()
            .map_or((0, Some(0)), |iter| iter.size_hint())
    }
}

#[cfg(test)]
mod tests {
    use crate::{LendingIterator, ToLendingIterator};

    struct OneThenError {
        yielded: bool,
    }

    impl OneThenError {
        fn new() -> Self {
            Self { yielded: false }
        }
    }

    impl LendingIterator for OneThenError {
        type Item<'a> = i32
        where
            Self: 'a;

        fn next(&mut self) -> Option<Self::Item<'_>> {
            if !self.yielded {
                self.yielded = true;
                Some(1)
            } else {
                None
            }
        }
    }

    fn identity(x: i32) -> i32 {
        x
    }

    #[test]
    fn fuse_basic() {
        let mut iter = OneThenError::new().fuse();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fuse_regular_iter() {
        let result: Vec<_> = (0..5)
            .into_lending()
            .fuse()
            .map(identity)
            .into_iter()
            .collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }
}
