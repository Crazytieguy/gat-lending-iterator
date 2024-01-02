use core::fmt;

use stable_try_trait_v2::{internal::NeverShortCircuit, try_, Try};

use crate::{ExactSizeLendingIterator, LendingIterator, TrustedLen};

// REVIEW: There are definitely ways to induce race conditions and such, but is there a way to induce UB with peekable in safe code?
// because we control all access to self.iter,
// the only contexts which may be of concern are after a `peek`-and-forget:
// ```
// let mut peekable = iter.peekable();
// let _ = peekable.peek(); // load the next item into peekable.peeked
// // any method called on peekable is safe, per manual inspection,
// // ... but some code that may or may not induce UB ...
// peekable.next(); // consume the item that was peeked after arbitrary code
// ```
// It's probably pretty easy to induce UB when using unsafe code either in the inner iter or outside as well, but I'm not going to consider that here.
// If so, we can always restrict to an unsafe trait.

/// A lending iterator with a `peek()` that returns an optional reference to the next
/// element.
///
/// This `struct` is created by the [`peekable`] method on [`LendingIterator`]. See its
/// documentation for more.
///
/// [`peekable`]: LendingIterator::peekable
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Peekable<'this, I: 'this>
where
    I: LendingIterator,
{
    iter: I,
    peeked: Option<Option<I::Item<'this>>>,
}
impl<'this, I> Peekable<'this, I>
where
    I: LendingIterator,
{
    pub(crate) fn new(iter: I) -> Peekable<'this, I> {
        Peekable { iter, peeked: None }
    }
    /// Returns a reference to the next() value without advancing the iterator.
    // REVIEW: is there a way to abuse &I::Item<'this> in safe code?
    pub fn peek(&mut self) -> Option<&'_ I::Item<'this>> {
        self.peeked
            .get_or_insert_with(|| unsafe {
                // SAFETY: We manually guarantee iter.next() is only called once per item
                core::mem::transmute::<Option<I::Item<'_>>, Option<I::Item<'this>>>(
                    self.iter.next(),
                )
            })
            .as_ref()
    }
    /// Returns a mutable reference to the next() value without advancing the iterator.
    pub fn peek_mut(&mut self) -> Option<&'_ mut I::Item<'this>> {
        self.peeked
            .get_or_insert_with(|| unsafe {
                // SAFETY: We manually guarantee iter.next() is only called once per item
                core::mem::transmute::<Option<I::Item<'_>>, Option<I::Item<'this>>>(
                    self.iter.next(),
                )
            })
            .as_mut()
    }
    /// Consume and return the next value of this iterator if a condition is true.
    pub fn next_if<'a, F>(&'a mut self, f: F) -> Option<I::Item<'a>>
    where
        F: FnOnce(&I::Item<'a>) -> bool,
    {
        let peeked = unsafe { &mut *(&mut self.peeked as *mut _) };
        match self.next() {
            Some(v) if f(&v) => Some(v),
            v => {
                // SAFETY: We manually guarantee iter.next() is only called once per item
                *peeked = Some(unsafe {
                    core::mem::transmute::<Option<I::Item<'_>>, Option<I::Item<'this>>>(v)
                });
                None
            }
        }
    }
    /// Consume and return the next item if it is equal to `expected`.
    pub fn next_if_eq<'a, T>(&'a mut self, t: &T) -> Option<I::Item<'a>>
    where
        T: PartialEq<I::Item<'a>>,
    {
        self.next_if(|v| t == v)
    }
}
impl<'this, I: fmt::Debug> fmt::Debug for Peekable<'this, I>
where
    I: LendingIterator + fmt::Debug,
    I::Item<'this>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Peekable")
            .field("lender", &self.iter)
            .field("peeked", &self.peeked)
            .finish()
    }
}

impl<'this, I> LendingIterator for Peekable<'this, I>
where
    I: LendingIterator,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;
    fn next(&mut self) -> Option<I::Item<'_>> {
        match self.peeked.take() {
            // SAFETY: 'this: 'call
            Some(peeked) => unsafe {
                core::mem::transmute::<Option<I::Item<'this>>, Option<I::Item<'_>>>(peeked)
            },
            None => self.iter.next(),
        }
    }
    #[inline]
    fn count(mut self) -> usize {
        match self.peeked.take() {
            Some(None) => 0,
            Some(Some(_)) => 1 + self.iter.count(),
            None => self.iter.count(),
        }
    }
    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item<'_>> {
        match self.peeked.take() {
            Some(None) => None,
            // SAFETY: 'this: 'call
            Some(v @ Some(_)) if n == 0 => unsafe {
                core::mem::transmute::<Option<I::Item<'this>>, Option<I::Item<'_>>>(v)
            },
            Some(Some(_)) => self.iter.nth(n - 1),
            None => self.iter.nth(n),
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.peeked {
            Some(None) => (0, Some(0)),
            Some(Some(_)) => (1, None),
            None => self.iter.size_hint(),
        }
    }
    // #[inline]
    // fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    // where
    //     Self: Sized,
    //     F: FnMut(B, Self::Item<'_>) -> R,
    //     R: Try<Output = B>,
    // {
    //     let acc = match self.peeked.take() {
    //         Some(None) => return Try::from_output(init),
    //         Some(Some(v)) => try_!(f(init, v)),
    //         None => init,
    //     };
    //     self.iter.try_fold(acc, f)
    // }
    #[inline]
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, I::Item<'_>) -> B,
    {
        let acc = match self.peeked.take() {
            Some(None) => return init,
            Some(Some(v)) => f(init, v),
            None => init,
        };
        self.iter.fold(acc, f)
    }
}
