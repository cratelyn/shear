//! this is a suite of property tests exercising [`LimitedIter<I>`].

use {
    self::{strategy::*, test_char_iter::TestIter},
    proptest::proptest,
    shear::iter::Limited,
    std::ops::Not,
    tap::{Conv, Pipe},
};

mod strategy;
mod test_char_iter;

// === test is_finished() ===

proptest! {
    /// a property test showing that the iterator is placed in the finished state properly.
    #[test]
    fn iterator_knows_when_it_is_finished(input in input_strategy()) {
        iterator_knows_when_it_is_finished_(input)
    }
}

fn iterator_knows_when_it_is_finished_(TestInput { value, length }: TestInput) {
    let mut iter = value.chars().conv::<TestIter>().limited(length);
    let length = std::cmp::min(length, value.chars().count());

    for _ in 0..length {
        iter.next().pipe(drop);
        iter.is_finished()
            .not()
            .pipe(|not| assert!(not, "iterator should not be finished"));
    }

    iter.next()
        .is_none()
        .pipe(|empty| debug_assert!(empty, "the iterator should yield a `None` at this point."));

    iter.is_finished().pipe(|is| {
        assert!(
            is,
            "iterator should be finished \
                     \n\tvalue: {value}  \
                     \n\tlength: {length}"
        )
    });
}

// === test shorter values ===

proptest! {
    /// a property test showing sequences shorter than the length pass through unaltered.
    #[test]
    fn shorter_values_are_provided_as_is(input in values_that_fit()) {
        shorter_values_are_provided_as_is_(input)
    }
}

fn shorter_values_are_provided_as_is_(
    TestInput {
        value: input,
        length,
    }: TestInput,
) {
    let output = input
        .chars()
        .conv::<TestIter>()
        .limited(length)
        .collect::<String>();
    assert_eq!(
        input, output,
        "a sequence that fits into its length should be returned unaltered"
    );
}

// === test truncation ===

proptest! {
    /// a property test showing long values are truncated.
    #[test]
    fn longer_values_are_truncated(input in values_that_need_truncation()) {
        longer_values_are_truncated_(input)
    }
}

fn longer_values_are_truncated_(TestInput { value, length }: TestInput) {
    use regex::Regex;
    let actual = value
        .chars()
        .conv::<TestIter>()
        .limited(length)
        .collect::<String>();
    let expected: Regex = {
        let contd = "...";
        let n = length - contd.len();
        let prefix = value.chars().conv::<TestIter>().take(n).collect::<String>();
        format!("{prefix}\\.\\.\\.")
            .as_str()
            .pipe(Regex::new)
            .expect("should be a valid regex")
    };

    expected.is_match(&actual).pipe(|is| {
        assert!(
            is,
            "value should match expected output regex \
                 \n\tvalue:    `{value}`              \
                 \n\tlimited:  `{actual}`             \
                 \n\texpected: `{expected}`"
        )
    });
}

proptest! {
    /// a property test showing small lengths like 1, 2, or 3 only emit limited `...` output.
    #[test]
    fn a_size_equal_to_or_smaller_than_contd_procedes_directly_to_limiting(
        value in value_strategy_non_empty(),
        length in 1..=3_usize,
    ) {
        a_size_equal_to_or_smaller_than_contd_procedes_directly_to_limiting_(value, length)
    }
}

fn a_size_equal_to_or_smaller_than_contd_procedes_directly_to_limiting_(
    value: String,
    length: usize,
) {
    let actual = value
        .chars()
        .conv::<TestIter>()
        .limited(length)
        .collect::<String>();
    let expected: regex::Regex = {
        "\\."
            .repeat(length)
            .as_str()
            .pipe(regex::Regex::new)
            .expect("should be a valid regex")
    };

    expected.is_match(&actual).pipe(|is| {
        assert!(
            is,
            "value should match expected output regex \
                 \n\tvalue:    `{value}`              \
                 \n\tlimited:  `{actual}`             \
                 \n\texpected: `{expected}`"
        )
    });
}
