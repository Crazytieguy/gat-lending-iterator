use crate::LendingIterator;

pub struct StepBy<I> {
    iter: I,
    step: usize,
    first_take: bool,
}

impl<I> StepBy<I> {
    pub(crate) fn new(iter: I, step: usize) -> StepBy<I> {
        assert!(step != 0);
        StepBy {
            iter,
            step: step - 1,
            first_take: true,
        }
    }
}

impl<I> LendingIterator for StepBy<I>
where
    I: LendingIterator,
{
    type Item<'a> = I::Item<'a>
        where
            Self: 'a
    ;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.first_take {
            self.first_take = false;
            self.iter.next()
        } else {
            self.iter.nth(self.step)
        }
    }
}
