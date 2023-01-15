use std::ops::Deref;

use crate::{Chain, Cloned, Filter, Map, SingleArgFnMut, StepBy};

pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.fold(0, |count, _| count + 1)
    }

    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        for i in 0..n {
            self.next().ok_or(i)?;
        }
        Ok(())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item<'_>> {
        self.advance_by(n).ok()?;
        self.next()
    }

    fn step_by(self, step: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        StepBy::new(self, step)
    }

    fn chain<I>(self, other: I) -> Chain<Self, I>
    where
        Self: Sized,
        for<'a> I: LendingIterator<Item<'a> = Self::Item<'a>> + 'a,
    {
        Chain::new(self, other)
    }

    fn map<F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: for<'a> SingleArgFnMut<Self::Item<'a>>,
    {
        Map::new(self, f)
    }

    fn for_each<F>(mut self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item<'_>),
    {
        while let Some(item) = self.next() {
            f(item);
        }
    }

    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: for<'a> FnMut(&Self::Item<'a>) -> bool,
    {
        Filter::new(self, predicate)
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item<'_>) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }

    fn cloned<T>(self) -> Cloned<Self>
    where
        Self: Sized,
        for<'a> Self::Item<'a>: Deref<Target = T>,
        T: Clone,
    {
        Cloned::new(self)
    }
}
