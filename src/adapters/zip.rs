use crate::LendingIterator;

/// A lending iterator that iterates two other lending iterators simultaneously.
///
/// This `struct` is created by the [`zip`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`zip`]: crate::LendingIterator::zip
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Zip<A, B> {
    a: A,
    b: B,
}
impl<A, B> Zip<A, B> {
    pub(crate) fn new(a: A, b: B) -> Zip<A, B> {
        Zip { a, b }
    }
}

impl<A, B> LendingIterator for Zip<A, B>
where
    A: LendingIterator,
    B: LendingIterator,
{
    type Item<'a>
        = (A::Item<'a>, B::Item<'a>)
    where
        A: 'a,
        B: 'a;

    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        let a = self.a.next()?;
        let b = self.b.next()?;
        Some((a, b))
    }
}

#[cfg(test)]
mod tests {
    use crate::{LendingIterator, ToLendingIterator};

    fn make_pair(pair: (i32, i32)) -> (i32, i32) {
        pair
    }

    #[test]
    fn zip_basic() {
        let result: Vec<_> = (0..3)
            .into_lending()
            .zip((10..13).into_lending())
            .map(make_pair)
            .into_iter()
            .collect();
        assert_eq!(result, vec![(0, 10), (1, 11), (2, 12)]);
    }

    #[test]
    fn zip_unequal_lengths() {
        let result: Vec<_> = (0..5)
            .into_lending()
            .zip((10..12).into_lending())
            .map(make_pair)
            .into_iter()
            .collect();
        assert_eq!(result, vec![(0, 10), (1, 11)]);
    }
}
