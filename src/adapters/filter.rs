use crate::LendingIterator;

pub struct Filter<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> Filter<I, P> {
    pub(crate) fn new(iter: I, predicate: P) -> Self {
        Self { iter, predicate }
    }
}

impl<I, P> LendingIterator for Filter<I, P>
where
    I: LendingIterator,
    P: for<'a> FnMut(&I::Item<'a>) -> bool,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        loop {
            // SAFETY: see https://docs.rs/polonius-the-crab/0.3.1/polonius_the_crab/#the-arcanemagic
            let _self = unsafe { &mut *(self as *mut Self) };
            if let Some(item) = _self.iter.next() {
                if (_self.predicate)(&item) {
                    return Some(item);
                }
            } else {
                return None;
            }
        }
    }
}
