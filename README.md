# Gat Lending Iterator

A library for lending iterators using Generic Associated Types (GATs). **Work in progress**.

[![Crates.io](https://img.shields.io/crates/v/gat-lending-iterator.svg)](https://crates.io/crates/gat-lending-iterator)
[![Documentation](https://docs.rs/gat-lending-iterator/badge.svg)](https://docs.rs/gat-lending-iterator)

## What are Lending Iterators?

**Lending iterators** yield items that borrow from the iterator itself. Standard Rust iterators cannot do this—each item must be independent. Using [Generic Associated Types](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html), lending iterators enable efficient patterns like overlapping windows without cloning, iteration over mutable views, and streaming parsers without buffering.

## Example

```rust
use gat_lending_iterator::{LendingIterator, ToLendingIterator};

// Iterate over overlapping windows
let result: Vec<Vec<i32>> = (0..5)
    .windows(3)
    .map(|w| w.to_vec())
    .into_iter()
    .collect();
assert_eq!(result, vec![vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]]);
```

## Features

Most `Iterator` methods work on `LendingIterator`, except those requiring multiple items simultaneously (e.g., `collect`, `peekable`). Some methods like `map` and `cloned` conditionally implement `IntoIterator` when the returned value doesn't borrow from input.

The crate includes a `ToLendingIterator` extension trait for converting standard iterators into lending iterators with methods like `windows()` and `windows_mut()`.
