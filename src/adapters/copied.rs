use std::ops::Deref;

use crate::LendingIterator;

/// A lending iterator that copies the elements of an underlying lending iterator.
///
/// This `struct` is created by the [`copied`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`copied`]: crate::LendingIterator::copied
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Copied<I> {
    iter: I,
}

impl<I> Copied<I> {
    pub(crate) fn new(iter: I) -> Copied<I> {
        Copied { iter }
    }
}

impl<I> LendingIterator for Copied<I>
where
    I: LendingIterator,
    for<'a> I::Item<'a>: Deref,
    for<'a> <I::Item<'a> as Deref>::Target: Copy,
{
    type Item<'a> = <I::Item<'a> as Deref>::Target
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|item| *item.deref())
    }
}

pub struct IntoIter<I> {
    iter: I,
}

impl<I, T> Iterator for IntoIter<I>
where
    I: LendingIterator,
    for<'a> I::Item<'a>: Deref<Target = T>,
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| *item.deref())
    }
}

impl<I, T> IntoIterator for Copied<I>
where
    I: LendingIterator,
    for<'a> I::Item<'a>: Deref<Target = T>,
    T: Copy,
{
    type Item = T;
    type IntoIter = IntoIter<I>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { iter: self.iter }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LendingIterator, ToLendingIterator};

    #[test]
    fn copied_basic() {
        let data = vec![1, 2, 3];
        let result: Vec<_> = data
            .lend_refs()
            .copied()
            .into_iter()
            .collect();
        assert_eq!(result, vec![1, 2, 3]);
    }
}
