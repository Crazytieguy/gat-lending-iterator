# Gat Lending Iterator

A library for lending iterators using Generic Associated Types (GATs). **Work in progress**.

[![Crates.io](https://img.shields.io/crates/v/gat-lending-iterator.svg)](https://crates.io/crates/gat-lending-iterator)
[![Documentation](https://docs.rs/gat-lending-iterator/badge.svg)](https://docs.rs/gat-lending-iterator)

## What are Lending Iterators?

**Lending iterators** (also called "streaming iterators") yield items that borrow from the iterator itself, rather than being owned values. This solves a fundamental limitation of Rust's standard `Iterator` trait.

### Why Use Them?

Standard Rust iterators cannot yield items that borrow from the iterator's internal state. This prevents patterns like:
- Iterating over **overlapping windows** without cloning each window
- Iterating with **mutable views** that can modify underlying data
- Creating **streaming parsers** without buffering entire chunks

Lending iterators enable these patterns efficiently using [Generic Associated Types (GATs)](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html), allowing items to have lifetimes tied to the iterator:

```rust
trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
```

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

Most `Iterator` methods can work as is on `LendingIterator`s, but some wouldn't make sense. Basically any method that needs to look at more than one element at once isn't possible.

Some `LendingIterator` methods _may_ return something that can act as an `Iterator`. For example `cloned`, or `map`, when the function passed to it returns a value that isn't tied to the lifetime of its input. In these cases, the design choice was to conditionally implement `IntoIterator` for the adapter.

The crate also includes an extension trait `ToLendingIterator: IntoIterator` for iterators that allows turning them into lending iterators in various ways, for example over windows of elements.

## methods that behave the same on `LendingIterator`s as they do on `Iterator`s

- advance_by
- all
- any
- by_ref
- chain
- cmp
- cmp_by
- count
- cycle
- enumerate
- eq
- eq_by
- filter
- filter_map
- find
- find_map
- flat_map
- flatten
- fold
- for_each
- fuse
- ge
- gt
- inspect
- intersperse
- intersperse_with
- is_partitioned
- le
- lt
- map
- map_while
- ne
- nth
- partial_cmp
- partial_cmp_by
- position
- product
- scan
- size_hint
- skip
- skip_while
- step_by
- sum
- take
- take_while
- try_find
- try_fold
- try_for_each
- zip

## methods that don't make sense on `LendingIterator`s

- array_chunks
- collect
- collect_into
- is_sorted
- is_sorted_by
- is_sorted_by_key
- last
- next_chunk
- partition
- partition_in_place
- peekable
- try_collect
- unzip

## methods that behave differently on `LendingIterator`s

- max
- max_by
- max_by_key
- min
- min_by
- min_by_key
- reduce
- try_reduce

## methods that can sometimes be used to convert a `LendingIterator` into an `Iterator`

- cloned
- copied
- filter_map
- flat_map
- map
- map_while
- scan

## methods I'm not sure about

- rev
- rposition
