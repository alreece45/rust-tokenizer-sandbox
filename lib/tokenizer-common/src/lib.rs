
#[doc(hidden)]
#[macro_export]
macro_rules! test_tokenizer {
    ($create_tokenizer:expr) => {
        #[test]
        fn whitespace() {
            $crate::tests::whitespace($create_tokenizer);
        }

        #[test]
        fn parentheses() {
            $crate::tests::parentheses($create_tokenizer);
        }

        #[test]
        fn math_single_character() {
            $crate::tests::math_single($create_tokenizer);
        }

        #[test]
        fn math_double_character() {
            $crate::tests::math_double_char($create_tokenizer);
        }

        #[test]
        fn logical_ops() {
            $crate::tests::logical_ops($create_tokenizer);
        }

        #[test]
        fn other() {
            $crate::tests::unused($create_tokenizer);
        }

        #[test]
        fn strings() {
            $crate::tests::strings($create_tokenizer);
        }
    };
}

#[inline]
pub fn is_whitespace(c: u8) -> bool {
    match c {
        b' ' | b'\t' | b'\n' | b'\r' => true,
        _ => false,
    }
}

#[inline]
pub fn is_not_whitespace(c: u8) -> bool {
    !is_whitespace(c)
}

#[inline]
pub fn is_start_of_two(c: u8) -> bool {
    match c {
        b'=' | b'!' | b'&' | b'|' | b'*' | b'b' => true,
        _ => false,
    }
}

#[inline]
pub fn is_start_prefix(c: u8) -> bool {
    match c {
        b'.' | b'=' | b'!' | b'&' | b'|' |
        b'(' | b')' | b'[' | b']' | b',' |
        b'*' | b'-' | b'+' | b'/' | b'^' => true,
        _ => false,
    }
}

#[inline]
pub fn is_single_character_token(c: u8) -> bool {
    match c {
        b'(' | b')' | b'[' | b']' | b',' |
        b'.' | b'|' | b'&' | b'-' | b'+' |
        b'/' | b'^' => true,
        _ => false,
    }
}

#[macro_use]
pub mod tests {
    macro_rules! expect_tokens {
        ( $factory:expr, $string:expr, $( $expected:expr ),+ ) => {
            let mut tokens = $factory($string);
            $(
                let expected_raw = &$expected[..];
                let actual_raw = tokens.next();

                if let Ok(expected_str) = std::str::from_utf8(expected_raw) {
                    match actual_raw.map(std::str::from_utf8) {
                        Some(Ok(actual_str)) => assert_eq!(actual_str, expected_str),
                        None => assert_eq!(None, Some(expected_str)),
                        _ => assert_eq!(actual_raw, Some(expected_raw)),
                    }
                } else {
                    assert_eq!(actual_raw, Some(expected_raw));
                }
            )+

            assert_eq!(None, tokens.next());
        }
    }

    pub fn whitespace<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
    {
        expect_tokens!(
            tokenizer_factory,
            b" \r\n\tthis\t\n\r is \r\n\t a \t\n\r test",

            b"this", b"is", b"a", b"test"
        );
    }

    pub fn parentheses<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
    {
        expect_tokens!(
            tokenizer_factory,
            b" (!test1000(test1001 test1002)test1003 () (( ))",
            b"(", b"!", b"test1000",
            b"(", b"test1001", b"test1002", b")",
            b"test1003",
            b"(", b")",
            b"(", b"(", b")", b")"
        );
    }

    pub fn math_single<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
    {
        expect_tokens!(
            tokenizer_factory,
            b"test2001 + test2002 - test2003 * test2004 / test2005 ++--*//",

            b"test2001", b"+", b"test2002", b"-",
            b"test2003", b"*", b"test2004", b"/",
            b"test2005", b"+", b"+", b"-", b"-", b"*", b"/", b"/"
        );
    }

    pub fn math_double_char<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
    {
        expect_tokens!(
            tokenizer_factory,
            b"test2100 ** test2101 ** test2102 ******",

            b"test2100", b"**", b"test2101", b"**", b"test2102",
            b"**", b"**", b"**"
        );
    }

    pub fn logical_ops<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
    {

        expect_tokens!(
            tokenizer_factory,
            b"test3001 && test3002 == test3003 || test3004 != test3005 !=||==&&",

            b"test3001", b"&&", b"test3002", b"==",
            b"test3003", b"||", b"test3004", b"!=",
            b"test3005", b"!=", b"||", b"==", b"&&"
        );
    }

    pub fn unused<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
    {
        expect_tokens!(
            tokenizer_factory,
            b"  [test9, test10, test11] , . - +",

            b"[", b"test9", b",", b"test10", b",", b"test11", b"]",
            b",", b".", b"-", b"+"
        );
    }

    pub fn strings<'a, F, I>(tokenizer_factory: F)
    where
        F: Fn(&'a [u8]) -> I,
        I: Iterator<Item = &'a [u8]>,
    {
        expect_tokens!(
            tokenizer_factory, br#"
            "test 4001" + b"test 4002" + "test4003 ()\"*+-\\ test4004"
            + b"test4005 +-\" + test4006" + beginning_with_a_b"#,

            br#""test 4001""#, b"+", br#"b"test 4002""#, b"+",
            br#""test4003 ()\"*+-\\ test4004""#, b"+",
            br#"b"test4005 +-\" + test4006""#, b"+",
            b"beginning_with_a_b"
        );
    }
}
