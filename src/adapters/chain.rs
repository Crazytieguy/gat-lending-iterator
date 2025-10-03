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
    type Item<'a>
        = A::Item<'a>
    where
        Self: 'a;

    #[inline]
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

#[cfg(test)]
mod tests {
    use crate::{LendingIterator, ToLendingIterator};

    fn to_vec_i32(w: &[i32]) -> Vec<i32> {
        w.to_vec()
    }

    #[test]
    fn chain_basic() {
        let result: Vec<_> = (0..3)
            .windows(2)
            .chain((5..8).windows(2))
            .map(to_vec_i32)
            .into_iter()
            .collect();
        assert_eq!(result, vec![vec![0, 1], vec![1, 2], vec![5, 6], vec![6, 7]]);
    }
}
