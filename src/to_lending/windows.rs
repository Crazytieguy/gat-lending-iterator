use crate::LendingIterator;

/// A lending iterator over windows.
///
/// This `struct` is created by the [`windows`] method on [`ToLendingIterator`]. See
/// its documentation for more.
///
/// [`ToLendingIterator`]: crate::ToLendingIterator
/// [`windows`]: crate::ToLendingIterator::windows
#[derive(Clone)]
pub struct Windows<I: Iterator> {
    iter: I,
    size: usize,
    buf: Vec<I::Item>,
}

impl<I: Iterator> Windows<I> {
    pub(crate) fn new(mut iter: I, size: usize) -> Self {
        let buf = iter.by_ref().take(size - 1).collect();
        Self { iter, size, buf }
    }
}

impl<I: Iterator> LendingIterator for Windows<I> {
    type Item<'a>
        = &'a [I::Item]
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|next| {
            if self.buf.len() == self.size * 2 - 1 {
                self.buf.drain(..self.size);
            }
            self.buf.push(next);
            &self.buf[self.buf.len() - self.size..]
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{LendingIterator, ToLendingIterator};

    fn to_vec_i32(w: &[i32]) -> Vec<i32> {
        w.to_vec()
    }

    #[test]
    fn windows_basic() {
        let result: Vec<_> = (0..5).windows(3).map(to_vec_i32).into_iter().collect();
        assert_eq!(result, vec![vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]]);
    }

    #[test]
    fn windows_size_one() {
        let result: Vec<_> = (0..3).windows(1).map(to_vec_i32).into_iter().collect();
        assert_eq!(result, vec![vec![0], vec![1], vec![2]]);
    }

    #[test]
    fn windows_larger_than_iterator() {
        let mut iter = (0..3).windows(5);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn windows_empty_iterator() {
        let mut iter = std::iter::empty::<i32>().windows(3);
        assert_eq!(iter.next(), None);
    }
}
