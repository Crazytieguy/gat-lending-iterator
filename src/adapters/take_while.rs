use crate::LendingIterator;

/// A lending iterator that yields items based on a predicate.
///
/// This iterator is fused.
#[derive(Clone)]
pub struct TakeWhile<I, P> {
    iter: I,
    predicate: P,
    done: bool,
}

impl<I, P> TakeWhile<I, P> {
    pub(crate) fn new(iter: I, predicate: P) -> Self {
        Self {
            iter,
            predicate,
            done: false,
        }
    }
}

impl<I, P> LendingIterator for TakeWhile<I, P>
where
    I: LendingIterator,
    P: for<'a> FnMut(&I::Item<'a>) -> bool
{
    type Item<'a> = I::Item<'a> where I: 'a, P: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.done {
            None
        } else {
            let item = self.iter.next()?;
            if (self.predicate)(&item) {
                Some(item)
            } else {
                self.done = true;
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.done {
            (0, Some(0))
        } else {
            (0, self.iter.size_hint().1)
        }
    }
}
