use crate::LendingIterator;

/// A lending iterator that given an iterator, lends
/// mutable references to the given iterator's items.
#[derive(Clone)]
pub struct LendRefsMut<I: Iterator> {
    item: Option<I::Item>,
    iter: I,
}

impl<I: Iterator> LendRefsMut<I> {
    pub(crate) fn new(iter: I) -> LendRefsMut<I> {
        LendRefsMut { item: None, iter }
    }
}

impl<I: Iterator> LendingIterator for LendRefsMut<I> {
    type Item<'a> = &'a mut I::Item where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.item = self.iter.next();
        self.item.as_mut()
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
        type Item<'a> = &'a mut Foo where Self: 'a;
        fn next(&mut self) -> Option<Self::Item<'_>> {
            self.x.0 += 1;
            Some(&mut self.x)
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test() {
        let mut xs = Vec::new();
        test_helper().take(3).for_each(|x: &mut Foo| {
            x.0 += 2;
            xs.push(x.clone());
        });
        assert_eq!(xs, vec![Foo(2), Foo(3), Foo(6)]);
    }

    fn test_helper() -> impl for<'a> LendingIterator<Item<'a> = &'a mut Foo> {
        let w = W { x: Foo(0) };
        core::iter::once(Foo(0)).lend_refs_mut().chain(w)
    }
}
