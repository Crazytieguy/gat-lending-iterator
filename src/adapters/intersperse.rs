use core::fmt;

use crate::{LendingIterator, Peekable};

// WAITING ON: Peekable resolution, although we could always defer to `TrustedLen` to bind instead of `Peekable`

/// see [`LendingIterator::intersperse`]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Intersperse<'this, I: 'this>
where
    I::Item<'this>: Clone,
    I: LendingIterator,
{
    iter: Peekable<'this, I>,
    separator: I::Item<'this>,
    needs_sep: bool,
}
impl<'this, I> Intersperse<'this, I>
where
    I::Item<'this>: Clone,
    I: LendingIterator,
{
    pub(crate) fn new(iter: I, separator: I::Item<'this>) -> Self {
        Self {
            iter: Peekable::new(iter),
            separator,
            needs_sep: false,
        }
    }
}

#[must_use = "lenders are lazy and do nothing unless consumed"]
pub struct IntersperseWith<'this, I, G>
where
    I: LendingIterator,
{
    separator: G,
    iter: Peekable<'this, I>,
    needs_sep: bool,
}
impl<'this, I, G> IntersperseWith<'this, I, G>
where
    I: LendingIterator,
    G: FnMut() -> I::Item<'this>,
{
    pub(crate) fn new(iter: I, seperator: G) -> Self {
        Self {
            iter: Peekable::new(iter),
            separator: seperator,
            needs_sep: false,
        }
    }
}
