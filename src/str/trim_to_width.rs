use {
    super::ellipsis::Ellipsis,
    crate::iter::{Limited, LimitedIter},
    std::marker::PhantomData,
};

pub struct TrimToWidthIter<I, E> {
    iter: I,
    ellipses: PhantomData<E>,
}

// === impl TrimToWidthIter ===

impl<I, E> TrimToWidthIter<I, E> {
    /// returns a new [`TrimToWidthIter`].
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            ellipses: PhantomData,
        }
    }
}

/// character iterators can be limited with an [`Ellipsis`].
impl<I, E> Limited for TrimToWidthIter<I, E>
where
    I: Iterator<Item = char> + Sized,
    E: Ellipsis,
{
    fn limited(self, length: usize) -> LimitedIter<Self> {
        LimitedIter::new(self, length)
    }

    type ContdIter = std::str::Chars<'static>;

    fn contd() -> Self::ContdIter {
        E::ellipsis().chars()
    }

    /// counts characters according to their unicode width.
    ///
    /// see [`unicode_width`] for more information.
    fn element_size(c: &char) -> usize {
        use unicode_width::UnicodeWidthChar;

        c.width()
            .unwrap_or_default(/* returns `None` for control characters */)
    }
}

impl<I, E> Iterator for TrimToWidthIter<I, E>
where
    I: Iterator<Item = char> + Sized,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { iter, .. } = self;

        iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let Self { iter, .. } = self;

        iter.size_hint()
    }
}
