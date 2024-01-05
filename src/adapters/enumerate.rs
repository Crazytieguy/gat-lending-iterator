use crate::LendingIterator;

/// A lending iterator that yields the current count and the element during iteration.
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Enumerate<I> {
    count: usize,
    iter: I,
}

impl<I> Enumerate<I> {
    pub(crate) fn new(iter: I) -> Self {
        Enumerate { iter, count: 0 }
    }
}

impl<I: LendingIterator> LendingIterator for Enumerate<I> {
    type Item<'a> = (usize, I::Item<'a>) where Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>> {
        let item = self.iter.next()?;
        let count = self.count;
        self.count += 1;
        Some((count, item))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ToLendingIterator;

    // A non-fused iterator for testing that we match std's behavior
    struct Delay<I> {
        countdown: usize,
        iter: I,
    }

    impl<I> Delay<I> {
        fn new(countdown: usize, iter: I) -> Self {
            Delay { countdown, iter }
        }
    }

    // Generally we avoid implementing both Iterator and LendingIterator
    // for the same type. Here in testing the bounds of the arguments
    // are known not to collide.
    impl<I: LendingIterator> LendingIterator for Delay<I> {
        type Item<'a> = I::Item<'a> where Self: 'a;
        fn next(&mut self) -> Option<Self::Item<'_>> {
            if self.countdown == 0 {
                self.iter.next()
            } else {
                self.countdown -= 1;
                None
            }
        }
    }

    impl<I: Iterator> Iterator for Delay<I> {
        type Item = I::Item;
        fn next(&mut self) -> Option<Self::Item> {
            if self.countdown == 0 {
                self.iter.next()
            } else {
                self.countdown -= 1;
                None
            }
        }
    }

    #[test]
    fn test() {
        let first = Some((0, ()));
        let second = Some((1, ()));
        let mut delay_iter = Delay::new(1, core::iter::repeat(()).take(2)).enumerate();
        let mut delay_lending =
            Delay::new(1, core::iter::repeat(()).into_lending().take(2)).enumerate();

        assert_eq!((None, None), (delay_iter.next(), delay_lending.next()));
        assert_eq!((first, first), (delay_iter.next(), delay_lending.next()));
        assert_eq!((second, second), (delay_iter.next(), delay_lending.next()));
        assert_eq!((None, None), (delay_iter.next(), delay_lending.next()));
    }
}
