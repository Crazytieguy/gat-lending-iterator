use crate::LendingIterator;

/// Creates an iterator that lends an element exactly once.
pub fn once<T>(value: T) -> Once<T> {
    Once {
        done: false,
        item: Some(value),
    }
}

/// An iterator that lends an element exactly once.
///
/// This `struct` is created by the [`once()`] function. See its documentation for more.
pub struct Once<T> {
    done: bool,
    item: Option<T>,
}

impl<T> LendingIterator for Once<T> {
    type Item<'a> = &'a T where T: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.done {
            self.item = None;
        } else {
            self.done = true;
        }
        self.item.as_ref()
    }
}
