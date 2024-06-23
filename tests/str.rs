//! test cases for string-specific facilities in [`shear::str`].

#![cfg(feature = "str")]

use {self::strategy::TestInput, proptest::proptest, regex::Regex, shear::str::Limited, tap::Pipe};

mod strategy;

/// this does not contains *tests*, but confirms that the public api is ergonomic.
///
/// confirm that we can limit smart pointers like e.g. [`std::borrow::Cow`].
/// compile the project using `--tests` or `all-targets` to exercise these definitions.
mod relevant_types_can_be_limited {
    use std::borrow::Cow;

    use super::*;

    #[allow(unreachable_code, unused_variables, clippy::diverging_sub_expression)]
    fn can_be_limited<T: shear::str::Limited>() {
        let value: T = unimplemented!("");
        value.limited(0).pipe(drop);
    }

    #[allow(dead_code)] // this is a compile-time check that needn't be called.
    fn compile_checks() {
        can_be_limited::<String>();
        can_be_limited::<Cow<str>>();
        can_be_limited::<Box<str>>();
    }
}

/// test that empty strings are still empty when [`Limited`].
mod empty_str_can_be_limited {
    use super::*;

    proptest! {
        #[test]
        fn empty_str_can_be_limited(len in 0..2048_usize)
        {
            empty_str_can_be_limited_(len)
        }
    }

    fn empty_str_can_be_limited_(len: usize) {
        "".limited(len)
            .pipe(|s| assert_eq!(s, "", "limited string should still be empty"))
    }
}

/// test that strings can be truncated correctly.
mod strs_can_be_truncated {
    use tap::Tap;

    use super::*;

    proptest! {
        #[test]
        fn strs_can_be_truncated (input in strategy::values_that_need_truncation())
        {
            strs_can_be_truncated_(input)
        }
    }

    fn strs_can_be_truncated_(TestInput { value, length }: TestInput) {
        let limited = value.clone().limited(length);
        let expected: Regex = {
            let prefix = (length - 3)
                .pipe(|upper| 0..upper)
                .pipe(|range| &value[range]);
            let suffix = "...".chars();
            let extend = |s: &mut String| s.extend(suffix);
            prefix
                .pipe(str::to_owned)
                .tap_mut(extend)
                .as_str()
                .pipe(Regex::new)
                .unwrap()
        };

        expected.is_match(&limited).pipe(|is| {
            assert!(
                is,
                "value should match expected output regex \
                 \n\tvalue:    `{value}`                  \
                 \n\tlimited:  `{limited}`                \
                 \n\texpected: `{expected}`"
            )
        });
    }
}
