use ::core::{num::NonZeroUsize, ops::Deref};
use core::{mem::transmute, ops::ControlFlow};

use stable_try_trait_v2::{try_, ChangeOutputType, FromResidual, Residual, Try};

use crate::{
    Chain, Cloned, Enumerate, ExactSizeLendingIterator, Filter, FilterMap, Map, OptionTrait,
    Peekable, SingleArgFnMut, SingleArgFnOnce, Skip, StepBy, Take, TakeWhile, Zip,
};

/// Like [`Iterator`], but items may borrow from `&mut self`.
///
/// This means that the compiler will check that you finish using an item
/// before requesting the next item, as it's not allowed for two `&mut self` to exist
/// at the same time.
#[must_use = "iterators are lazy and do nothing unless consumed"]
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

    /// Consumes the lending iterator, returning the last element that can be predicted.
    ///
    /// Relies on [`size_hint`](LendingIterator::size_hint) to determine the number of elements to skip.
    ///
    /// See [`Iterator::last`].
    #[inline]
    fn last(&mut self) -> Option<Self::Item<'_>> {
        match self.size_hint().1 {
            Some(n) => self.nth(n.saturating_sub(1)),
            None => None,
        }
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
        for<'all> P: FnMut(&Self::Item<'all>) -> bool,
    {
        TakeWhile::new(self, predicate)
    }

    /// Takes two lending iterators and creates a new lending iterator over both in sequence.
    ///
    /// See [`Iterator::chain`].
    #[inline]
    fn chain<'a, I>(self, other: I) -> Chain<Self, I>
    where
        Self: 'a + Sized,
        I: 'a + LendingIterator<Item<'a> = Self::Item<'a>>,
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
        for<'all> F: SingleArgFnMut<Self::Item<'all>>,
    {
        Map::new(self, f)
    }

    /// Borrows an iterator, rather than consuming it.
    ///
    /// See [`Iterator::by_ref`].
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    /// A lending iterator method that applies a fallible function to each item in the iterator, stopping at the first error and returning that error.
    ///
    /// See [`Iterator::try_for_each`].
    #[inline]
    fn try_for_each<F, R>(&mut self, mut f: F) -> R
    where
        Self: Sized,
        for<'all> F: FnMut(Self::Item<'all>) -> R,
        R: Try<Output = ()>,
    {
        self.try_fold(
            (),
            #[inline]
            move |(), x| f(x),
        )
    }

    /// Calls a closure on each element of the lending iterator.
    ///
    /// See [`Iterator::for_each`].
    #[inline]
    fn for_each<F>(self, mut f: F)
    where
        Self: Sized,
        for<'all> F: FnMut(Self::Item<'all>),
    {
        self.fold(
            (),
            #[inline]
            move |(), item| f(item),
        );
    }

    /// Creates a lending iterator which uses a closure to determine if an element
    /// should be yielded.
    ///
    /// See [`Iterator::filter`].
    #[inline]
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        for<'all> P: FnMut(&Self::Item<'all>) -> bool,
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
        for<'all> F: SingleArgFnMut<Self::Item<'all>>,
        for<'all> <F as SingleArgFnOnce<Self::Item<'all>>>::Output: OptionTrait,
    {
        FilterMap::new(self, f)
    }

    /// A lending iterator method that applies a function as long as it returns successfully, producing a single, final value.
    ///
    /// See [`Iterator::try_fold`].
    #[inline]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        Self: Sized,
        for<'all> F: FnMut(B, Self::Item<'all>) -> R,
        R: Try<Output = B>,
    {
        let mut acc = init;
        while let Some(x) = self.next() {
            acc = try_!(f(acc, x));
        }
        Try::from_output(acc)
    }

    /// Folds every element into an accumulator by applying an operation,
    /// returning the final result.
    ///
    /// See [`Iterator::fold`].
    #[inline]
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        for<'all> F: FnMut(B, Self::Item<'all>) -> B,
    {
        let mut acc = init;
        while let Some(x) = self.next() {
            acc = f(acc, x);
        }
        acc
    }

    /// Creates a lending iterator which [`clone`]s all of its elements.
    ///
    /// The resulting lending iterator implements [`IntoIterator`].
    ///
    /// See [`Iterator::cloned`].
    ///
    /// [`clone`]: Clone::clone
    #[inline]
    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: Sized + 'a,
        Self::Item<'a>: Deref<Target = T>,
        T: Clone,
    {
        Cloned::new(self)
    }

    /// Creates a lending iterator which gives the current iteration count as well as the next value.
    #[inline]
    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate::new(self)
    }

    /// Creates an iterator which can use the [`peek`] and [`peek_mut`] methods
    /// to look at the next element of the iterator without consuming it.
    ///
    /// see [`Iterator::peekable`].
    ///
    /// [`peek`]: Peekable::peek
    /// [`peek_mut`]: Peekable::peek_mut
    #[inline]
    fn peekable<'a>(self) -> Peekable<'a, Self>
    where
        Self: Sized,
    {
        Peekable::new(self)
    }

    /// Creates a lending iterator that skips over the first `n` elements of self.
    #[inline]
    fn skip(self, n: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip::new(self, n)
    }

    /// Creates an iterator that [`skip`]s elements based on a predicate.
    ///
    /// see [`Iterator::skip_while`].
    ///
    /// [`skip`]: Iterator::skip
    // #[inline]
    // #[doc(alias = "drop_while")]
    // fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
    // where
    //     Self: Sized,
    //     for<'all> P: FnMut(&Self::Item<'all>) -> bool,
    // {
    //     SkipWhile::new(self, predicate)
    // }

    /// Searches for an element of an iterator that satisfies a predicate.
    ///
    /// see [`Iterator::find`]
    #[inline]
    fn find<'a, P>(&'a mut self, mut predicate: P) -> Option<Self::Item<'a>>
    where
        Self: Sized,
        for<'all> P: FnMut(&Self::Item<'all>) -> bool,
    {
        while let Some(x) = self.next() {
            if predicate(&x) {
                // SAFETY: `x` is the last value yielded by `self`. polonious return
                return Some(unsafe { transmute::<Self::Item<'_>, Self::Item<'a>>(x) });
            }
        }
        None
    }

    /// Applies function to the elements of iterator and returns
    /// the first non-none result.
    ///
    /// see [`Iterator::find_map`]
    #[inline]
    fn find_map<B, F>(&mut self, mut f: F) -> Option<B>
    where
        Self: Sized,
        for<'all> F: FnMut(Self::Item<'all>) -> Option<B>,
    {
        while let Some(x) = self.next() {
            if let Some(x) = f(x) {
                return Some(x);
            }
        }
        None
    }

    /// Applies function to the elements of iterator and returns
    /// the first true result or the first error.
    ///
    /// see [`Iterator::try_find`]
    #[inline]
    fn try_find<F, R>(&mut self, mut f: F) -> ChangeOutputType<R, Option<Self::Item<'_>>>
    where
        Self: Sized,
        F: for<'all> FnMut(&Self::Item<'all>) -> R,
        R: Try<Output = bool>,
        for<'all> R::Residual: Residual<Option<Self::Item<'all>>>,
    {
        // SAFETY: `self` is not used after early return. polonious return
        while let Some(x) = unsafe { &mut *(self as *mut Self) }.next() {
            match f(&x).branch() {
                ControlFlow::Continue(false) => (),
                ControlFlow::Continue(true) => return Try::from_output(Some(x)),
                ControlFlow::Break(r) => return FromResidual::from_residual(r),
            }
        }
        Try::from_output(None)
    }
}

impl<I: LendingIterator + ?Sized> LendingIterator for &mut I {
    type Item<'a> = I::Item<'a> where Self: 'a;
    #[inline]
    fn next(&mut self) -> Option<Self::Item<'_>> {
        (**self).next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
    #[inline]
    fn advance_by(&mut self, n: usize) -> Result<(), NonZeroUsize> {
        (**self).advance_by(n)
    }
    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item<'_>> {
        (**self).nth(n)
    }
}
