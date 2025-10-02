use crate::LendingIterator;

/// A lending iterator over mutable windows.
///
/// This `struct` is created by the [`windows_mut`] method on [`ToLendingIterator`]. See
/// its documentation for more.
///
/// [`ToLendingIterator`]: crate::ToLendingIterator
/// [`windows_mut`]: crate::ToLendingIterator::windows_mut
#[derive(Clone)]
pub struct WindowsMut<I: Iterator> {
    iter: I,
    size: usize,
    buf: Vec<I::Item>,
}

impl<I: Iterator> WindowsMut<I> {
    pub(crate) fn new(mut iter: I, size: usize) -> Self {
        let buf = iter.by_ref().take(size - 1).collect();
        Self { iter, size, buf }
    }
}

impl<I: Iterator> LendingIterator for WindowsMut<I> {
    type Item<'a> = &'a mut [I::Item]
        where
            Self: 'a
    ;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        self.iter.next().map(|next| {
            if self.buf.len() == self.size * 2 - 1 {
                self.buf.drain(..self.size);
            }
            self.buf.push(next);
            let range = self.buf.len() - self.size..;
            &mut self.buf[range]
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{LendingIterator, ToLendingIterator};

    fn accumulate_window(w: &mut [i32]) -> i32 {
        w[1] += w[0];
        w[1]
    }

    #[test]
    fn windows_mut_basic() {
        let result: Vec<_> = (0..5)
            .windows_mut(2)
            .map(accumulate_window)
            .into_iter()
            .collect();
        assert_eq!(result, vec![1, 3, 6, 10]);
    }

    #[test]
    fn windows_mut_modifies_elements() {
        let mut sum = 0;
        (0..4).windows_mut(2).for_each(|w| {
            w[0] = w[0] * 2;
            sum += w[0];
        });
        assert_eq!(sum, 6);
    }
}
