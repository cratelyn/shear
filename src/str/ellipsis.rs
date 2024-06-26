/// An ellipsis.
///
/// This can be implemented by a struct to provide an ellipsis, for use in trimming strings.
pub trait Ellipsis {
    /// return the ellipsis as a static string.
    fn ellipsis() -> &'static str;
}

/// an asci ellipsis.
pub struct Ascii;

/// a horizontal utf-8 ellipsis.
pub struct Horizontal;

// === impl ascii ===

impl Ellipsis for Ascii {
    fn ellipsis() -> &'static str {
        "..."
    }
}

// === impl horizontal ===

impl Ellipsis for Horizontal {
    fn ellipsis() -> &'static str {
        "…"
    }
}
