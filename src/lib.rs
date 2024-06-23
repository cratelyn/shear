//! a library for trimming excess contents from things.

/// [`Iterator`] limiting.
///
/// see [`Limited::limited()`][self::iter::Limited::limited] for more information.
pub mod iter;

/// [`String`] limiting.
///
/// see [`Limited::limited()`][self::str::Limited::limited] for more information.
#[cfg(feature = "str")]
pub mod str;
