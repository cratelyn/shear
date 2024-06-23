mod trim_to_length;
mod trim_to_width;

/// an extension trait for limiting strings.
///
/// # examples
///
/// strings that are longer than the given width will be truncated, with a trailing ellipses `...`
/// to indicate that additional contents have been discarded.
///
/// ```
/// use shear::str::Limited;
///
/// let s = "a very long string value";
/// let limited = s.trim_to_length(18);
///
/// assert_eq!(limited, "a very long str...");
/// assert_eq!(limited.len(), 18);
/// ```
///
/// strings that are shorter than the given width will be returned unaltered.
///
/// ```
/// use shear::str::Limited;
///
/// let s = "a shorter value";
/// let limited = s.trim_to_length(18);
///
/// assert_eq!(limited, "a shorter value");
/// assert_eq!(limited.len(), 15);
/// ```
///
/// similarly, strings that are precisely of the given width will also be returned unaltered.
///
/// ```
/// use shear::str::Limited;
///
/// let s = "cindarella slipper";
/// let limited = s.trim_to_length(18);
/// # debug_assert_eq!(s.len(), 18); // confirm our example string is exactly 18 characters.
///
/// assert_eq!(limited, "cindarella slipper");
/// assert_eq!(limited.len(), 18);
/// ```
pub trait Limited {
    /// returns a limited string.
    fn trim_to_length(&self, length: usize) -> String;

    // trim_to_length()
    // trim_to_length_utf8()
    // trim_to_width()

    // "\U{2026}"
}

// === impl s: asref<str> ===

impl<S> Limited for S
where
    S: AsRef<str>,
{
    fn trim_to_length(&self, length: usize) -> String {
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
                .pipe(TrimToLengthIter)
                .limited(length)
                .collect()
        };

        fits.then_some(value)
            .map(str::to_owned)
            .unwrap_or_else(limit)
    }
}
