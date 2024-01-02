use crate::LendingIterator;

/// A lending iterator that reports an accurate length using size_hint. (This is a guarantee for ExactSizeIterator in unsafe code)
///
/// see [`TrustedLen`](::core::iter::TrustedLen).
pub unsafe trait TrustedLen: LendingIterator {}
unsafe impl<I: TrustedLen + ?Sized> TrustedLen for &mut I {}
/// A iterator that reports an accurate length using size_hint.
///
/// see [`TrustedLen`](::core::iter::TrustedLen).
///
/// note: this is `pub` for the convenience of anyone who wants to reuse this crate's stable "implementation" of TrustedLenIterator (which is specialized in std).
pub unsafe trait TrustedLenIterator: Iterator {}
#[cfg(feature = "nightly")]
unsafe impl<I: ::core::iter::TrustedLen + ?Sized> TrustedLenIterator for I {}
#[cfg(not(feature = "nightly"))]
mod impl_trusted_len;
