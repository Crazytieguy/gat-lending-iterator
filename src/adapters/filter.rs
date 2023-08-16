use crate::LendingIterator;

/// A lending iterator that filters the elements of `iter` with `predicate`.
///
/// This `struct` is created by the [`filter`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`filter`]: crate::LendingIterator::filter
pub struct Filter<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> Filter<I, P> {
    pub(crate) fn new(iter: I, predicate: P) -> Self {
        Self { iter, predicate }
    }
}

impl<I, P> LendingIterator for Filter<I, P>
where
    I: LendingIterator,
    P: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        loop {
            // SAFETY: see https://docs.rs/polonius-the-crab/0.3.1/polonius_the_crab/#the-arcanemagic
            let self_ = unsafe { &mut *(self as *mut Self) };
            if let Some(item) = self_.iter.next() {
                if (self_.predicate)(&item) {
                    return Some(item);
                }
            } else {
                return None;
            }
        }
    }
}
