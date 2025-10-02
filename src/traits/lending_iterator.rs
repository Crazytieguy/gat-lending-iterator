use core::cmp::Ordering;
use std::{num::NonZeroUsize, ops::Deref};

use crate::{
    Chain, Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, Fuse, Inspect, Map, MapWhile, OptionTrait, Scan, SingleArgFnMut, SingleArgFnOnce, Skip, SkipWhile, StepBy, Take, TakeWhile, Zip
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
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Returns the number of items in the lending iterator.
    ///
    /// See [`Iterator::count`].
    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.fold(0, |count, _| count + 1)
    }

    /// Advances the lending iterator by `n` elements.
    ///
    /// See [`Iterator::advance_by`].
    #[inline]
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
    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item<'_>> {
        self.advance_by(n).ok()?;
        self.next()
    }

    /// Creates a lending iterator starting at the same point, but stepping by
    /// the given amount at each iteration.
    ///
    /// See [`Iterator::step_by`].
    #[inline]
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
    #[inline]
    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, n)
    }

    /// Creates a lending iterator that lends items matching a predicate.
    ///
    /// The predicate is called once for every item.
    /// Once it returns false once, `None` is returned for all subsequent calls to [`next`].
    ///
    /// [`next`]: Self::next
    #[inline]
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where
        Self: Sized,
        P: for<'a> FnMut(&Self::Item<'a>) -> bool,
    {
        TakeWhile::new(self, predicate)
    }

    /// Takes two lending iterators and creates a new lending iterator over both in sequence.
    ///
    /// See [`Iterator::chain`].
    #[inline]
    fn chain<I>(self, other: I) -> Chain<Self, I>
    where
        Self: Sized,
        for<'a> I: LendingIterator<Item<'a> = Self::Item<'a>> + 'a,
    {
        Chain::new(self, other)
    }

    /// 'Zips up' two lending iterators into a single lending iterator of pairs.
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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

    /// Creates a lending iterator which copies all of its elements.
    ///
    /// The resulting lending iterator implements [`IntoIterator`].
    ///
    /// See [`Iterator::copied`].
    #[inline]
    fn copied<T>(self) -> Copied<Self>
    where
        Self: Sized,
        for<'a> Self::Item<'a>: Deref<Target = T>,
        T: Copy,
    {
        Copied::new(self)
    }

    /// Creates a lending iterator which gives the current iteration count as well as the next value.
    #[inline]
    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate::new(self)
    }

    /// Creates a lending iterator that skips over the first `n` elements of self.
    #[inline]
    fn skip(self, n: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip::new(self, n)
    }

    /// Creates a lending iterator that rejects elements while `predicate` returns `true`.
    #[inline]
    fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
    where
        Self: Sized,
        P: for<'a> FnMut(&Self::Item<'a>) -> bool,
    {
        SkipWhile::new(self, predicate)
    }

    /// Creates a lending iterator that yields the current element unchanged,
    /// while calling a provided function with a reference to that element.
    ///
    /// See [`Iterator::inspect`].
    #[inline]
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        Self: Sized,
        F: for<'a> FnMut(&Self::Item<'a>),
    {
        Inspect::new(self, f)
    }

    /// Creates a lending iterator that yields elements transformed by a stateful closure.
    ///
    /// See [`Iterator::scan`].
    #[inline]
    fn scan<St, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
    where
        Self: Sized,
    {
        Scan::new(self, initial_state, f)
    }

    /// Creates a lending iterator that yields elements while the predicate returns `Some`.
    ///
    /// See [`Iterator::map_while`].
    #[inline]
    fn map_while<F>(self, f: F) -> MapWhile<Self, F>
    where
        Self: Sized,
        F: for<'a> SingleArgFnMut<Self::Item<'a>>,
        for<'a> <F as SingleArgFnOnce<Self::Item<'a>>>::Output: OptionTrait,
    {
        MapWhile::new(self, f)
    }

    /// Creates a lending iterator which can use the [`peek`] method
    /// to look at the next element without consuming it. Returns `None`
    /// when iteration is finished.
    ///
    /// See [`Iterator::fuse`].
    #[inline]
    fn fuse(self) -> Fuse<Self>
    where
        Self: Sized,
    {
        Fuse::new(self)
    }

    /// Creates a lending iterator that repeats elements endlessly by cloning the iterator.
    ///
    /// See [`Iterator::cycle`].
    #[inline]
    fn cycle(self) -> Cycle<Self>
    where
        Self: Sized + Clone,
    {
        Cycle::new(self)
    }

    /// Borrows the lending iterator.
    /// 
    /// This is useful to allow applying iterator adapters while still
    /// retaining ownership of the original iterator.
    /// 
    /// Unfortunately adapters that take in a closure are currently
    /// incompatible with this, due to limitations in the borrow checker.
    #[inline]
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    /// Tests if every element of the iterator matches a predicate.
    #[inline]
    fn all<P>(&mut self, mut predicate: P) -> bool
    where
        P: FnMut(Self::Item<'_>) -> bool,
    {
        while let Some(item) = self.next() {
            if !predicate(item) {
                return false;
            }
        }
        true
    }

    /// Tests if any element of the iterator matches a predicate.
    #[inline]
    fn any<P>(&mut self, mut predicate: P) -> bool
    where
        P: FnMut(Self::Item<'_>) -> bool,
    {
        while let Some(item) = self.next() {
            if predicate(item) {
                return true;
            }
        }
        false
    }

    /// Checks if the elements of this iterator are partitioned according to the given predicate,
    /// such that all those that return `true` precede all those that return `false`.
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn is_partitioned<P>(mut self, mut predicate: P) -> bool
    where
        Self: Sized,
        P: FnMut(Self::Item<'_>) -> bool,
    {
        while let Some(item) = self.next() {
            if !predicate(item) {
                break
            }
        }
        while let Some(item) = self.next() {
            if predicate(item) {
                return false
            }
        }
        true
    }

    /// Searches for an element of an iterator that satisfies a predicate.
    #[inline]
    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item<'_>>
    where
        P: FnMut(&Self::Item<'_>) -> bool,
    {
        loop {
            // SAFETY: see https://docs.rs/polonius-the-crab/0.3.1/polonius_the_crab/#the-arcanemagic
            let self_ = unsafe { &mut *(self as *mut Self) };
            if let Some(item) = self_.next() {
                if (predicate)(&item) {
                    return Some(item);
                }
            } else {
                return None;
            }
        }
    }

    /// Applies function to the elements of iterator and returns
    /// the first non-none result.
    #[inline]
    fn find_map<B, F>(&mut self, mut f: F) -> Option<B>
    where
        F: FnMut(Self::Item<'_>) -> Option<B>,
    {
        loop {
            // SAFETY: see https://docs.rs/polonius-the-crab/0.3.1/polonius_the_crab/#the-arcanemagic
            let self_ = unsafe { &mut *(self as *mut Self) };
            if let Some(item) = self_.next() {
                if let Some(result) = f(item) {
                    return Some(result);
                }
            } else {
                return None;
            }
        }
    }

    /// Searches for an element in an iterator, returning its index.
    #[inline]
    fn position<P>(&mut self, mut predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item<'_>) -> bool,
    {
        let mut i = 0;
        while let Some(item) = self.next() {
            if predicate(item) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// [Lexicographically](Ord#lexicographical-comparison) compares the elements of this [`Iterator`] with those
    /// of another.
    fn cmp<I>(mut self, mut other: I) -> Ordering
    where
        I: for<'a> LendingIterator<Item<'a> = Self::Item<'a>>,
        for<'a> Self::Item<'a>: Ord,
        Self: Sized,
    {
        // TODO: this could be implemented as `self.cmp_by(other, |x, y| x.cmp(y))`
        // except that doesn't type check due to lifetime issues.
        loop {
            match (self.next(), other.next()) {
                (Some(x), Some(y)) => match x.cmp(&y) {
                    Ordering::Equal => continue,
                    non_eq => return non_eq,
                },
                (None, None) => return Ordering::Equal,
                (None, _) => return Ordering::Less,
                (_, None) => return Ordering::Greater,
            }
        }
    }

    /// [Lexicographically](Ord#lexicographical-comparison) compares the elements of this [`Iterator`] with those
    /// of another with respect to the specified comparison function.
    fn cmp_by<I, F>(mut self, mut other: I, mut cmp: F) -> Ordering
    where
        Self: Sized,
        I: LendingIterator,
        F: FnMut(Self::Item<'_>, I::Item<'_>) -> Ordering,
    {
        loop {
            match (self.next(), other.next()) {
                (Some(x), Some(y)) => match cmp(x, y) {
                    Ordering::Equal => continue,
                    non_eq => return non_eq,
                },
                (None, None) => return Ordering::Equal,
                (None, _) => return Ordering::Less,
                (_, None) => return Ordering::Greater,
            }
        }
    }

    /// [Lexicographically](Ord#lexicographical-comparison) compares the [`PartialOrd`] elements of
    /// this [`Iterator`] with those of another. The comparison works like short-circuit
    /// evaluation, returning a result without comparing the remaining elements.
    /// As soon as an order can be determined, the evaluation stops and a result is returned.
    fn partial_cmp<I>(mut self, mut other: I) -> Option<Ordering>
    where
        I: LendingIterator,
        for<'a> Self::Item<'a>: PartialOrd<I::Item<'a>>,
        Self: Sized,
    {
        loop {
            match (self.next(), other.next()) {
                (Some(x), Some(y)) => match x.partial_cmp(&y) {
                    Some(Ordering::Equal) => continue,
                    non_eq => return non_eq,
                },
                (None, None) => return Some(Ordering::Equal),
                (None, _) => return Some(Ordering::Less),
                (_, None) => return Some(Ordering::Greater),
            }
        }
    }

    /// [Lexicographically](Ord#lexicographical-comparison) compares the elements of this [`Iterator`] with those
    /// of another with respect to the specified comparison function.
    fn partial_cmp_by<I, F>(mut self, mut other: I, mut partial_cmp: F) -> Option<Ordering>
    where
        Self: Sized,
        I: LendingIterator,
        F: FnMut(Self::Item<'_>, I::Item<'_>) -> Option<Ordering>,
    {
        loop {
            match (self.next(), other.next()) {
                (Some(x), Some(y)) => match partial_cmp(x, y) {
                    Some(Ordering::Equal) => continue,
                    non_eq => return non_eq,
                },
                (None, None) => return Some(Ordering::Equal),
                (None, _) => return Some(Ordering::Less),
                (_, None) => return Some(Ordering::Greater),
            }
        }
    }

    /// Determines if the elements of this [`Iterator`] are equal to those of
    /// another.
    fn eq<I>(mut self, mut other: I) -> bool
    where
        I: LendingIterator,
        for<'a> Self::Item<'a>: PartialEq<I::Item<'a>>,
        Self: Sized,
    {
        loop {
            match (self.next(), other.next()) {
                (Some(x), Some(y)) => if x != y {
                    return false;
                },
                (None, None) => return true,
                _ => return false,
            }
        }
    }

    /// Determines if the elements of this [`Iterator`] are equal to those of
    /// another with respect to the specified equality function.
    fn eq_by<I, F>(mut self, mut other: I, mut eq: F) -> bool
    where
        Self: Sized,
        I: LendingIterator,
        F: FnMut(Self::Item<'_>, I::Item<'_>) -> bool,
    {
        loop {
            match (self.next(), other.next()) {
                (Some(x), Some(y)) => if !eq(x, y) {
                    return false;
                },
                (None, None) => return true,
                _ => return false,
            }
        }
    }

    /// Determines if the elements of this [`Iterator`] are not equal to those of
    /// another.
    fn ne<I>(self, other: I) -> bool
    where
        I: LendingIterator,
        for<'a> Self::Item<'a>: PartialEq<I::Item<'a>>,
        Self: Sized,
    {
        !self.eq(other)
    }

    /// Determines if the elements of this [`Iterator`] are [lexicographically](Ord#lexicographical-comparison)
    /// less than those of another.
    fn lt<I>(self, other: I) -> bool
    where
        I: LendingIterator,
        for<'a> Self::Item<'a>: PartialOrd<I::Item<'a>>,
        Self: Sized,
    {
        self.partial_cmp(other) == Some(Ordering::Less)
    }

    /// Determines if the elements of this [`Iterator`] are [lexicographically](Ord#lexicographical-comparison)
    /// less or equal to those of another.
    fn le<I>(self, other: I) -> bool
    where
        I: LendingIterator,
        for<'a> Self::Item<'a>: PartialOrd<I::Item<'a>>,
        Self: Sized,
    {
        matches!(self.partial_cmp(other), Some(Ordering::Less | Ordering::Equal))
    }

    /// Determines if the elements of this [`Iterator`] are [lexicographically](Ord#lexicographical-comparison)
    /// greater than those of another.
    fn gt<I>(self, other: I) -> bool
    where
        I: LendingIterator,
        for<'a> Self::Item<'a>: PartialOrd<I::Item<'a>>,
        Self: Sized,
    {
        self.partial_cmp(other) == Some(Ordering::Greater)
    }

    /// Determines if the elements of this [`Iterator`] are [lexicographically](Ord#lexicographical-comparison)
    /// greater or equal to those of another.
    fn ge<I>(self, other: I) -> bool
    where
        I: LendingIterator,
        for<'a> Self::Item<'a>: PartialOrd<I::Item<'a>>,
        Self: Sized,
    {
        matches!(self.partial_cmp(other), Some(Ordering::Greater | Ordering::Equal))
    }

    /// Reduces the elements to a single one, by repeatedly applying a reducing
    /// operation.
    ///
    /// If the iterator is empty, returns [`None`]; otherwise, returns the
    /// result of the reduction.
    ///
    /// The reducing function takes an accumulator and an element and returns a new accumulator.
    /// For lending iterators, since items borrow from `&mut self`, the accumulator must be
    /// an owned type that doesn't borrow from the iterator.
    ///
    /// See [`Iterator::reduce`].
    #[inline]
    fn reduce<B, F>(mut self, f: F) -> Option<B>
    where
        Self: Sized,
        F: FnMut(B, Self::Item<'_>) -> B,
        B: for<'a> From<Self::Item<'a>>,
    {
        let first = B::from(self.next()?);
        Some(self.fold(first, f))
    }

    /// An iterator method that applies a fallible function to each item in the
    /// iterator, stopping at the first error and returning that error.
    ///
    /// After an error is returned, the iterator may be in an unspecified state.
    ///
    /// See [`Iterator::try_fold`].
    #[inline]
    fn try_fold<B, F, E>(&mut self, init: B, mut f: F) -> Result<B, E>
    where
        Self: Sized,
        F: FnMut(B, Self::Item<'_>) -> Result<B, E>,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x)?;
        }
        Ok(accum)
    }

    /// An iterator method that applies a fallible function to each item in the
    /// iterator, stopping at the first error and returning that error.
    ///
    /// See [`Iterator::try_for_each`].
    #[inline]
    fn try_for_each<F, E>(&mut self, mut f: F) -> Result<(), E>
    where
        Self: Sized,
        F: FnMut(Self::Item<'_>) -> Result<(), E>,
    {
        while let Some(x) = self.next() {
            f(x)?;
        }
        Ok(())
    }

    /// Applies function to the elements of iterator and returns
    /// the first true result or the first error.
    ///
    /// See [`Iterator::try_find`].
    #[inline]
    fn try_find<F, R>(&mut self, mut f: F) -> Result<Option<Self::Item<'_>>, R>
    where
        Self: Sized,
        F: FnMut(&Self::Item<'_>) -> Result<bool, R>,
    {
        loop {
            let self_ = unsafe { &mut *(self as *mut Self) };
            if let Some(item) = self_.next() {
                match f(&item) {
                    Ok(true) => return Ok(Some(item)),
                    Ok(false) => continue,
                    Err(e) => return Err(e),
                }
            } else {
                return Ok(None);
            }
        }
    }

    /// Reduces the elements to a single one by repeatedly applying a reducing operation.
    /// If the closure returns a failure, the failure is propagated back to the caller immediately.
    ///
    /// For lending iterators, the accumulator must be an owned type since items borrow from `&mut self`.
    ///
    /// See [`Iterator::try_reduce`].
    #[inline]
    fn try_reduce<B, F, E>(mut self, f: F) -> Result<Option<B>, E>
    where
        Self: Sized,
        F: FnMut(B, Self::Item<'_>) -> Result<B, E>,
        B: for<'a> From<Self::Item<'a>>,
    {
        let first = B::from(match self.next() {
            Some(i) => i,
            None => return Ok(None),
        });
        self.try_fold(first, f).map(Some)
    }

    /// Returns the maximum element of an iterator.
    ///
    /// For lending iterators, this returns an owned value converted from the item,
    /// as we need to hold onto the maximum value while iterating through borrowed items.
    ///
    /// See [`Iterator::max`].
    #[inline]
    fn max<T>(mut self) -> Option<T>
    where
        Self: Sized,
        T: Ord + for<'a> From<Self::Item<'a>>,
    {
        let first = T::from(self.next()?);
        Some(self.fold(first, |max, x| {
            let x = T::from(x);
            if x > max {
                x
            } else {
                max
            }
        }))
    }

    /// Returns the minimum element of an iterator.
    ///
    /// For lending iterators, this returns an owned value converted from the item,
    /// as we need to hold onto the minimum value while iterating through borrowed items.
    ///
    /// See [`Iterator::min`].
    #[inline]
    fn min<T>(mut self) -> Option<T>
    where
        Self: Sized,
        T: Ord + for<'a> From<Self::Item<'a>>,
    {
        let first = T::from(self.next()?);
        Some(self.fold(first, |min, x| {
            let x = T::from(x);
            if x < min {
                x
            } else {
                min
            }
        }))
    }

    /// Returns the element that gives the maximum value from the specified function.
    ///
    /// For lending iterators, this returns an owned value converted from the item.
    ///
    /// See [`Iterator::max_by`].
    #[inline]
    fn max_by<T, F>(mut self, mut compare: F) -> Option<T>
    where
        Self: Sized,
        F: FnMut(&T, &T) -> Ordering,
        T: for<'a> From<Self::Item<'a>>,
    {
        let first = T::from(self.next()?);
        Some(self.fold(first, |max, x| {
            let x = T::from(x);
            match compare(&x, &max) {
                Ordering::Greater => x,
                _ => max,
            }
        }))
    }

    /// Returns the element that gives the minimum value from the specified function.
    ///
    /// For lending iterators, this returns an owned value converted from the item.
    ///
    /// See [`Iterator::min_by`].
    #[inline]
    fn min_by<T, F>(mut self, mut compare: F) -> Option<T>
    where
        Self: Sized,
        F: FnMut(&T, &T) -> Ordering,
        T: for<'a> From<Self::Item<'a>>,
    {
        let first = T::from(self.next()?);
        Some(self.fold(first, |min, x| {
            let x = T::from(x);
            match compare(&x, &min) {
                Ordering::Less => x,
                _ => min,
            }
        }))
    }

    /// Returns the element that gives the maximum value with respect to the
    /// specified key function.
    ///
    /// For lending iterators, this returns an owned value converted from the item.
    ///
    /// See [`Iterator::max_by_key`].
    #[inline]
    fn max_by_key<T, B, F>(mut self, mut f: F) -> Option<T>
    where
        Self: Sized,
        B: Ord,
        F: FnMut(&T) -> B,
        T: for<'a> From<Self::Item<'a>>,
    {
        let first = T::from(self.next()?);
        let first_key = f(&first);
        Some(self.fold((first, first_key), |(max, max_key), x| {
            let x = T::from(x);
            let x_key = f(&x);
            if x_key > max_key {
                (x, x_key)
            } else {
                (max, max_key)
            }
        }).0)
    }

    /// Returns the element that gives the minimum value with respect to the
    /// specified key function.
    ///
    /// For lending iterators, this returns an owned value converted from the item.
    ///
    /// See [`Iterator::min_by_key`].
    #[inline]
    fn min_by_key<T, B, F>(mut self, mut f: F) -> Option<T>
    where
        Self: Sized,
        B: Ord,
        F: FnMut(&T) -> B,
        T: for<'a> From<Self::Item<'a>>,
    {
        let first = T::from(self.next()?);
        let first_key = f(&first);
        Some(self.fold((first, first_key), |(min, min_key), x| {
            let x = T::from(x);
            let x_key = f(&x);
            if x_key < min_key {
                (x, x_key)
            } else {
                (min, min_key)
            }
        }).0)
    }

    /// Sums the elements of an iterator.
    ///
    /// Takes each element, converts it to the sum type, and adds them together.
    ///
    /// An empty iterator returns the zero value of the type.
    ///
    /// For lending iterators, items must be convertible to the sum type.
    ///
    /// See [`Iterator::sum`].
    #[inline]
    fn sum<S>(self) -> S
    where
        Self: Sized,
        for<'a> S: std::ops::AddAssign<Self::Item<'a>> + Default,
    {
        let mut sum = S::default();
        self.for_each(|item| sum += item);
        sum
    }

    /// Iterates over the entire iterator, multiplying all the elements
    ///
    /// An empty iterator returns the one value of the type.
    ///
    /// For lending iterators, items must be multiplicable with the product type.
    ///
    /// See [`Iterator::product`].
    #[inline]
    fn product<P>(self) -> P
    where
        Self: Sized,
        for<'a> P: std::ops::MulAssign<Self::Item<'a>>,
        P: From<u8>,
    {
        let mut product = P::from(1u8);
        self.for_each(|item| product *= item);
        product
    }
}

impl<T: LendingIterator> LendingIterator for &mut T {
    type Item<'a> = T::Item<'a> where Self: 'a;

    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        (**self).next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
}
