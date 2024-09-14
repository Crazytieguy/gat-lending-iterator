use crate::LendingIterator;
use core::fmt;

/// A lending iterator that that rejects elements while `predicate` returns `true`.
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SkipWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I, P> SkipWhile<I, P> {
    pub(crate) fn new(iter: I, predicate: P) -> Self {
        Self {
            iter,
            flag: false,
            predicate,
        }
    }
}

impl<I: fmt::Debug, P> fmt::Debug for SkipWhile<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TakeWhile")
            .field("iter", &self.iter)
            .field("flag", &self.flag)
            .finish_non_exhaustive()
    }
}

impl<I, P> LendingIterator for SkipWhile<I, P>
where
    I: LendingIterator,
    P: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    type Item<'a> = I::Item<'a> where Self: 'a;

    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.flag {
            return self.iter.next()
        }
        loop {
            // SAFETY: see https://docs.rs/polonius-the-crab/0.3.1/polonius_the_crab/#the-arcanemagic
            let self_ = unsafe { &mut *(self as *mut Self) };
            if let Some(item) = self_.iter.next() {
                if !(self_.predicate)(&item) {
                    self_.flag = true;
                    return Some(item);
                }
            } else {
                return None;
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }

    // TODO: there's a `fold` optimization possible here,
    // but for some reason the lifetimes don't type check
}
