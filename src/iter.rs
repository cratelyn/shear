use {
    std::{iter::Peekable, ops::SubAssign},
    tap::{Pipe, TapOptional},
};

/// a trait for "limiting" an iterator.
///
/// this is used to wrap an iterator,
pub trait Limited: Iterator + Sized {
    /// returns a "limited" iterator.
    ///
    /// this will return at most `size` elements. once space is running out, the contents of
    /// the iterator returned by [`Limited::contd()`] will be used to indicate that the value is
    /// being "limited", or truncated.
    ///
    /// e.g. for strings, represented as an iterator of characters, one might use `"..."`.
    fn limited(self, length: usize) -> LimitedIter<Self> {
        LimitedIter::new(self, length)
    }

    /// the type of iterator returned by [`Limited::contd()`].
    type ContdIter: Iterator<Item = Self::Item>;

    /// returns an iterator of values to use as an indication of truncation.
    fn contd() -> Self::ContdIter;
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
    /// the iterator is limiting the value.
    ///
    /// in this state, the iterator is emitting the contents of [`I::contd()`].
    Limiting {
        iter: Peekable<<Vec<I::Item> as IntoIterator>::IntoIter>,
    },
    /// the iterator has opted to emit the "tail" of the inner sequence.
    ///
    /// this will occur if the the inner sequence is large enough to have *possibly* needed
    /// truncation, but wasn't needed after all.
    Tail {
        iter: <Vec<I::Item> as IntoIterator>::IntoIter,
    },
    /// the iterator is finished.
    ///
    /// we will always yield `None` once in this terminal state.
    Finished,
}

// === impl i: iterator ===

/// character iterators can be limited with elipses.
impl<I> Limited for I
where
    I: Iterator<Item = char> + Sized,
{
    fn limited(self, length: usize) -> LimitedIter<Self> {
        Inner::new(self, length).pipe(LimitedIter::new)
    }

    type ContdIter = std::str::Chars<'static>;

    fn contd() -> Self::ContdIter {
        "...".chars()
    }
}

// === impl limitediter ===

impl<I: Iterator> LimitedIter<I> {
    /// returns a new [`LimitedIter`].
    fn new(inner: Inner<I>) -> Self {
        Self { inner }
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
            // if we have space remaining, emit the next element in the sequence.
            Running {
                iter,
                contd: _,
                remaining: remaining @ 1..,
            } => {
                remaining.sub_assign(1);
                next_and_mark_finished!(iter)
            }

            // if we have no more space remaining, then we should begin emitting the `contd` items.
            Running {
                contd,
                iter,
                remaining: 0..,
            } => {
                // collect the final amount of space available, and check if the sequence ended.
                *inner = if let Some(tail) = Self::collect_tail(iter, contd.len()) {
                    // the rest of the sequence fits, emit that without truncating.
                    tail.into_iter().pipe(|iter| Tail { iter })
                } else {
                    // the rest of the sequence wouldn't fit. emit the "continued" sequence.
                    contd.pipe(std::mem::take).pipe(Inner::limiting)
                };

                self.next() // ...and then poll again for the next item.
            }

            Limiting { iter } => next_and_mark_finished!(iter),
            Tail { iter } => next_and_mark_finished!(iter),
            Finished => None, /* we are already done. */
        }
    }
}

impl<I: Iterator + Limited> LimitedIter<I> {
    /// returns the "tail" of an [`Iterator`].
    ///
    /// if there are more than `len` items remaining in the iterator, this returns `None`.
    fn collect_tail(iter: &mut Peekable<I>, len: usize) -> Option<Vec<I::Item>> {
        let mut fits = false;
        let mut tail = Vec::with_capacity(len);

        // take the next `len` items from the provided iterator.
        for _ in 0..len {
            match iter.next() {
                Some(item) => tail.push(item),
                None => {
                    fits = true;
                    break; // we encountered the end of the sequence, so the tail fits.
                }
            }
        }

        if iter.peek().is_none() {
            fits = true; // there are no more items remaining, so the tail fits.
        }

        fits.then_some(tail)
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
    fn new(iter: I, length: usize) -> Self {
        let mut iter = iter.peekable();
        if length == 0 || iter.peek().is_none() {
            // we're already finished if our length is 0, or if given an empty iterator.
            return Self::Finished;
        }

        let contd = I::contd().collect::<Vec<_>>();
        match length.saturating_sub(contd.len()) {
            // the common case: we are emitting the contents of the inner iterator.
            remaining @ 1.. => Self::Running {
                iter,
                remaining,
                contd,
            },
            // there isn't any room for our inner iterator's contents. we are already limiting.
            0 => {
                let mut contd = contd;
                while contd.len() > length {
                    contd.pop(); // we should make sure to trim any excess from the `contd` value.
                }
                Self::limiting(contd)
            }
        }
    }

    /// returns a new [`Inner`] given a vector of items.
    fn limiting(contd: Vec<I::Item>) -> Self {
        contd
            .into_iter()
            .peekable()
            .pipe(|iter| Self::Limiting { iter })
    }
}
