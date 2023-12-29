use crate::LendingIterator;

/// A lending iterator that iterates over the elements of two iterators
/// in sequence.
///
/// This `struct` is created by the [`chain`] method on [`LendingIterator`]. See
/// its documentation for more.
///
/// [`LendingIterator`]: crate::LendingIterator
/// [`chain`]: crate::LendingIterator::chain
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Chain<A, B> {
    a: A,
    b: B,
    a_done: bool,
}
impl<A, B> Chain<A, B> {
    pub(crate) fn new(a: A, b: B) -> Chain<A, B> {
        Chain {
            a,
            b,
            a_done: false,
        }
    }
}

impl<A, B> LendingIterator for Chain<A, B>
where
    A: LendingIterator,
    for<'a> B: LendingIterator<Item<'a> = A::Item<'a>> + 'a,
{
    type Item<'a> = A::Item<'a>
        where
            Self: 'a
    ;

    fn next(&mut self) -> Option<A::Item<'_>> {
        if self.a_done {
            self.b.next()
        } else {
            self.a.next().or_else(|| {
                self.a_done = true;
                self.b.next()
            })
        }
    }
}
