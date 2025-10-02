use crate::LendingIterator;

/// A Lending iterator that only lends the first `n` iterations of `iter`.
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Take<I> {
    iter: I,
    n: usize,
}

impl<I> Take<I>
where
    I: LendingIterator,
{
    pub(crate) fn new(iter: I, n: usize) -> Take<I> {
        Take { iter, n }
    }
}

impl<I> LendingIterator for Take<I>
where
    I: LendingIterator,
{
    type Item<'a> = I::Item<'a> where I: 'a;

    #[inline]
    #[allow(clippy::if_not_else)]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.n != 0 {
            self.n -= 1;
            self.iter.next()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{LendingIterator, ToLendingIterator};

    fn identity(x: i32) -> i32 {
        x
    }

    #[test]
    fn test() {
        assert_eq!(
            std::iter::repeat(())
                .into_lending()
                .take(5)
                .fold(0, |count, ()| { count + 1 }),
            5
        );
    }

    #[test]
    fn take_basic() {
        let result: Vec<_> = (0..10)
            .into_lending()
            .take(3)
            .map(identity)
            .into_iter()
            .collect();
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn take_more_than_available() {
        let result: Vec<_> = (0..3)
            .into_lending()
            .take(10)
            .map(identity)
            .into_iter()
            .collect();
        assert_eq!(result, vec![0, 1, 2]);
    }
}
