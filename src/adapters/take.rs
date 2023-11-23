use crate::LendingIterator;

/// A Lending iterator that only lends the first `n` iterations of `iter`.
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
    use super::*;
    use crate::ToLendingIterator;
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
}
