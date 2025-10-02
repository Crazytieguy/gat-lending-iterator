use crate::{LendingIterator, OptionTrait, SingleArgFnMut, SingleArgFnOnce};
use core::fmt;

/// A lending iterator that maps elements and yields while the mapping returns Some.
///
/// This `struct` is created by the [`map_while`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`map_while`]: crate::LendingIterator::map_while
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct MapWhile<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> MapWhile<I, P> {
    pub(crate) fn new(iter: I, predicate: P) -> Self {
        Self { iter, predicate }
    }
}

impl<I: fmt::Debug, P> fmt::Debug for MapWhile<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapWhile")
            .field("iter", &self.iter)
            .finish_non_exhaustive()
    }
}

impl<I, P> LendingIterator for MapWhile<I, P>
where
    I: LendingIterator,
    P: for<'a> SingleArgFnMut<I::Item<'a>>,
    for<'a> <P as SingleArgFnOnce<I::Item<'a>>>::Output: OptionTrait,
{
    type Item<'a> = <<P as SingleArgFnOnce<I::Item<'a>>>::Output as OptionTrait>::Item
    where
        Self: 'a;

    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        let item = self.iter.next()?;
        (self.predicate)(item).into_option()
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

    fn identity(x: i32) -> i32 {
        x
    }

    #[test]
    fn map_while_basic() {
        let result: Vec<_> = (0..10)
            .into_lending()
            .map_while(|x| if x < 5 { Some(x * 2) } else { None })
            .map(identity)
            .into_iter()
            .collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn map_while_early_termination() {
        let result: Vec<_> = (1..=10)
            .into_lending()
            .map_while(|x| if x <= 5 { Some(x * x) } else { None })
            .map(identity)
            .into_iter()
            .collect();
        assert_eq!(result, vec![1, 4, 9, 16, 25]);
    }
}
