use {
    super::ellipsis::Ellipsis,
    crate::iter::{Limited, LimitedIter},
    std::marker::PhantomData,
};

pub struct TrimToLengthIter<I, E> {
    iter: I,
    ellipses: PhantomData<E>,
}

// === impl TrimToLengthIter ===

impl<I, E> TrimToLengthIter<I, E> {
    /// returns a new [`TrimToLengthIter`].
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            ellipses: PhantomData,
        }
    }
}

/// character iterators can be limited with an [`Ellipsis`].
impl<I, E> Limited for TrimToLengthIter<I, E>
where
    I: Iterator<Item = char> + Sized,
    E: Ellipsis,
{
    fn limited(self, size: usize) -> LimitedIter<Self> {
        LimitedIter::new(self, size)
    }

    type Contd = std::str::Chars<'static>;

    fn contd() -> Self::Contd {
        E::ellipsis().chars()
    }

    /// counts characters according to their encoded length.
    fn element_size(c: &char) -> usize {
        c.len_utf8()
    }
}

impl<I, E> Iterator for TrimToLengthIter<I, E>
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
