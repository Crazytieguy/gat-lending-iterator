use crate::LendingIterator;

use core::fmt;

/// An iterator that rejects elements while `predicate` returns `true`.
///
/// see [std::iter::SkipWhile](https://doc.rust-lang.org/std/iter/struct.SkipWhile.html)
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct SkipWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I, P> SkipWhile<I, P> {
    pub(crate) fn new(iter: I, predicate: P) -> SkipWhile<I, P> {
        SkipWhile {
            iter,
            flag: false,
            predicate,
        }
    }
}

impl<I: fmt::Debug, P> fmt::Debug for SkipWhile<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SkipWhile")
            .field("iter", &self.iter)
            .field("flag", &self.flag)
            .finish()
    }
}

impl<I: LendingIterator, P> LendingIterator for SkipWhile<I, P>
where
    for<'all> P: FnMut(&I::Item<'all>) -> bool,
{
    type Item<'a> = I::Item<'a> where Self: 'a;

    #[inline]
    fn next(&mut self) -> Option<I::Item<'_>> {
        let flag = &mut self.flag;
        let pred = &mut self.predicate;
        self.iter.find(move |x| {
            if *flag || !pred(x) {
                *flag = true;
                true
            } else {
                false
            }
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}
