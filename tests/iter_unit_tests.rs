//! a small suite of unit tests to exercise [`shear::LimitedIter<I>`].

use {
    self::test_char_iter::TestIter,
    shear::iter::Limited,
    tap::{Conv, Pipe},
};

mod test_char_iter;

#[test]
fn empty_input_will_have_empty_output() {
    "".chars()
        .conv::<TestIter>()
        .limited(5)
        .collect::<String>()
        .pipe(|s| assert_eq!(s, "", "string should still be empty"));
}

#[test]
fn longer_input_will_have_truncated_output() {
    "123456"
        .chars()
        .conv::<TestIter>()
        .limited(5)
        .collect::<String>()
        .pipe(|s| assert_eq!(s, "12...", "a longer string should be limited"));
}

#[test]
fn input_that_exactly_fits_will_not_have_truncated_output() {
    "123456"
        .chars()
        .conv::<TestIter>()
        .limited(6)
        .collect::<String>()
        .pipe(|s| assert_eq!(s, "123456", "if the string fits it should not be limited"));
}
