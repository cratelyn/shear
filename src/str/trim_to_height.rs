use {
    super::ellipsis::Ellipsis,
    crate::iter::{Limited, LimitedIter},
    std::{marker::PhantomData, str::Lines},
    tap::Pipe,
};

pub struct TrimToHeightIter<'a, E> {
    lines: Lines<'a>,
    ellipses: PhantomData<E>,
}

// === impl TrimToHeightIter ===

impl<'a, E> TrimToHeightIter<'a, E> {
    /// returns a new [`TrimToHeightIter`].
    pub fn new<S>(s: &'a S) -> Self
    where
        S: AsRef<str> + ?Sized,
    {
        Self {
            lines: s.as_ref().lines(),
            ellipses: PhantomData,
        }
    }
}

impl<'a, E> Limited for TrimToHeightIter<'a, E>
where
    E: Ellipsis,
{
    fn limited(self, size: usize) -> LimitedIter<Self> {
        LimitedIter::new(self, size)
    }

    type Contd = std::iter::Once<&'a str>;

    fn contd() -> Self::Contd {
        E::ellipsis().pipe(std::iter::once)
    }

    /// counts characters according to their unicode width.
    ///
    /// see [`unicode_width`] for more information.
    fn element_size(item: &Self::Item) -> usize {
        if item.is_empty() {
            return 0;
        }

        item.chars().filter(|c| *c == '\n').count() + 1
    }
}

impl<'a, E> Iterator for TrimToHeightIter<'a, E> {
    type Item = <Lines<'a> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { lines, .. } = self;

        lines.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let Self { lines, .. } = self;

        lines.size_hint()
    }
}

#[cfg(test)]
mod element_size {
    use super::*;
    use crate::str::ellipsis;

    type TrimToHeightIter = super::TrimToHeightIter<'static, ellipsis::Ascii>;

    #[test]
    fn empty() {
        TrimToHeightIter::element_size(&"").pipe(|height| assert_eq!(height, 0))
    }

    #[test]
    fn one_line() {
        TrimToHeightIter::element_size(&"line").pipe(|height| assert_eq!(height, 1))
    }

    #[test]
    fn two_lines() {
        TrimToHeightIter::element_size(&"two\nlines").pipe(|height| assert_eq!(height, 2))
    }

    #[test]
    fn trailing_newline() {
        TrimToHeightIter::element_size(&"two lines\nwith a trailing newline\n")
            .pipe(|height| assert_eq!(height, 3))
    }
}
