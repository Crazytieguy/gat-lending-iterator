use core::{fmt, marker::PhantomPinned, pin::Pin};

use crate::LendingIterator;

// REVIEW: I assumed the worst possible case is self-referencing, so I used Pin, but I'm unsure if that's really the case

/// A lending iterator with a `peek()` that returns an optional reference to the next
/// element. requires pinning to peek.
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
    #[allow(clippy::option_option)]
    peeked: Option<Option<I::Item<'this>>>,
    _pin: PhantomPinned,
}

impl<'this, I> Peekable<'this, I>
where
    I: LendingIterator,
{
    pub(crate) fn new(iter: I) -> Peekable<'this, I> {
        Peekable {
            iter,
            peeked: None,
            _pin: PhantomPinned,
        }
    }
    #[inline]
    pub(crate) fn get_peeked(self: Pin<&mut Self>) -> &mut Option<I::Item<'this>> {
        // SAFETY: we can return a mutable reference to peeked because any self-referencing to self.iter is pinned
        unsafe {
            // SAFETY: mutable references to self or fields do not move self
            let this = self.get_unchecked_mut();
            let iter = &mut this.iter;
            this.peeked
                .get_or_insert_with(#[inline] ||
                    // SAFETY: We manually guarantee iter.next() is only called once per item, and we are pinning any possible self-referencing
                    core::mem::transmute::<Option<I::Item<'_>>, Option<I::Item<'this>>>(iter.next())
                )
        }
    }
    /// Returns a reference to the next() value without advancing the iterator.
    #[inline]
    pub fn peek(self: Pin<&mut Self>) -> Option<&I::Item<'this>> {
        self.get_peeked().as_ref()
    }
    /// Returns a mutable reference to the next() value without advancing the iterator.
    #[inline]
    pub fn peek_mut(self: Pin<&mut Self>) -> Option<&mut I::Item<'this>> {
        self.get_peeked().as_mut()
    }
    /// Consume and return the next value of this iterator if a condition is true.
    pub fn next_if<F>(self: Pin<&mut Self>, f: F) -> Option<I::Item<'_>>
    where
        F: FnOnce(&I::Item<'this>) -> bool,
    {
        // SAFETY: we use pin for self-referencing by peeked, so returning peeked is safe as long as we don't move self
        unsafe {
            let this = self.get_unchecked_mut();
            let iter = &mut this.iter;
            match &this.peeked {
                Some(Some(v)) if f(v) => (),
                None => return iter.next(),
                _ => return None,
            }
            // SAFETY: 'this: 'a
            core::mem::transmute::<Option<I::Item<'this>>, Option<I::Item<'_>>>(
                this.peeked.take().unwrap_unchecked(),
            )
        }
    }
    /// Consume and return the next item if it is equal to `expected`.
    pub fn next_if_eq<'a, T>(self: Pin<&'a mut Self>, t: &T) -> Option<I::Item<'a>>
    where
        T: PartialEq<I::Item<'this>>,
    {
        self.next_if(|v| t == v)
    }
    /// Drop any peeked value and unpin the iterator.
    #[inline]
    pub fn unpin(self: Pin<&mut Self>) -> &mut Self {
        // SAFETY: we remove any possible self-referencing
        unsafe {
            let this = self.get_unchecked_mut();
            this.peeked = None;
            this
        }
    }
}

impl<'this, I: fmt::Debug> fmt::Debug for Peekable<'this, I>
where
    I: LendingIterator + fmt::Debug,
    I::Item<'this>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Peekable")
            .field("iter", &self.iter)
            .field("peeked", &self.peeked)
            .finish()
    }
}

// using deref instead of implementing LendingIterator directly
// allows us to propogate the specialized implementations of the underlying iterator
// (self.peeked is always None when self is not Pinned)
impl<'this, I> ::core::ops::Deref for Peekable<'this, I>
where
    I: LendingIterator,
{
    type Target = I;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.iter
    }
}
impl<'this, I> ::core::ops::DerefMut for Peekable<'this, I>
where
    I: LendingIterator,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.iter
    }
}
impl<'this, I> ::core::convert::AsRef<I> for Peekable<'this, I>
where
    I: LendingIterator,
{
    #[inline]
    fn as_ref(&self) -> &I {
        &self.iter
    }
}
impl<'this, I> ::core::convert::AsMut<I> for Peekable<'this, I>
where
    I: LendingIterator,
{
    #[inline]
    fn as_mut(&mut self) -> &mut I {
        &mut self.iter
    }
}

impl<'this, I> LendingIterator for Pin<&mut Peekable<'this, I>>
where
    I: LendingIterator,
{
    type Item<'a> = I::Item<'a>
    where
        Self: 'a;
    fn next<'a>(&'a mut self) -> Option<I::Item<'a>> {
        // SAFETY: we remove any self-referencing, we are just incurring a double pointer indirection
        unsafe {
            let this = self.as_mut().get_unchecked_mut();
            let iter = &mut this.iter;
            match this.peeked.take() {
                // SAFETY: 'this: 'a
                Some(peeked) => {
                    core::mem::transmute::<Option<I::Item<'this>>, Option<I::Item<'a>>>(peeked)
                }
                None => iter.next(),
            }
        }
    }
    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        // SAFETY: ...
        unsafe {
            let this = self.get_unchecked_mut();
            let iter = &mut this.iter;
            match this.peeked.take() {
                Some(None) => 0,
                Some(Some(_)) => 1 + iter.count(),
                None => iter.count(),
            }
        }
    }
    #[inline]
    fn nth<'a>(&'a mut self, n: usize) -> Option<I::Item<'a>> {
        // SAFETY: ...
        unsafe {
            let this = self.as_mut().get_unchecked_mut();
            let iter = &mut this.iter;
            match this.peeked.take() {
                Some(None) => None,
                // SAFETY: 'this: 'a
                Some(Some(v)) if n == 0 => {
                    Some(core::mem::transmute::<I::Item<'this>, I::Item<'a>>(v))
                }
                Some(Some(_)) => iter.nth(n - 1),
                None => iter.nth(n),
            }
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let this = self.as_ref().get_ref();
        let iter = &this.iter;
        match this.peeked {
            Some(None) => (0, Some(0)),
            Some(Some(_)) => (1, None),
            None => iter.size_hint(),
        }
    }
    // fn try_fold cannot be specialized as of 1.75.0
    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        for<'all> F: FnMut(B, I::Item<'all>) -> B,
    {
        // SAFETY: ...
        unsafe {
            let this = self.get_unchecked_mut();
            let iter = &mut this.iter;
            let mut acc = match this.peeked.take() {
                Some(None) => return init,
                Some(Some(v)) => f(init, v),
                None => init,
            };
            // iter.fold moves iter so it violates &mut Self.
            while let Some(x) = iter.next() {
                acc = f(acc, x);
            }
            acc
        }
    }
}

#[cfg(test)]
mod test {
    use core::pin::pin;

    use super::*;
    use crate::ToLendingIterator;
    #[test]
    fn test() {
        assert_eq!(
            Peekable::new((0..5).into_lending()).as_mut().skip(1).nth(1),
            (0..5).skip(1).nth(1)
        );
        assert_eq!(
            pin!(Peekable::new((0..5).into_lending())).peek(),
            (0..5).peekable().peek()
        );

        let mut peekable = Peekable::new(vec![0, 1].into_lending());
        assert_eq!(peekable.next(), Some(0));
        assert_eq!(peekable.next(), Some(1));
        assert_eq!(peekable.next(), None);
        peekable = Peekable::new(vec![0, 1, 2].into_lending());
        {
            let mut pin = pin!(peekable);
            assert_eq!(pin.as_mut().peek(), Some(&0));
            assert_eq!(pin.as_mut().peek(), Some(&0));
            assert_eq!(pin.as_mut().peek_mut(), Some(&mut 0));
            assert_eq!(pin.as_mut().next(), Some(0));
            assert_eq!(pin.as_mut().next(), Some(1));
            assert_eq!(pin.as_mut().peek(), Some(&2));
            let peekable = pin.unpin();
            assert_eq!(peekable.next(), None);
        }
    }
}
