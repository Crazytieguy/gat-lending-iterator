use crate::LendingIterator;

pub struct Zip<A, B> {
    a: A,
    b: B,
}
impl<A, B> Zip<A, B> {
    pub(crate) fn new(a: A, b: B) -> Zip<A, B> {
        Zip { a, b }
    }
}

impl<A, B> LendingIterator for Zip<A, B>
where
    A: LendingIterator,
    B: LendingIterator,
{
    type Item<'a> = (A::Item<'a>, B::Item<'a>)
        where
            A: 'a, B: 'a
    ;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let a = self.a.next()?;
        let b = self.b.next()?;
        Some((a, b))
    }
}
