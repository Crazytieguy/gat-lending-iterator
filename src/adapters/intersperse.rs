use crate::{HasNextLendingIterator, LendingIterator};

// DISCUSS: I cannot think of a good use case for this.

/// see [`Iterator::intersperse_with()`].
#[must_use = "lenders are lazy and do nothing unless consumed"]
pub struct IntersperseWith<I, G> {
    separator: G,
    iter: I,
    needs_sep: bool,
}
impl<I, G> IntersperseWith<I, G>
where
    I: HasNextLendingIterator,
    for<'all> G: FnMut(&'all ()) -> I::Item<'all>,
{
    pub(crate) fn new(iter: I, separator: G) -> Self {
        Self {
            iter,
            separator,
            needs_sep: false,
        }
    }
}
impl<I, G> LendingIterator for IntersperseWith<I, G>
where
    I: HasNextLendingIterator,
    for<'all> G: FnMut(&'all ()) -> I::Item<'all>,
{
    type Item<'a> = I::Item<'a>
        where
            Self: 'a
    ;

    fn next(&mut self) -> Option<I::Item<'_>> {
        if self.iter.has_next() {
            if self.needs_sep {
                self.needs_sep = false;
                Some((self.separator)(&()))
            } else {
                let needs_sep = &mut self.needs_sep;
                self.iter.next().map(|item| {
                    *needs_sep = true;
                    item
                })
            }
        } else {
            None
        }
    }
}
