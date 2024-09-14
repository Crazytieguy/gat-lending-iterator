//! This crate uses generic associated types to supply an iterator trait
//! that allows the items to \[mutably\] borrow from the iterator.
//! See [the GAT anouncement](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html)
//!
//! Most `Iterator` methods can work as is on `LendingIterator`s, but some wouldn't make sense.
//! Basically any method that needs to look at more than one element at once isn't possible, or needs to be modified.
//!
//! Some `LendingIterator` methods *may* return something that can act as an `Iterator`.
//! For example `cloned`, or `map`, when the function passed to it
//! returns a value that isn't tied to the lifetime of its input.
//! In these cases, my design choice was to conditionally implement `IntoIterator` for the adapter.
//!
//! This crate also provides an extension trait `ToLendingIterator: Iterator` for iterators
//! that allows turning them into lending iterators (over windows of elements).
//! There may be more methods added to this trait in the future.
//!
//! # Examples
//!
//! Using [`windows`](crate::ToLendingIterator::windows) on a range, filtering it and chaining it:
//! ```
//! use gat_lending_iterator::{LendingIterator, ToLendingIterator};
//!
//! (0..5)
//!     .windows(3)
//!     .filter(|x| x[0] % 2 == 0)
//!     .chain((0..6).windows(2))
//!     .for_each(|x| println!("{x:?}"));
//! ```
//!
//! Prints:
//! ```ignore
//! [0, 1, 2]
//! [2, 3, 4]
//! [0, 1]
//! [1, 2]
//! [2, 3]
//! [3, 4]
//! [4, 5]
//! ```
//!
//! Using [`windows_mut`](crate::ToLendingIterator::windows_mut) on a range, mutating it and mapping it:
//! ```
//! use gat_lending_iterator::{LendingIterator, ToLendingIterator};
//!
//! for sum in (0..7).windows_mut(2).map(|slice: &mut [usize]| {
//!     slice[1] += slice[0];
//!     slice[1]
//! }) {
//!     println!("{sum}");
//! }
//! ```
//!
//! Prints:
//! ```ignore
//! 1
//! 3
//! 6
//! 10
//! 15
//! 21
//! ```
//!
//! Using [`windows`](crate::ToLendingIterator::windows) on a range, and mapping it:
//! ```
//! use gat_lending_iterator::{LendingIterator, ToLendingIterator};
//! fn second(slice: &[usize]) -> &usize {
//!     &slice[1]
//! }
//!
//! for n in (0..5).windows(3).map(second).cloned() {
//!     println!("{n}");
//! }
//! ```
//!
//! Prints:
//! ```ignore
//! 1
//! 2
//! 3
//! ```

#![deny(missing_docs)]
#![warn(clippy::pedantic)]

mod adapters;
mod to_lending;
mod traits;
pub use self::adapters::*;
pub use self::to_lending::*;
pub use self::traits::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn second(slice: &[usize]) -> &usize {
        &slice[1]
    }

    #[test]
    fn playground() {
        (0..5)
            .windows(3)
            .filter(|x| x[0] % 2 == 0)
            .chain((0..6).windows(2))
            .for_each(|x| println!("{x:?}"));

        println!();

        for sum in (0..7).windows_mut(2).map(|slice: &mut [usize]| {
            slice[1] += slice[0];
            slice[1]
        }) {
            println!("{sum}");
        }

        println!();

        for n in (0..5).windows(3).map(second).cloned() {
            println!("{n}");
        }

        println!();

        (0..5)
            .windows(4)
            .zip([0, 1].into_lending())
            .for_each(|(a, b)| {
                println!("{a:?}, {b:?}");
            });
        
        (0..5)
            .windows(2)
            .skip_while(|w| w[0] < 2)
            .fold(0, |acc, x| acc + x[1]);
    }
}
