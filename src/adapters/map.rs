use crate::{LendingIterator, SingleArgFnMut, SingleArgFnOnce};

pub struct Map<I, F> {
    iter: I,
    f: F,
}

impl<I, F> Map<I, F> {
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I, F> LendingIterator for Map<I, F>
where
    I: LendingIterator,
    F: for<'a> SingleArgFnMut<I::Item<'a>>,
{
    type Item<'a> = <F as SingleArgFnOnce<I::Item<'a>>>::Output
        where
            Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|item| self.f.call_mut(item))
    }
}

pub struct MapIntoIter<I, F> {
    iter: I,
    f: F,
}

impl<I, F, O> Iterator for MapIntoIter<I, F>
where
    I: LendingIterator,
    F: FnMut(I::Item<'_>) -> O,
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.f)
    }
}

impl<I, F, O> IntoIterator for Map<I, F>
where
    I: LendingIterator,
    F: FnMut(I::Item<'_>) -> O,
{
    type Item = O;
    type IntoIter = MapIntoIter<I, F>;

    fn into_iter(self) -> Self::IntoIter {
        MapIntoIter {
            iter: self.iter,
            f: self.f,
        }
    }
}
