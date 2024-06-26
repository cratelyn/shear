pub use self::ellipsis::{Ascii, Ellipsis, Horizontal};

/// defines an [`Ellipsis`] trait.
///
/// this can be used by [`Limited`] implementations to provide a way to indicate that a string is
/// being truncated. most users will want to use [`ellipsis::Ascii`].
pub mod ellipsis;

mod trim_to_length;
mod trim_to_width;

/// a trait for limiting strings.
///
/// # examples
///
/// strings that are longer than the given width will be truncated, with a trailing ellipses `...`
/// to indicate that additional contents have been discarded.
///
/// ```
/// use shear::str::{Limited, Ascii};
///
/// let s = "a very long string value";
/// let limited = s.trim_to_length::<Ascii>(18);
///
/// assert_eq!(limited, "a very long str...");
/// assert_eq!(limited.len(), 18);
/// ```
///
/// strings that are shorter than the given width will be returned unaltered.
///
/// ```
/// use shear::str::{Limited, Ascii};
///
/// let s = "a shorter value";
/// let limited = s.trim_to_length::<Ascii>(18);
///
/// assert_eq!(limited, "a shorter value");
/// assert_eq!(limited.len(), 15);
/// ```
///
/// similarly, strings that are precisely of the given width will also be returned unaltered.
///
/// ```
/// use shear::str::{Limited, Ascii};
///
/// let s = "cindarella slipper";
/// let limited = s.trim_to_length::<Ascii>(18);
/// # debug_assert_eq!(s.len(), 18); // confirm our example string is exactly 18 characters.
///
/// assert_eq!(limited, "cindarella slipper");
/// assert_eq!(limited.len(), 18);
/// ```
pub trait Limited {
    /// returns a limited string.
    fn trim_to_length<E: Ellipsis>(&self, length: usize) -> String;
}

/// an extension trait of [`Limited<E>`].
///
/// this defines methods that supply the [`Ellipsis`] to use when limiting a string e.g.,
/// an [`ellipsis::Ascii`] ellipsis `"..."`.
pub trait LimitedExt: Limited + AsRef<str> + Sized {
    /// trim the string using an ascii `"..."` ellipsis.
    ///
    /// # examples
    ///
    /// ```
    /// use shear::str::LimitedExt;
    ///
    /// let s = "a very long string value";
    /// let limited = s.trim_to_length_ascii(18);
    ///
    /// assert_eq!(limited, "a very long str...");
    /// assert_eq!(limited.len(), 18);
    /// ```
    fn trim_to_length_ascii(&self, length: usize) -> String {
        <Self as Limited>::trim_to_length::<ellipsis::Ascii>(self, length)
    }

    /// trim the string using a horizontal `"…"` ellipsis.
    fn trim_to_length_utf8_horizontal(&self, length: usize) -> String {
        <Self as Limited>::trim_to_length::<ellipsis::Horizontal>(self, length)
    }
}
impl<S> LimitedExt for S where S: Limited + AsRef<str> {}

// === impl s: asref<str> ===

impl<S> Limited for S
where
    S: AsRef<str>,
{
    fn trim_to_length<E: Ellipsis>(&self, length: usize) -> String {
        use self::trim_to_length::TrimToLengthIter;

        let value: &'_ str = self.as_ref();

        // we know the length of a string in advance, so we can check if the value fits into the
        // given length, without having to iterate over its characters.
        let fits = value.len() <= length;

        // helper fn: if called, limits the contents of the string.
        let limit = || {
            use {crate::iter::Limited, tap::Pipe};
            value
                .chars()
                .pipe(TrimToLengthIter::<_, E>::new)
                .limited(length)
                .collect()
        };

        fits.then_some(value)
            .map(str::to_owned)
            .unwrap_or_else(limit)
    }
}
