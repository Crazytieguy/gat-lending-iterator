use crate::LendingIterator;

/// A lending iterator that skips over the first `n` items of `iter`.
#[derive(Clone)]
pub struct Skip<I> {
    n: usize,
    iter: I,
}

impl<I> Skip<I> {
    pub(crate) fn new(iter: I, n: usize) -> Skip<I> {
        Skip { iter, n }
    }
}

impl<I> LendingIterator for Skip<I>
where
    I: LendingIterator,
{
    type Item<'a> = I::Item<'a> where I: 'a;

    fn next(&mut self) -> Option<I::Item<'_>> {
        if self.n > 0 {
            self.iter.nth(core::mem::take(&mut self.n))
        } else {
            self.iter.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();

        let lower = lower.saturating_sub(self.n);
        let upper = upper.map(|x| x.saturating_sub(self.n));
        (lower, upper)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ToLendingIterator;
    #[test]
    fn test() {
        assert_eq!((0..5).into_lending().skip(1).nth(1), (0..5).skip(1).nth(1))
    }
}
