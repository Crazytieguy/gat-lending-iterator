use std::{num::NonZeroUsize, ops::Deref};

use crate::{
    Chain, Cloned, Enumerate, Filter, FilterMap, Map, OptionTrait, SingleArgFnMut, SingleArgFnOnce,
    StepBy, Take, Zip,
};

/// Like [`Iterator`], but items may borrow from `&mut self`.
///
/// This means that the compiler will check that you finish using an item
/// before requesting the next item, as it's not allowed for two `&mut self` to exist
/// at the same time.
pub trait LendingIterator {
    /// The type of the elements being iterated over.
    type Item<'a>
    where
        Self: 'a;

    /// Advances the lending iterator and returns the next value.
    ///
    /// See [`Iterator::next`].
    fn next(&mut self) -> Option<Self::Item<'_>>;

    /// Returns the bounds on the remaining length of the iterator.
    ///
    /// See [`Iterator::size_hint`].
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Returns the number of items in the lending iterator.
    ///
    /// See [`Iterator::count`].
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.fold(0, |count, _| count + 1)
    }

    /// Advances the lending iterator by `n` elements.
    ///
    /// See [`Iterator::advance_by`].
    #[allow(clippy::missing_errors_doc)]
    fn advance_by(&mut self, n: usize) -> Result<(), NonZeroUsize> {
        for i in 0..n {
            if self.next().is_none() {
                // SAFETY: `i` is always less than `n`.
                return Err(unsafe { NonZeroUsize::new_unchecked(n - i) });
            }
        }
        Ok(())
    }

    /// Returns the `n`th element of the lending iterator.
    ///
    /// See [`Iterator::nth`].
    fn nth(&mut self, n: usize) -> Option<Self::Item<'_>> {
        self.advance_by(n).ok()?;
        self.next()
    }

    /// Creates a lending iterator starting at the same point, but stepping by
    /// the given amount at each iteration.
    ///
    /// See [`Iterator::step_by`].
    fn step_by(self, step: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        StepBy::new(self, step)
    }

    /// Creates a lending iterator that lends the first `n` elements, or fewer
    /// if the underlying iterator ends sooner.
    ///
    /// See [`Iterator::take`].
    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, n)
    }

    /// Takes two lending iterators and creates a new lending iterator over both in sequence.
    ///
    /// See [`Iterator::chain`].
    fn chain<I>(self, other: I) -> Chain<Self, I>
    where
        Self: Sized,
        for<'a> I: LendingIterator<Item<'a> = Self::Item<'a>> + 'a,
    {
        Chain::new(self, other)
    }

    /// 'Zips up' two lending iterators into a single lending iterator of pairs.
    fn zip<I>(self, other: I) -> Zip<Self, I>
    where
        Self: Sized,
        I: LendingIterator,
    {
        Zip::new(self, other)
    }

    /// Takes a closure and creates a lending iterator which calls that closure on each
    /// element.
    ///
    /// As of writing, in stable rust it's not possible to create a closure
    /// where the lifetime of its output is tied to its input.
    /// If you're on nightly, you can use the unstable
    /// `closure_lifetime_binder` feature. If you're on stable, try using
    /// a function.
    ///
    /// In the case that the closure's return type doesn't borrow from its input,
    /// the resulting `LendingIterator` will implement [`IntoIterator`].
    ///
    /// See [`Iterator::map`].
    fn map<F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: for<'a> SingleArgFnMut<Self::Item<'a>>,
    {
        Map::new(self, f)
    }

    /// Calls a closure on each element of the lending iterator.
    ///
    /// See [`Iterator::for_each`].
    fn for_each<F>(mut self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item<'_>),
    {
        while let Some(item) = self.next() {
            f(item);
        }
    }

    /// Creates a lending iterator which uses a closure to determine if an element
    /// should be yielded.
    ///
    /// See [`Iterator::filter`].
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: for<'a> FnMut(&Self::Item<'a>) -> bool,
    {
        Filter::new(self, predicate)
    }

    /// Creates a lending iterator that both filters and maps.
    ///
    /// See [`Iterator::filter_map`].
    fn filter_map<F>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
        F: for<'a> SingleArgFnMut<Self::Item<'a>>,
        for<'a> <F as SingleArgFnOnce<Self::Item<'a>>>::Output: OptionTrait,
    {
        FilterMap::new(self, f)
    }

    /// Folds every element into an accumulator by applying an operation,
    /// returning the final result.
    ///
    /// See [`Iterator::fold`].
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

    /// Creates a lending iterator which [`clone`]s all of its elements.
    ///
    /// The resulting lending iterator implements [`IntoIterator`].
    ///
    /// See [`Iterator::cloned`].
    ///
    /// [`clone`]: Clone::clone
    fn cloned<T>(self) -> Cloned<Self>
    where
        Self: Sized,
        for<'a> Self::Item<'a>: Deref<Target = T>,
        T: Clone,
    {
        Cloned::new(self)
    }

    /// Creates a lending iterator which gives the current iteration count as well as the next value.
    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate::new(self)
    }
}
