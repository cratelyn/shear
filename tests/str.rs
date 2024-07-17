//! test cases for string-specific facilities in [`shear::str`].

#![cfg(feature = "str")]

use {
    self::strategy::TestInput, proptest::proptest, regex::Regex, shear::str::LimitedExt, tap::Pipe,
};

mod strategy;

/// this does not contains *tests*, but confirms that the public api is ergonomic.
///
/// confirm that we can limit smart pointers like e.g. [`std::borrow::Cow`].
/// compile the project using `--tests` or `all-targets` to exercise these definitions.
mod relevant_types_can_be_limited {
    use std::borrow::Cow;

    use super::*;

    #[allow(unreachable_code, unused_variables, clippy::diverging_sub_expression)]
    fn can_be_limited<T: shear::str::LimitedExt>() {
        let value: T = unimplemented!("");
        value.trim_to_length_ascii(0).pipe(drop);
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
        "".trim_to_length_ascii(len)
            .pipe(|s| assert_eq!(s, "", "limited string should still be empty"))
    }
}

mod strings_with_various_utf8_character_lengths_can_be_limited {
    use {
        shear::str::{ellipsis::Ascii, Limited},
        tap::Pipe,
    };

    /// an input string for use in tests below.
    const HELLO: &str = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";

    #[test]
    fn twenty_five_columns_wide() {
        HELLO
            .trim_to_width::<Ascii>(25)
            //                      "1234567890123456789012345"
            .pipe(|s| assert_eq!(s, "Ｈｅｌｌｏ, ｗｏｒｌｄ!"))
    }

    #[test]
    fn twenty_four_columns_wide() {
        HELLO
            .trim_to_width::<Ascii>(24)
            //                      "123456789012345678901234"
            .pipe(|s| assert_eq!(s, "Ｈｅｌｌｏ, ｗｏｒｌｄ!"))
    }

    #[test]
    fn twenty_three_columns_wide() {
        HELLO
            .trim_to_width::<Ascii>(23)
            //                      "12345678901234567890123"
            .pipe(|s| assert_eq!(s, "Ｈｅｌｌｏ, ｗｏｒｌｄ!"))
    }

    #[test]
    fn twenty_two_columns_wide() {
        HELLO
            .trim_to_width::<Ascii>(22)
            //                      "1234567890123456789012"
            .pipe(|s| assert_eq!(s, "Ｈｅｌｌｏ, ｗｏｒ..."))
    }

    #[test]
    fn thirtheen_columns_wide() {
        HELLO
            .trim_to_width::<Ascii>(13)
            //                      "1234567890123"
            .pipe(|s| assert_eq!(s, "Ｈｅｌｌｏ..."))
    }

    #[test]
    fn twelve_columns_wide() {
        HELLO
            .trim_to_width::<Ascii>(12)
            //                      "12345678901"
            .pipe(|s| assert_eq!(s, "Ｈｅｌｌ..."))
    }

    #[test]
    fn eleven_columns_wide() {
        HELLO
            .trim_to_width::<Ascii>(11)
            //                      "12345678901"
            .pipe(|s| assert_eq!(s, "Ｈｅｌｌ..."))
    }

    #[test]
    fn five_columns_wide() {
        "Ｈｅｌｌｏ, ｗｏｒｌｄ!"
            .trim_to_width::<Ascii>(5)
            //                      "12345"
            .pipe(|s| assert_eq!(s, "Ｈ..."))
    }

    #[test]
    fn four_columns_wide() {
        "Ｈｅｌｌｏ, ｗｏｒｌｄ!"
            .trim_to_width::<Ascii>(4)
            //                      "123"
            .pipe(|s| assert_eq!(s, "..."))
    }

    #[test]
    fn three_columns_wide() {
        "Ｈｅｌｌｏ, ｗｏｒｌｄ!"
            .trim_to_width::<Ascii>(3)
            //                      "123"
            .pipe(|s| assert_eq!(s, "..."))
    }
}

/// test that strings can be truncated correctly.
mod strs_can_be_truncated {
    use {super::*, tap::Tap};

    proptest! {
        #[test]
        fn strs_can_be_truncated (input in strategy::values_that_need_truncation())
        {
            strs_can_be_truncated_(input)
        }
    }

    fn strs_can_be_truncated_(TestInput { value, length }: TestInput) {
        let limited = value.clone().trim_to_length_ascii(length);
        let expected: Regex = {
            let prefix = {
                let mut s = String::new();
                let mut remaining = length - 3;
                for c in value.chars() {
                    let len = c.len_utf8();
                    if remaining >= len {
                        s.push(c);
                        remaining -= len;
                        continue;
                    } else {
                        break;
                    }
                }
                s
            };
            let suffix = "...".chars();
            let extend = |s: &mut String| s.extend(suffix);
            prefix.tap_mut(extend).as_str().pipe(Regex::new).unwrap()
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
