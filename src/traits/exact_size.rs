use crate::LendingIterator;

/// A lending iterator that knows its exact length.
///
/// see [`ExactSizeIterator`](::core::iter::ExactSizeIterator).
pub trait ExactSizeLendingIterator: LendingIterator {
    /// Returns the exact remaining length of the iterator.
    ///
    /// see [`ExactSizeIterator::len()`](::core::iter::ExactSizeIterator::len).
    #[inline]
    fn len(&self) -> usize {
        let (lower, upper) = self.size_hint();
        assert_eq!(upper, Some(lower));
        lower
    }

    /// Returns `true` if the iterator is empty.
    ///
    /// see [`ExactSizeIterator::is_empty()`](::core::iter::ExactSizeIterator::is_empty).
    #[inline]
    #[cfg(feature = "unstable")]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<I: ExactSizeLendingIterator + ?Sized> ExactSizeLendingIterator for &mut I {
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline]
    #[cfg(feature = "unstable")]
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}
