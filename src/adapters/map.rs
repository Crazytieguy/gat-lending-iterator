use crate::{LendingIterator, SingleArgFnMut, SingleArgFnOnce};
use core::fmt;

/// A lending iterator that maps the elements of `iter` with `f`.
///
/// This `struct` is created by the [`map`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`map`]: crate::LendingIterator::map
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Map<I, F> {
    iter: I,
    f: F,
}

impl<I, F> Map<I, F> {
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I: fmt::Debug, F> fmt::Debug for Map<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map").field("iter", &self.iter).finish()
    }
}

impl<I, F> LendingIterator for Map<I, F>
where
    I: LendingIterator,
    F: for<'a> SingleArgFnMut<I::Item<'a>>,
{
    type Item<'a> = <F as SingleArgFnOnce<I::Item<'a>>>::Output
        where
            Self: 'a;

    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(&mut self.f)
    }
}

/// An iterator that maps the elements of `iter` with `f`.
///
/// This `struct` is created when [`IntoIterator::into_iter`] is called on [`Map`].
pub struct IntoIter<I, F> {
    iter: I,
    f: F,
}

impl<I, F, O> Iterator for IntoIter<I, F>
where
    I: LendingIterator,
    F: FnMut(I::Item<'_>) -> O,
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.f)
    }
}

impl<I, F, O> IntoIterator for Map<I, F>
where
    I: LendingIterator,
    F: FnMut(I::Item<'_>) -> O,
{
    type Item = O;
    type IntoIter = IntoIter<I, F>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.iter,
            f: self.f,
        }
    }
}
