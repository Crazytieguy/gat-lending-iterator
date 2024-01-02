use super::*;

unsafe impl<I: TrustedLenIterator + ?Sized> TrustedLenIterator for &mut I {}

unsafe impl<T, const N: usize> TrustedLenIterator for ::core::array::IntoIter<T, N> {}
unsafe impl<A, B> TrustedLenIterator for ::core::iter::Chain<A, B>
where
    A: TrustedLenIterator,
    B: Iterator<Item = A::Item> + TrustedLenIterator,
{
}
unsafe impl<'a, T: 'a, I> TrustedLenIterator for ::core::iter::Cloned<I>
where
    T: Clone,
    I: Iterator<Item = &'a T> + TrustedLenIterator,
{
}
unsafe impl<'a, T: 'a, I> TrustedLenIterator for ::core::iter::Copied<I>
where
    T: Copy,
    I: Iterator<Item = &'a T> + TrustedLenIterator,
{
}

unsafe impl<I> TrustedLenIterator for ::core::iter::Enumerate<I> where I: TrustedLenIterator {}
struct FlattenCompat<I, U>(I, U);
impl<I, U> Iterator for FlattenCompat<I, U>
where
    I: Iterator,
    U: Iterator,
    I::Item: IntoIterator<Item = U::Item, IntoIter = U>,
{
    type Item = ();
    fn next(&mut self) -> Option<()> {
        None
    }
}
unsafe impl<I, T, const N: usize> TrustedLenIterator
    for FlattenCompat<I, ::core::array::IntoIter<T, N>>
where
    I: TrustedLenIterator<Item = [T; N]>,
{
}
unsafe impl<'a, I, T, const N: usize> TrustedLenIterator
    for FlattenCompat<I, ::core::slice::Iter<'a, T>>
where
    I: TrustedLenIterator<Item = &'a [T; N]>,
{
}
unsafe impl<'a, I, T, const N: usize> TrustedLenIterator
    for FlattenCompat<I, ::core::slice::IterMut<'a, T>>
where
    I: TrustedLenIterator<Item = &'a mut [T; N]>,
{
}
unsafe impl<I, U, F> TrustedLenIterator for ::core::iter::FlatMap<I, U, F>
where
    I: Iterator,
    U: IntoIterator,
    F: FnMut(I::Item) -> U,
    FlattenCompat<::core::iter::Map<I, F>, <U as IntoIterator>::IntoIter>: TrustedLenIterator,
{
}
unsafe impl<I> TrustedLenIterator for ::core::iter::Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    FlattenCompat<I, <I::Item as IntoIterator>::IntoIter>: TrustedLenIterator,
{
}
unsafe impl<I> TrustedLenIterator for ::core::iter::Fuse<I> where I: TrustedLenIterator {}
unsafe impl<I, F, U> TrustedLenIterator for ::core::iter::Map<I, F>
where
    I: TrustedLenIterator,
    F: FnMut(I::Item) -> U,
{
}
unsafe impl<I> TrustedLenIterator for ::core::iter::Peekable<I> where I: TrustedLenIterator {}
unsafe impl<I> TrustedLenIterator for ::core::iter::Rev<I> where
    I: TrustedLenIterator + DoubleEndedIterator
{
}
unsafe impl TrustedLenIterator for ::core::iter::StepBy<::core::ops::Range<u8>> {}
unsafe impl TrustedLenIterator for ::core::iter::StepBy<::core::ops::Range<u16>> {}
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
unsafe impl TrustedLenIterator for ::core::iter::StepBy<::core::ops::Range<u32>> {}
#[cfg(target_pointer_width = "64")]
unsafe impl TrustedLenIterator for ::core::iter::StepBy<::core::ops::Range<u64>> {}
unsafe impl TrustedLenIterator for ::core::iter::StepBy<::core::ops::Range<usize>> {}
unsafe impl<I> TrustedLenIterator for ::core::iter::Take<I> where I: TrustedLenIterator {}
unsafe impl<A, B> TrustedLenIterator for ::core::iter::Zip<A, B>
where
    A: TrustedLenIterator,
    B: TrustedLenIterator,
{
}
macro_rules! unsafe_impl_trusted_step {
    ($($t:ty),*) => {$(
        unsafe impl TrustedLenIterator for ::core::ops::Range<$t> {}
        unsafe impl TrustedLenIterator for ::core::ops::RangeFrom<$t> {}
        unsafe impl TrustedLenIterator for ::core::ops::RangeInclusive<$t> {}
    )*};
}
unsafe_impl_trusted_step![::core::net::Ipv4Addr, ::core::net::Ipv6Addr];
unsafe_impl_trusted_step![char, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
unsafe impl<T> TrustedLenIterator for ::core::iter::Empty<T> {}
unsafe impl<T> TrustedLenIterator for ::core::iter::Once<T> {}
unsafe impl<T, F: FnOnce() -> T> TrustedLenIterator for ::core::iter::OnceWith<F> {}
unsafe impl<T: Clone> TrustedLenIterator for ::core::iter::Repeat<T> {}
unsafe impl<T, F: FnMut() -> T> TrustedLenIterator for ::core::iter::RepeatWith<F> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::option::Iter<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::option::IterMut<'a, T> {}
unsafe impl<T> TrustedLenIterator for ::core::option::IntoIter<T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::result::Iter<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::result::IterMut<'a, T> {}
unsafe impl<T> TrustedLenIterator for ::core::result::IntoIter<T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::Iter<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::IterMut<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::Windows<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::Chunks<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::ChunksMut<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::ChunksExact<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::ChunksExactMut<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::RChunks<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::RChunksMut<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::RChunksExact<'a, T> {}
unsafe impl<'a, T> TrustedLenIterator for ::core::slice::RChunksExactMut<'a, T> {}
unsafe impl<'a> TrustedLenIterator for ::core::str::Bytes<'a> {}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use ::alloc::{collections::*, vec};
    unsafe impl<T> TrustedLenIterator for vec_deque::Iter<'_, T> {}
    unsafe impl<T> TrustedLenIterator for vec_deque::IterMut<'_, T> {}
    unsafe impl<T> TrustedLenIterator for vec_deque::IntoIter<T> {}
    unsafe impl<T> TrustedLenIterator for vec::Drain<'_, T> {}
    unsafe impl<T> TrustedLenIterator for vec::IntoIter<T> {}
}
