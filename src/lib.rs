//! # Lending Iterators with Generic Associated Types
//!
//! A **lending iterator** yields items that borrow from the iterator itself, unlike standard
//! `Iterator` where each item must be independent. This is enabled by [Generic Associated Types
//! (GATs)](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html):
//!
//! ```ignore
//! trait LendingIterator {
//!     type Item<'a> where Self: 'a;
//!     fn next(&mut self) -> Option<Self::Item<'_>>;
//! }
//! ```
//!
//! ## Why Lending Iterators?
//!
//! Standard iterators cannot return items that borrow from `&mut self` due to lifetime constraints.
//! Lending iterators solve this, enabling patterns like overlapping mutable windows without cloning
//! or streaming parsers that reuse internal buffers.
//!
//! ## API Design
//!
//! Most `Iterator` methods work on `LendingIterator`, except those requiring multiple items
//! simultaneously (e.g., `collect`, `peekable`). Some methods like `map` and `cloned` conditionally
//! implement `IntoIterator` when the returned value doesn't borrow from input.
//!
//! This crate provides `ToLendingIterator` to convert standard iterators into lending iterators,
//! enabling methods like `windows()` and `windows_mut()`.
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

    #[test]
    fn count_basic() {
        assert_eq!((0..5).windows(2).count(), 4);
        assert_eq!((0..3).into_lending().count(), 3);
        assert_eq!(std::iter::empty::<i32>().into_lending().count(), 0);
    }

    #[test]
    fn nth_basic() {
        let mut iter = (0..5).windows(2);
        assert_eq!(iter.nth(0), Some(&[0, 1][..]));
        assert_eq!(iter.nth(1), Some(&[2, 3][..]));
        assert_eq!(iter.nth(0), Some(&[3, 4][..]));
        assert_eq!(iter.nth(0), None);
    }

    #[test]
    fn nth_out_of_bounds() {
        let mut iter = (0..3).into_lending();
        assert_eq!(iter.nth(10), None);
    }

    #[test]
    fn advance_by_basic() {
        let mut iter = (0..5).into_lending();
        assert_eq!(iter.advance_by(2), Ok(()));
        assert_eq!(iter.next(), Some(2));
    }

    #[test]
    fn advance_by_too_far() {
        let mut iter = (0..3).into_lending();
        assert!(iter.advance_by(10).is_err());
    }

    #[test]
    fn for_each_basic() {
        let mut sum = 0;
        (1..=5).into_lending().for_each(|x| sum += x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn fold_basic() {
        let sum = (1..=5).into_lending().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn fold_product() {
        let product = (1..=5).into_lending().fold(1, |acc, x| acc * x);
        assert_eq!(product, 120);
    }

    #[test]
    fn all_true() {
        assert!((0..5).into_lending().all(|x| x < 10));
    }

    #[test]
    fn all_false() {
        assert!(!(0..5).into_lending().all(|x| x < 3));
    }

    #[test]
    fn all_empty() {
        assert!(std::iter::empty::<i32>().into_lending().all(|_| false));
    }

    #[test]
    fn any_true() {
        assert!((0..5).into_lending().any(|x| x == 3));
    }

    #[test]
    fn any_false() {
        assert!(!(0..5).into_lending().any(|x| x > 10));
    }

    #[test]
    fn any_empty() {
        assert!(!std::iter::empty::<i32>().into_lending().any(|_| true));
    }

    #[test]
    fn is_partitioned_true() {
        assert!(vec![2, 4, 6, 1, 3, 5]
            .into_lending()
            .is_partitioned(|x| x % 2 == 0));
    }

    #[test]
    fn is_partitioned_false() {
        assert!(!vec![2, 1, 4, 3]
            .into_lending()
            .is_partitioned(|x| x % 2 == 0));
    }

    #[test]
    fn find_found() {
        let mut iter = (0..5).into_lending();
        assert_eq!(iter.find(|x| *x == 3), Some(3));
    }

    #[test]
    fn find_not_found() {
        let mut iter = (0..5).into_lending();
        assert_eq!(iter.find(|x| *x == 10), None);
    }

    #[test]
    fn find_map_found() {
        let mut iter = vec![1, -2, 3, -4].into_lending();
        let result = iter.find_map(|x| if x < 0 { Some(-x) } else { None });
        assert_eq!(result, Some(2));
    }

    #[test]
    fn find_map_not_found() {
        let mut iter = vec![1, 2, 3].into_lending();
        let result = iter.find_map(|x| if x < 0 { Some(-x) } else { None });
        assert_eq!(result, None);
    }

    #[test]
    fn position_found() {
        let mut iter = (0..5).into_lending();
        assert_eq!(iter.position(|x| x == 3), Some(3));
    }

    #[test]
    fn position_not_found() {
        let mut iter = (0..5).into_lending();
        assert_eq!(iter.position(|x| x == 10), None);
    }

    #[test]
    fn cmp_equal() {
        use std::cmp::Ordering;
        let result = (0i32..3).into_lending().cmp((0i32..3).into_lending());
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn cmp_less() {
        use std::cmp::Ordering;
        let result = (0i32..3).into_lending().cmp((0i32..5).into_lending());
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn cmp_greater() {
        use std::cmp::Ordering;
        let result = (0i32..5).into_lending().cmp((0i32..3).into_lending());
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn cmp_by_basic() {
        use std::cmp::Ordering;
        let result = (0i32..3)
            .into_lending()
            .cmp_by((0i32..3).into_lending(), |a, b| a.cmp(&b));
        assert_eq!(result, Ordering::Equal);
    }

    #[test]
    fn partial_cmp_by_basic() {
        use std::cmp::Ordering;
        let result = (0i32..3)
            .into_lending()
            .partial_cmp_by((0i32..3).into_lending(), |a, b| a.partial_cmp(&b));
        assert_eq!(result, Some(Ordering::Equal));
    }

    #[test]
    fn eq_by_basic() {
        assert!((0i32..3)
            .into_lending()
            .eq_by((0i32..3).into_lending(), |a, b| a == b));
    }

    #[test]
    fn eq_by_false() {
        assert!(!(0i32..3)
            .into_lending()
            .eq_by((1i32..4).into_lending(), |a, b| a == b));
    }

    #[test]
    fn size_hint_basic() {
        let iter = (0..5).into_lending();
        assert_eq!(iter.size_hint(), (5, Some(5)));
    }

    fn to_vec_i32(w: &[i32]) -> Vec<i32> {
        w.to_vec()
    }

    #[test]
    fn complex_chain_windows_filter() {
        let result: Vec<_> = (0..5)
            .windows(3)
            .filter(|x| x[0] % 2 == 0)
            .chain((0..6).windows(2))
            .map(to_vec_i32)
            .into_iter()
            .collect();
        assert_eq!(
            result,
            vec![
                vec![0, 1, 2],
                vec![2, 3, 4],
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5]
            ]
        );
    }

    fn accumulate_window_usize(slice: &mut [usize]) -> usize {
        slice[1] += slice[0];
        slice[1]
    }

    #[test]
    fn complex_windows_mut_map() {
        let result: Vec<_> = (0..7)
            .windows_mut(2)
            .map(accumulate_window_usize)
            .into_iter()
            .collect();
        assert_eq!(result, vec![1, 3, 6, 10, 15, 21]);
    }

    fn second_usize(slice: &[usize]) -> &usize {
        &slice[1]
    }

    #[test]
    fn complex_windows_map_cloned() {
        let result: Vec<_> = (0..5)
            .windows(3)
            .map(second_usize)
            .cloned()
            .into_iter()
            .collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn reduce_basic() {
        let result: Option<i32> = (1..=5).into_lending().reduce(|a, b| a + b);
        assert_eq!(result, Some(15));
    }

    #[test]
    fn reduce_empty() {
        let result: Option<i32> = std::iter::empty::<i32>()
            .into_lending()
            .reduce(|a, b| a + b);
        assert_eq!(result, None);
    }

    #[test]
    fn reduce_single() {
        let result: Option<i32> = std::iter::once(42).into_lending().reduce(|a, b| a + b);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn try_fold_basic() {
        let result: Result<i32, &str> = (1..=5).into_lending().try_fold(0, |acc, x| Ok(acc + x));
        assert_eq!(result, Ok(15));
    }

    #[test]
    fn try_fold_early_exit() {
        let result = (1..=5).into_lending().try_fold(0, |acc, x| {
            if x == 3 {
                Err("stopped at 3")
            } else {
                Ok(acc + x)
            }
        });
        assert_eq!(result, Err("stopped at 3"));
    }

    #[test]
    fn try_for_each_basic() {
        let mut sum = 0;
        let result: Result<(), &str> = (1..=5).into_lending().try_for_each(|x| {
            sum += x;
            Ok(())
        });
        assert_eq!(result, Ok(()));
        assert_eq!(sum, 15);
    }

    #[test]
    fn try_for_each_early_exit() {
        let mut sum = 0;
        let result = (1..=5).into_lending().try_for_each(|x| {
            if x == 3 {
                Err("stopped")
            } else {
                sum += x;
                Ok(())
            }
        });
        assert_eq!(result, Err("stopped"));
        assert_eq!(sum, 3);
    }

    #[test]
    fn try_find_found() {
        let mut iter = (0..5).into_lending();
        let result: Result<Option<i32>, &str> =
            iter.try_find(|&x| if x == 3 { Ok(true) } else { Ok(false) });
        assert_eq!(result, Ok(Some(3)));
    }

    #[test]
    fn try_find_not_found() {
        let mut iter = (0..5).into_lending();
        let result: Result<Option<i32>, &str> = iter.try_find(|&x| Ok(x == 10));
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn try_find_error() {
        let mut iter = (0..5).into_lending();
        let result: Result<Option<i32>, &str> =
            iter.try_find(|&x| if x == 2 { Err("error") } else { Ok(false) });
        assert_eq!(result, Err("error"));
    }

    #[test]
    fn try_reduce_basic() {
        let result: Result<Option<i32>, &str> = (1..=5).into_lending().try_reduce(|a, b| Ok(a + b));
        assert_eq!(result, Ok(Some(15)));
    }

    #[test]
    fn try_reduce_empty() {
        let result: Result<Option<i32>, &str> = std::iter::empty::<i32>()
            .into_lending()
            .try_reduce(|a, b| Ok(a + b));
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn try_reduce_error() {
        let result: Result<Option<i32>, &str> =
            (1..=5).into_lending().try_reduce(
                |a, b| {
                    if b == 3 {
                        Err("error at 3")
                    } else {
                        Ok(a + b)
                    }
                },
            );
        assert_eq!(result, Err("error at 3"));
    }

    #[test]
    fn max_basic() {
        let max: i32 = (1..=5).into_lending().max().unwrap();
        assert_eq!(max, 5);
    }

    #[test]
    fn max_empty() {
        let max: Option<i32> = std::iter::empty::<i32>().into_lending().max();
        assert_eq!(max, None);
    }

    #[test]
    fn max_single() {
        let max: i32 = std::iter::once(42).into_lending().max().unwrap();
        assert_eq!(max, 42);
    }

    #[test]
    fn min_basic() {
        let min: i32 = (1..=5).into_lending().min().unwrap();
        assert_eq!(min, 1);
    }

    #[test]
    fn min_empty() {
        let min: Option<i32> = std::iter::empty::<i32>().into_lending().min();
        assert_eq!(min, None);
    }

    #[test]
    fn max_by_basic() {
        let max: i32 = (1..=5)
            .into_lending()
            .max_by(|a: &i32, b: &i32| a.cmp(b))
            .unwrap();
        assert_eq!(max, 5);
    }

    #[test]
    fn min_by_basic() {
        let min: i32 = (1..=5)
            .into_lending()
            .min_by(|a: &i32, b: &i32| a.cmp(b))
            .unwrap();
        assert_eq!(min, 1);
    }

    #[test]
    fn max_by_key_basic() {
        let max: i32 = vec![1, 5, 3, 2, 4]
            .into_lending()
            .max_by_key(|x| *x)
            .unwrap();
        assert_eq!(max, 5);
    }

    #[test]
    fn min_by_key_basic() {
        let min: i32 = vec![3, 1, 4, 5, 2]
            .into_lending()
            .min_by_key(|x| *x)
            .unwrap();
        assert_eq!(min, 1);
    }

    #[test]
    fn sum_basic() {
        let sum: i32 = (1..=5).into_lending().sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn sum_empty() {
        let sum: i32 = std::iter::empty::<i32>().into_lending().sum();
        assert_eq!(sum, 0);
    }

    #[test]
    fn product_basic() {
        let product: i32 = (1..=5).into_lending().product();
        assert_eq!(product, 120);
    }

    #[test]
    fn product_empty() {
        let product: i32 = std::iter::empty::<i32>().into_lending().product();
        assert_eq!(product, 1);
    }

    #[test]
    fn max_with_windows() {
        let max: Vec<i32> = (0..5)
            .windows(2)
            .max_by(|a: &Vec<i32>, b| {
                if a[0] == b[0] {
                    a[1].cmp(&b[1])
                } else {
                    a[0].cmp(&b[0])
                }
            })
            .unwrap();
        assert_eq!(max, vec![3, 4]);
    }

    #[test]
    fn min_with_windows() {
        let min: Vec<i32> = (0..5)
            .windows(2)
            .min_by(|a: &Vec<i32>, b| {
                if a[0] == b[0] {
                    a[1].cmp(&b[1])
                } else {
                    a[0].cmp(&b[0])
                }
            })
            .unwrap();
        assert_eq!(min, vec![0, 1]);
    }

    #[test]
    fn max_by_with_windows() {
        let max: Vec<i32> = (0..5)
            .windows(2)
            .max_by(|a: &Vec<i32>, b| a[0].cmp(&b[0]))
            .unwrap();
        assert_eq!(max, vec![3, 4]);
    }

    #[test]
    fn max_by_key_with_windows() {
        let max: Vec<i32> = (0..5).windows(2).max_by_key(|w| w[0] + w[1]).unwrap();
        assert_eq!(max, vec![3, 4]);
    }

    #[test]
    fn reduce_with_windows() {
        let sum: Vec<i32> = (0..5)
            .windows(2)
            .reduce(|mut acc: Vec<i32>, w| {
                acc[0] += w[0];
                acc[1] += w[1];
                acc
            })
            .unwrap();
        assert_eq!(sum, vec![6, 10]);
    }

    #[test]
    fn try_fold_with_windows() {
        let result: Result<i32, &str> = (0..5)
            .windows(2)
            .try_fold(0, |acc, w| Ok(acc + w[0] + w[1]));
        assert_eq!(result, Ok(16));
    }

    #[test]
    fn try_reduce_with_windows() {
        let result: Result<Option<Vec<i32>>, &str> =
            (0..5).windows(2).try_reduce(|mut acc: Vec<i32>, w| {
                acc[0] += w[0];
                acc[1] += w[1];
                Ok(acc)
            });
        assert_eq!(result, Ok(Some(vec![6, 10])));
    }

    #[test]
    fn inspect_with_windows() {
        use std::cell::Cell;
        use std::rc::Rc;
        let sum = Rc::new(Cell::new(0));
        let sum_clone = sum.clone();
        let _: Vec<_> = (0..4)
            .windows(2)
            .inspect(move |w| sum_clone.set(sum_clone.get() + w[0] + w[1]))
            .map(to_vec_i32)
            .into_iter()
            .collect();
        assert_eq!(sum.get(), 9);
    }

    #[test]
    fn scan_with_windows() {
        let mut result = Vec::new();
        (0..4)
            .windows(2)
            .scan(0i32, |state: &mut i32, w: &[i32]| {
                *state += w[0];
                Some(*state)
            })
            .for_each(|x| result.push(x));
        assert_eq!(result, vec![0, 1, 3]);
    }

    #[test]
    fn fuse_with_windows() {
        let mut iter = (0..3).windows(2).fuse();
        assert_eq!(iter.next(), Some(&[0, 1][..]));
        assert_eq!(iter.next(), Some(&[1, 2][..]));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn map_while_with_windows() {
        let mut result = Vec::new();
        (0..5)
            .windows(2)
            .map_while(|w: &[i32]| if w[0] < 2 { Some(w[0] + w[1]) } else { None })
            .for_each(|x| result.push(x));
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn cycle_with_lending() {
        let mut iter = (0..3).into_lending().cycle();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn copied_with_refs() {
        let data = vec![1, 2, 3, 4];
        let result: Vec<_> = data.lend_refs().copied().into_iter().collect();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }
}
