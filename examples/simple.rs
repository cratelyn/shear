//! this example program demonstrates using a custom [`Ellipsis`] when trimming strings.
//!
//! this program prints a collection of strings, trimming them to fit a length (in bytes).

use {
    shear::str::{ellipsis, Limited},
    std::ops::Deref,
};

pub const WIDTH: usize = 50;

pub const FRUITS: &[&str] = &[
    "an apple is red",
    "a banana is yellow",
    "a cherry is also red",
    "a dragonfruit is black and white",
    "a lemon is yellow",
    "a lime is green",
    "a watermelon is green on the outside, but red on the inside",
];

fn main() {
    // helper fn: trim a string to `WIDTH`.
    let trim = |s: &str| s.trim_to_length::<ellipsis::Ascii>(WIDTH);

    // print each element, trimming it to a fixed length in bytes.
    FRUITS.iter().map(Deref::deref).map(trim).for_each(|fruit| {
        println!("{fruit}");
    });
}
