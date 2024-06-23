use crate::iter::{Limited, LimitedIter};

pub struct TrimToLengthIter<I>(pub(super) I);

// === impl TrimToLengthIter ===

/// character iterators can be limited with elipses.
impl<I> Limited for TrimToLengthIter<I>
where
    I: Iterator<Item = char> + Sized,
{
    fn limited(self, length: usize) -> LimitedIter<Self> {
        LimitedIter::new(self, length)
    }

    type ContdIter = std::str::Chars<'static>;

    fn contd() -> Self::ContdIter {
        "...".chars()
    }
}

impl<I> Iterator for TrimToLengthIter<I>
where
    I: Iterator<Item = char> + Sized,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
