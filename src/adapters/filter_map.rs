use crate::{LendingIterator, OptionTrait, SingleArgFnMut, SingleArgFnOnce};

/// A lending iterator that uses `f` to both filter and map elements from `iter`.
///
/// This `struct` is created by the [`filter_map`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`filter_map`]: crate::LendingIterator::filter_map
#[derive(Clone)]
pub struct FilterMap<I, F> {
    iter: I,
    f: F,
}

impl<I, F> FilterMap<I, F> {
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I, F> LendingIterator for FilterMap<I, F>
where
    I: LendingIterator,
    F: for<'a> SingleArgFnMut<I::Item<'a>>,
    for<'a> <F as SingleArgFnOnce<I::Item<'a>>>::Output: OptionTrait,
{
    type Item<'a> = <<F as SingleArgFnOnce<I::Item<'a>>>::Output as OptionTrait>::Item
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        loop {
            // SAFETY: see https://docs.rs/polonius-the-crab/0.3.1/polonius_the_crab/#the-arcanemagic
            let self_ = unsafe { &mut *(self as *mut Self) };
            if let Some(item) = self_.iter.next() {
                let output = (self_.f)(item).into_option();
                if output.is_some() {
                    return output;
                }
            } else {
                return None;
            }
        }
    }
}
