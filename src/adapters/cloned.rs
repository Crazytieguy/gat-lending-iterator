use std::ops::Deref;

use crate::LendingIterator;

/// A lending iterator that clones the elements of an underlying lending iterator.
///
/// This `struct` is created by the [`cloned`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`cloned`]: crate::LendingIterator::cloned
pub struct Cloned<I> {
    iter: I,
}

impl<I> Cloned<I> {
    pub(crate) fn new(iter: I) -> Cloned<I> {
        Cloned { iter }
    }
}

impl<I> LendingIterator for Cloned<I>
where
    I: LendingIterator,
    for<'a> I::Item<'a>: Deref,
    for<'a> <I::Item<'a> as Deref>::Target: Clone,
{
    type Item<'a> = <I::Item<'a> as Deref>::Target
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|item| item.deref().clone())
    }
}

pub struct IntoIter<I> {
    iter: I,
}

impl<I, T> Iterator for IntoIter<I>
where
    I: LendingIterator,
    for<'a> I::Item<'a>: Deref<Target = T>,
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.deref().clone())
    }
}

impl<I, T> IntoIterator for Cloned<I>
where
    I: LendingIterator,
    for<'a> I::Item<'a>: Deref<Target = T>,
    T: Clone,
{
    type Item = T;
    type IntoIter = IntoIter<I>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { iter: self.iter }
    }
}
