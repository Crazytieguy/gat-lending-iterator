use crate::LendingIterator;

/// A lending iterator that given an iterator, lends
/// references to the given iterator's items.
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct LendRefs<I: Iterator> {
    item: Option<I::Item>,
    iter: I,
}

impl<I: Iterator> LendRefs<I> {
    pub(crate) fn new(iter: I) -> LendRefs<I> {
        LendRefs { item: None, iter }
    }
}

impl<I> LendingIterator for LendRefs<I>
where
    I: Iterator,
{
    type Item<'a> = &'a I::Item where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.item = self.iter.next();
        self.item.as_ref()
    }
}
#[cfg(test)]
mod test {
    use crate::{LendingIterator, ToLendingIterator};
    #[derive(Clone, Eq, PartialEq, Debug)]
    struct Foo(usize);
    struct W {
        x: Foo,
    }
    impl LendingIterator for W {
        type Item<'a> = &'a Foo where Self: 'a;
        fn next(&mut self) -> Option<Self::Item<'_>> {
            self.x.0 += 1;
            Some(&self.x)
        }
    }
    #[test]
    fn test() {
        let mut xs = Vec::new();
        test_helper().take(3).for_each(|x: &Foo| {
            xs.push(x.clone());
        });
        assert_eq!(xs, vec![Foo(0), Foo(1), Foo(2)]);
    }

    fn test_helper() -> impl for<'a> LendingIterator<Item<'a> = &'a Foo> {
        let w = W { x: Foo(0) };
        std::iter::once(Foo(0)).lend_refs().chain(w)
    }
}
