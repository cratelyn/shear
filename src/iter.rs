use {
    std::iter::Peekable,
    tap::{Pipe, TapOptional},
};

/// a trait for "limiting" an iterator.
///
/// [`limited()`][Limited::limited] will transform an iterator, returning a [`LimitedIter<I>`] that
/// will be limited by `size`.
///
/// [`element_size()`][Limited::element_size] determines how "large" an item is. by default, an
/// identity function that counts items is used, always returning `1`. if an iterator's contents
/// are too long to fit in `size`, then [`contd()`][Limited::contd] will be yielded, indicating
/// that the iterator has been limited.
///
/// use [`str::Limited`][crate::str::Limited] to limit the contents of strings.
pub trait Limited: Iterator + Sized {
    /// returns a "limited" iterator.
    fn limited(self, size: usize) -> LimitedIter<Self> {
        LimitedIter::new(self, size)
    }

    /// the type of iterator returned by [`Limited::contd()`].
    type ContdIter: Iterator<Item = Self::Item>;

    /// returns an iterator of values to use as an indication of truncation.
    ///
    /// e.g. for strings, represented as an iterator of characters, one might use `"..."`.
    fn contd() -> Self::ContdIter;

    /// defines the size of an item in this iterator.
    ///
    /// this is how "space" is measured by this limited iterator. for example, how many bytes are
    /// needed to encode a character if limiting a string according to bytes, or by how many
    /// columns wide a character is if limiting a string according to visual width.
    ///
    /// by default this counts each element as 1.
    fn element_size(_: &Self::Item) -> usize {
        1
    }
}

/// a "limited" iterator.
///
/// see [`Limited::limited()`] for more information.
pub struct LimitedIter<I: Iterator> {
    inner: Inner<I>,
}

/// the inner finite state machine for a [`LimitedIter<I>`].
///
/// ```ignore
///                       ┏━━━━━━━━━━┓
///               +---->  ┃ limiting ┃ >--+
/// ┏━━━━━━━━━━┓  |       ┗━━━━━━━━━━┛    |     ┏━━━━━━━━━━┓
/// ┃ running  ┃ -+---->  >----------> >--+---> ┃ finished ┃
/// ┗━━━━━━━━━━┛  |       ┏━━━━━━━━━━┓    |     ┗━━━━━━━━━━┛
///               +---->  ┃ tail     ┃ >--+
///                       ┗━━━━━━━━━━┛
/// ```
enum Inner<I: Iterator> {
    /// the iterator is running.
    Running {
        iter: Peekable<I>,
        remaining: usize,
        contd: Vec<I::Item>,
    },
    /// the iterator is emitting the "tail" of the sequence.
    ///
    /// in this state, the iterator is either emitting the contents of [`I::contd()`], or
    /// possibly the end of the inner iterator's items if they fit in the remaining space.
    Tail {
        iter: Peekable<<Vec<I::Item> as IntoIterator>::IntoIter>,
    },
    /// the iterator is finished.
    ///
    /// we will always yield `None` once in this terminal state.
    Finished,
}

// === impl limitediter ===

impl<I: Iterator + Limited> LimitedIter<I> {
    /// returns a new [`LimitedIter`].
    pub fn new(iter: I, size: usize) -> Self {
        Inner::new(iter, size).pipe(|inner| Self { inner })
    }
}

impl<I: Iterator + Limited> Iterator for LimitedIter<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        use Inner::*;

        let Self { inner } = self;

        /// helper macro:
        ///
        /// get the next item in the iterator, and mark ourselves as finished if the next item
        /// was `None`.
        macro_rules! next_and_mark_finished {
            ($iter:ident) => {
                $iter.next().tap_none(|| {
                    *inner = Finished; // the inner iterator is empty. we are all done!
                })
            };
        }

        match inner {
            Running {
                contd,
                iter,
                remaining,
            } => {
                match iter
                    .peek()
                    .map(I::element_size) // how much space does the next item take..
                    .map(|len| remaining.checked_sub(len)) // ..and does it fit?
                {
                    // the next item exists, and there is room for this element.
                    Some(Some(r)) => {
                        *remaining = r;
                        next_and_mark_finished!(iter)
                    }
                    // the next item exists, but we have to determine whether to truncate.
                    Some(None) => {
                        let space = {
                            let c = contd.iter().map(I::element_size).sum::<usize>();
                            c + *remaining
                        };

                        *inner = Self::collect_tail(iter, space)
                            .unwrap_or_else(|| std::mem::take(contd))
                            .pipe(Inner::tail);

                        self.next()
                    }
                    // the inner iterator has finished.
                    None => {
                        *inner = Finished;
                        None
                    }
                }
            }

            Tail { iter } => next_and_mark_finished!(iter),
            Finished => None, /* we are already done. */
        }
    }
}

impl<I: Iterator + Limited> LimitedIter<I> {
    /// returns the "tail" of an [`Iterator`].
    ///
    /// if the remaining elements of the iterator take more than `remaining` space according to
    /// [`Limited::element_size()`], this returns `None`.
    fn collect_tail(iter: &mut Peekable<I>, mut remaining: usize) -> Option<Vec<I::Item>> {
        let mut tail: Vec<I::Item> = iter
            .size_hint()
            .pipe(|(lower, upper)| upper.unwrap_or(lower))
            .pipe(Vec::with_capacity);

        for item in iter {
            let size = I::element_size(&item);
            if size > remaining {
                return None;
            }
            remaining -= size;
            tail.push(item);
        }

        Some(tail)
    }
}

impl<I: Iterator> LimitedIter<I> {
    /// returns true if this iterator is finished.
    pub fn is_finished(&self) -> bool {
        matches!(
            self,
            Self {
                inner: Inner::Finished
            }
        )
    }
}

// === impl inner ===

impl<I: Iterator + Limited> Inner<I> {
    /// returns a new [`Inner`].
    fn new(iter: I, total: usize) -> Self {
        // collect the continuation sequence, and find out how large it is.
        let contd = I::contd().collect::<Vec<_>>();
        let contd_size = contd.iter().map(I::element_size).sum();

        match total.checked_sub(contd_size) {
            Some(0) | None => Self::tail(contd),
            Some(remaining @ 1..) => Self::Running {
                iter: iter.peekable(),
                remaining,
                contd,
            },
        }
    }

    /// returns a new [`Inner`] given a vector of items.
    fn tail(tail: Vec<I::Item>) -> Self {
        tail.into_iter().peekable().pipe(|iter| Self::Tail { iter })
    }
}
