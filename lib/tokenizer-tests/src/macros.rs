
#[doc(hidden)]
macro_rules! expect_tokens {
    ( $factory:expr, $string:expr, $( $expected:expr ),+ ) => {
        let mut tokens = $factory($string);
        $(
            let expected_raw = &$expected[..];
            let actual_raw = tokens.next();

            match (std::str::from_utf8(expected_raw), actual_raw.map(std::str::from_utf8)) {
                (Ok(expected_str), Some(Ok(actual_str))) => assert_eq!(actual_str, expected_str),
                (Ok(expected_str), actual) => assert_eq!(actual.and_then(Result::ok), Some(expected_str)),
                (_, _) => assert_eq!(actual_raw, Some(expected_raw)),
            }
        )+

        assert_eq!(None, tokens.next());
    }
}
