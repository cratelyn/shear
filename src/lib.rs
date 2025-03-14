#![doc = include_str!("../README.md")]
#![deny(
    // rustc lints:
    deprecated,
    future_incompatible,
    keyword_idents,
    let_underscore,
    missing_docs,
    nonstandard_style,
    unused,
    // clippy lints:
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::suspicious,
    // rustdoc lints:
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::private_doc_tests,
    rustdoc::private_intra_doc_links,
    rustdoc::redundant_explicit_links,
    rustdoc::unescaped_backticks,
)]

/// [`Iterator`] limiting.
///
/// see [`Limited`][self::iter::Limited] for more information.
pub mod iter;

/// [`String`] limiting.
///
/// see [`Limited`][self::str::Limited] for more information.
#[cfg(feature = "str")]
pub mod str;
