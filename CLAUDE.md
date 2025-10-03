# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library implementing a lending iterator trait using Generic Associated Types (GATs). Lending iterators allow items to borrow from `&mut self`, which enables patterns like iterating over windows of elements with references that don't outlive the iterator.

## Core Architecture

**Main Trait**: `LendingIterator` (src/traits/lending_iterator.rs)
- Uses GAT syntax: `type Item<'a> where Self: 'a`
- Items can borrow from the iterator, enforced by the compiler
- Methods mirror standard `Iterator` trait where applicable

**Extension Trait**: `ToLendingIterator` (src/traits/to_lending_iterator.rs)
- Implemented for all `IntoIterator` types
- Converts regular iterators into lending iterators
- Key methods: `windows()`, `windows_mut()`, `into_lending()`, `lend_refs()`, `lend_refs_mut()`

**Adapters** (src/adapters/)
- Each adapter type is in its own file
- Some adapters conditionally implement `IntoIterator` when the returned type doesn't borrow from input (e.g., `Map`, `Cloned`)
- Helper traits: `SingleArgFnMut`, `SingleArgFnOnce`, `OptionTrait` enable generic behavior over closures

**Converters** (src/to_lending/)
- Transform regular iterators into lending iterators
- `Windows` and `WindowsMut` use a buffer that grows to at most size * 2 (tradeoff between memory and avoiding element shifting)

## Design Constraints

- Closure lifetime binders: On stable Rust, closures can't have lifetime-bound outputs tied to inputs. Use functions instead or nightly's `closure_lifetime_binder` feature.
- Some `Iterator` methods don't make sense for `LendingIterator` (e.g., `collect`, `peekable`, `last`) because they require multiple items to be accessible simultaneously.
- The `find()` and `find_map()` methods use unsafe pointer casting (polonius pattern) to work around current borrow checker limitations.

## Important Notes

- Quality is the most important consideration, only commit work you think maintains or improves the quality of the crate
- Always test, clippt, and fmt before pushing

